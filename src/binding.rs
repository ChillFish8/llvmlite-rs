use std::error::Error;
use std::ffi::{c_char, CStr, CString, OsStr};
use std::fmt::{Debug, Display, Formatter};
use std::mem;

use libloading::{Library, Symbol};

use llvmlite_types::*;
use crate::wrappers::{ByteString, Context, Utf8String};

pub enum LoadError {
    LoadFunction {
        inner: libloading::Error,
        target: String,
    },
    LoadLLVM {
        inner: libloading::Error,
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::LoadFunction { target, inner } => write!(f, "LoadError(function={target:?}, error={inner}, help=\"Are you using a llvmlite.dll from 0.40.0+?\")"),
            LoadError::LoadLLVM { inner } => write!(f, "LoadError(load_llvm={inner})"),
        }
    }
}

impl Debug for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Error for LoadError {}

pub struct LLVMLite {
    // NOTE: This is not actually `'static` it's bound to the lifetime of `Self`
    methods: LLVMMethods<'static>,
    _library: Library,
}

impl LLVMLite {
    /// Attempts to load the LLVMLite library DLL/SO.
    ///
    /// This will automatically add the `.dll` and `.so` depending on the platform.
    ///
    /// # Safety
    /// When a library is loaded, initialisation routines contained within it are executed.
    /// For the purposes of safety, the execution of these routines is conceptually the same
    /// calling an unknown foreign function and may impose arbitrary requirements on the caller
    /// for the call to be sound.
    ///
    /// Additionally, the callers of this function must also ensure that execution of the
    /// termination routines contained within the library is safe as well.
    /// These routines may be executed when the library is unloaded.
    pub unsafe fn new(path: impl AsRef<OsStr>) -> Result<Self, LoadError> {
        let path = libloading::library_filename(path.as_ref());
        let library = Library::new(path)
            .map_err(|e| LoadError::LoadLLVM { inner: e })?;
        let methods = LLVMMethods::load(&library)?;
        let methods = mem::transmute::<LLVMMethods<'_>, LLVMMethods<'static>>(methods);

        Ok(Self {
            methods,
            _library: library,
        })
    }

    /// Creates a new string from a given input.
    ///
    /// This string is re-allocated by LLVM which can then be passed back
    /// and forth if needed.
    pub fn create_string(&self, s: impl Into<String>) -> Utf8String {
        let c_str = CString::new(s.into())
            .expect("CString should not contain null byte");
        let ptr = c_str.as_ptr();

        // We re-alloc the string
        let ptr: *const c_char = unsafe { (self.methods.create_string)(ptr) };

        if ptr.is_null() {
            return Utf8String::empty(self.methods.clone())
        }

        let c_str = unsafe { CStr::from_ptr(ptr) };
        let str_inner = c_str.to_str()
            .expect("Re-alloc UTF-8 string has produced non-UTF-8 result");
        let inner = unsafe { mem::transmute::<&str, &'static str>(str_inner) };

        Utf8String {
            inner,
            ptr,
            lib: self.methods.clone(),
        }
    }

    /// Creates a new non-UTF-8 encoded byte string.
    ///
    /// This string is re-allocated by LLVM which can then be passed back
    /// and forth if needed.
    pub fn create_byte_string(&self, s: impl Into<CString>) -> ByteString {
        let c_str = s.into();
        let len = c_str.as_bytes().len();
        let ptr = c_str.as_ptr();

        // We re-alloc the string
        let ptr: *const c_char = unsafe { (self.methods.create_byte_string)(ptr, len) };

        if ptr.is_null() {
            return ByteString::empty(self.methods.clone())
        }

        let inner = unsafe { CStr::from_ptr(ptr) };

        ByteString {
            inner: Some(inner),
            ptr,
            lib: self.methods.clone(),
        }
    }

    /// Gets the global LLVM context.
    pub fn get_global_context(&self) -> Context {
        let ctx_ptr = unsafe { (self.methods.get_global_context)() };
        Context {
            is_global: true,
            inner: ctx_ptr,
            lib: self.methods.clone(),
        }
    }

    /// Creates a new LLVM context.
    pub fn create_context(&self) -> Context {
        let ctx_ptr = unsafe { (self.methods.context_create)() };
        Context {
            is_global: false,
            inner: ctx_ptr,
            lib: self.methods.clone(),
        }
    }
}


#[derive(Clone)]
pub struct LLVMMethods<'lib> {
    pub(crate) set_cmd_line: Symbol<'lib, LLVMPY_SetCommandLine>,
    pub(crate) parse_assembly: Symbol<'lib, LLVMPY_ParseAssembly>,
    pub(crate) write_bitcode_to_string: Symbol<'lib, LLVMPY_WriteBitcodeToString>,
    pub(crate) parse_bitcode: Symbol<'lib, LLVMPY_ParseBitcode>,
    pub(crate) create_string: Symbol<'lib, LLVMPY_CreateString>,
    pub(crate) create_byte_string: Symbol<'lib, LLVMPY_CreateByteString>,
    pub(crate) dispose_string: Symbol<'lib, LLVMPY_DisposeString>,
    pub(crate) get_global_context: Symbol<'lib, LLVMPY_GetGlobalContext>,
    pub(crate) context_create: Symbol<'lib, LLVMPY_ContextCreate>,
    pub(crate) context_dispose: Symbol<'lib, LLVMPY_ContextDispose>,
    pub(crate) add_ref_prune_pass: Symbol<'lib, LLVMPY_AddRefPrunePass>,
    pub(crate) dump_ref_prune_stats: Symbol<'lib, LLVMPY_DumpRefPruneStats>,
    pub(crate) search_address_of_symbol: Symbol<'lib, LLVMPY_SearchAddressOfSymbol>,
    pub(crate) add_symbol: Symbol<'lib, LLVMPY_AddSymbol>,
    pub(crate) load_library_permanently: Symbol<'lib, LLVMPY_LoadLibraryPermanently>,
    pub(crate) link_in_mc_jit: Symbol<'lib, LLVMPY_LinkInMCJIT>,
    pub(crate) dispose_execution_engine: Symbol<'lib, LLVMPY_DisposeExecutionEngine>,
    pub(crate) add_module: Symbol<'lib, LLVMPY_AddModule>,
    pub(crate) remove_module: Symbol<'lib, LLVMPY_RemoveModule>,
    pub(crate) finalize_object: Symbol<'lib, LLVMPY_FinalizeObject>,
    pub(crate) create_mc_jit_compiler: Symbol<'lib, LLVMPY_CreateMCJITCompiler>,
    pub(crate) get_global_value_address: Symbol<'lib, LLVMPY_GetGlobalValueAddress>,
    pub(crate) get_function_address: Symbol<'lib, LLVMPY_GetFunctionAddress>,
    pub(crate) run_static_constructors: Symbol<'lib, LLVMPY_RunStaticConstructors>,
    pub(crate) run_static_destructors: Symbol<'lib, LLVMPY_RunStaticDestructors>,
    pub(crate) add_global_mapping: Symbol<'lib, LLVMPY_AddGlobalMapping>,
    pub(crate) get_execution_engine_target_data: Symbol<'lib, LLVMPY_GetExecutionEngineTargetData>,
    pub(crate) try_allocate_executable_memory: Symbol<'lib, LLVMPY_TryAllocateExecutableMemory>,
    pub(crate) enable_jit_events: Symbol<'lib, LLVMPY_EnableJITEvents>,
    pub(crate) mc_jit_add_object_file: Symbol<'lib, LLVMPY_MCJITAddObjectFile>,
    pub(crate) create_object_cache: Symbol<'lib, LLVMPY_CreateObjectCache>,
    pub(crate) dispose_object_cache: Symbol<'lib, LLVMPY_DisposeObjectCache>,
    pub(crate) set_object_cache: Symbol<'lib, LLVMPY_SetObjectCache>,
    pub(crate) shutdown: Symbol<'lib, LLVMPY_Shutdown>,
    pub(crate) get_version_info: Symbol<'lib, LLVMPY_GetVersionInfo>,
    pub(crate) link_modules: Symbol<'lib, LLVMPY_LinkModules>,
    pub(crate) dispose_module: Symbol<'lib, LLVMPY_DisposeModule>,
    pub(crate) print_module_to_string: Symbol<'lib, LLVMPY_PrintModuleToString>,
    pub(crate) get_module_source_file_name: Symbol<'lib, LLVMPY_GetModuleSourceFileName>,
    pub(crate) get_module_name: Symbol<'lib, LLVMPY_GetModuleName>,
    pub(crate) set_module_name: Symbol<'lib, LLVMPY_SetModuleName>,
    pub(crate) get_named_function: Symbol<'lib, LLVMPY_GetNamedFunction>,
    pub(crate) get_named_global_variable: Symbol<'lib, LLVMPY_GetNamedGlobalVariable>,
    pub(crate) get_named_struct_type: Symbol<'lib, LLVMPY_GetNamedStructType>,
    pub(crate) verify_module: Symbol<'lib, LLVMPY_VerifyModule>,
    pub(crate) get_data_layout: Symbol<'lib, LLVMPY_GetDataLayout>,
    pub(crate) set_data_layout: Symbol<'lib, LLVMPY_SetDataLayout>,
    pub(crate) get_target: Symbol<'lib, LLVMPY_GetTarget>,
    pub(crate) set_target: Symbol<'lib, LLVMPY_SetTarget>,
    pub(crate) module_globals_iter: Symbol<'lib, LLVMPY_ModuleGlobalsIter>,
    pub(crate) module_functions_iter: Symbol<'lib, LLVMPY_ModuleFunctionsIter>,
    pub(crate) module_types_iter: Symbol<'lib, LLVMPY_ModuleTypesIter>,
    pub(crate) globals_iter_next: Symbol<'lib, LLVMPY_GlobalsIterNext>,
    pub(crate) functions_iter_next: Symbol<'lib, LLVMPY_FunctionsIterNext>,
    pub(crate) types_iter_next: Symbol<'lib, LLVMPY_TypesIterNext>,
    pub(crate) dispose_globals_iter: Symbol<'lib, LLVMPY_DisposeGlobalsIter>,
    pub(crate) dispose_functions_iter: Symbol<'lib, LLVMPY_DisposeFunctionsIter>,
    pub(crate) dispose_types_iter: Symbol<'lib, LLVMPY_DisposeTypesIter>,
    pub(crate) clone_module: Symbol<'lib, LLVMPY_CloneModule>,
    pub(crate) create_object_file: Symbol<'lib, LLVMPY_CreateObjectFile>,
    pub(crate) dispose_object_file: Symbol<'lib, LLVMPY_DisposeObjectFile>,
    pub(crate) get_sections: Symbol<'lib, LLVMPY_GetSections>,
    pub(crate) dispose_section_iterator: Symbol<'lib, LLVMPY_DisposeSectionIterator>,
    pub(crate) move_to_next_section: Symbol<'lib, LLVMPY_MoveToNextSection>,
    pub(crate) is_section_iterator_at_end: Symbol<'lib, LLVMPY_IsSectionIteratorAtEnd>,
    pub(crate) get_section_name: Symbol<'lib, LLVMPY_GetSectionName>,
    pub(crate) get_section_address: Symbol<'lib, LLVMPY_GetSectionAddress>,
    pub(crate) get_section_contents: Symbol<'lib, LLVMPY_GetSectionContents>,
    pub(crate) get_section_size: Symbol<'lib, LLVMPY_GetSectionSize>,
    pub(crate) is_section_text: Symbol<'lib, LLVMPY_IsSectionText>,
    pub(crate) set_time_passes: Symbol<'lib, LLVMPY_SetTimePasses>,
    pub(crate) report_and_reset_timings: Symbol<'lib, LLVMPY_ReportAndResetTimings>,
    pub(crate) create_pass_manager: Symbol<'lib, LLVMPY_CreatePassManager>,
    pub(crate) dispose_pass_manager: Symbol<'lib, LLVMPY_DisposePassManager>,
    pub(crate) create_function_pass_manager: Symbol<'lib, LLVMPY_CreateFunctionPassManager>,
    pub(crate) run_pass_manager_with_remarks: Symbol<'lib, LLVMPY_RunPassManagerWithRemarks>,
    pub(crate) run_pass_manager: Symbol<'lib, LLVMPY_RunPassManager>,
    pub(crate) run_function_pass_manager_with_remarks: Symbol<'lib, LLVMPY_RunFunctionPassManagerWithRemarks>,
    pub(crate) run_function_pass_manager: Symbol<'lib, LLVMPY_RunFunctionPassManager>,
    pub(crate) initialize_function_pass_manager: Symbol<'lib, LLVMPY_InitializeFunctionPassManager>,
    pub(crate) finalize_function_pass_manager: Symbol<'lib, LLVMPY_FinalizeFunctionPassManager>,
    pub(crate) add_a_a_eval_pass: Symbol<'lib, LLVMPY_AddAAEvalPass>,
    pub(crate) add_basic_a_a_wrapper_pass: Symbol<'lib, LLVMPY_AddBasicAAWrapperPass>,
    pub(crate) add_dependence_analysis_pass: Symbol<'lib, LLVMPY_AddDependenceAnalysisPass>,
    pub(crate) add_call_graph_dot_printer_pass: Symbol<'lib, LLVMPY_AddCallGraphDOTPrinterPass>,
    pub(crate) add_dot_dom_printer_pass: Symbol<'lib, LLVMPY_AddDotDomPrinterPass>,
    pub(crate) add_globals_mod_ref_a_a_pass: Symbol<'lib, LLVMPY_AddGlobalsModRefAAPass>,
    pub(crate) add_dot_post_dom_printer_pass: Symbol<'lib, LLVMPY_AddDotPostDomPrinterPass>,
    pub(crate) add_cfg_printer_pass: Symbol<'lib, LLVMPY_AddCFGPrinterPass>,
    pub(crate) add_constant_merge_pass: Symbol<'lib, LLVMPY_AddConstantMergePass>,
    pub(crate) add_dead_store_elimination_pass: Symbol<'lib, LLVMPY_AddDeadStoreEliminationPass>,
    pub(crate) add_reverse_post_order_function_attrs_pass: Symbol<'lib, LLVMPY_AddReversePostOrderFunctionAttrsPass>,
    pub(crate) add_dead_arg_elimination_pass: Symbol<'lib, LLVMPY_AddDeadArgEliminationPass>,
    pub(crate) add_instruction_count_pass: Symbol<'lib, LLVMPY_AddInstructionCountPass>,
    pub(crate) add_iv_users_pass: Symbol<'lib, LLVMPY_AddIVUsersPass>,
    pub(crate) add_lazy_value_info_pass: Symbol<'lib, LLVMPY_AddLazyValueInfoPass>,
    pub(crate) add_lint_pass: Symbol<'lib, LLVMPY_AddLintPass>,
    pub(crate) add_module_debug_info_printer_pass: Symbol<'lib, LLVMPY_AddModuleDebugInfoPrinterPass>,
    pub(crate) add_region_info_pass: Symbol<'lib, LLVMPY_AddRegionInfoPass>,
    pub(crate) add_scalar_evolution_a_a_pass: Symbol<'lib, LLVMPY_AddScalarEvolutionAAPass>,
    pub(crate) add_aggressive_d_c_e_pass: Symbol<'lib, LLVMPY_AddAggressiveDCEPass>,
    pub(crate) add_always_inliner_pass: Symbol<'lib, LLVMPY_AddAlwaysInlinerPass>,
    pub(crate) add_arg_promotion_pass: Symbol<'lib, LLVMPY_AddArgPromotionPass>,
    pub(crate) add_break_critical_edges_pass: Symbol<'lib, LLVMPY_AddBreakCriticalEdgesPass>,
    pub(crate) add_function_attrs_pass: Symbol<'lib, LLVMPY_AddFunctionAttrsPass>,
    pub(crate) add_function_inlining_pass: Symbol<'lib, LLVMPY_AddFunctionInliningPass>,
    pub(crate) add_global_optimizer_pass: Symbol<'lib, LLVMPY_AddGlobalOptimizerPass>,
    pub(crate) add_global_dce_pass: Symbol<'lib, LLVMPY_AddGlobalDCEPass>,
    pub(crate) add_ipsccp_pass: Symbol<'lib, LLVMPY_AddIPSCCPPass>,
    pub(crate) add_dead_code_elimination_pass: Symbol<'lib, LLVMPY_AddDeadCodeEliminationPass>,
    pub(crate) add_aggressive_instruction_combining_pass: Symbol<'lib, LLVMPY_AddAggressiveInstructionCombiningPass>,
    pub(crate) add_internalize_pass: Symbol<'lib, LLVMPY_AddInternalizePass>,
    pub(crate) add_jump_threading_pass: Symbol<'lib, LLVMPY_AddJumpThreadingPass>,
    pub(crate) add_lcssa_pass: Symbol<'lib, LLVMPY_AddLCSSAPass>,
    pub(crate) add_loop_deletion_pass: Symbol<'lib, LLVMPY_AddLoopDeletionPass>,
    pub(crate) add_single_loop_extractor_pass: Symbol<'lib, LLVMPY_AddSingleLoopExtractorPass>,
    pub(crate) add_loop_strength_reduce_pass: Symbol<'lib, LLVMPY_AddLoopStrengthReducePass>,
    pub(crate) add_loop_simplification_pass: Symbol<'lib, LLVMPY_AddLoopSimplificationPass>,
    pub(crate) add_loop_unroll_pass: Symbol<'lib, LLVMPY_AddLoopUnrollPass>,
    pub(crate) add_loop_unroll_and_jam_pass: Symbol<'lib, LLVMPY_AddLoopUnrollAndJamPass>,
    pub(crate) add_loop_unswitch_pass: Symbol<'lib, LLVMPY_AddLoopUnswitchPass>,
    pub(crate) add_lower_atomic_pass: Symbol<'lib, LLVMPY_AddLowerAtomicPass>,
    pub(crate) add_lower_invoke_pass: Symbol<'lib, LLVMPY_AddLowerInvokePass>,
    pub(crate) add_lower_switch_pass: Symbol<'lib, LLVMPY_AddLowerSwitchPass>,
    pub(crate) add_mem_cpy_optimization_pass: Symbol<'lib, LLVMPY_AddMemCpyOptimizationPass>,
    pub(crate) add_merge_functions_pass: Symbol<'lib, LLVMPY_AddMergeFunctionsPass>,
    pub(crate) add_merge_returns_pass: Symbol<'lib, LLVMPY_AddMergeReturnsPass>,
    pub(crate) add_partial_inlining_pass: Symbol<'lib, LLVMPY_AddPartialInliningPass>,
    pub(crate) add_prune_exception_handling_pass: Symbol<'lib, LLVMPY_AddPruneExceptionHandlingPass>,
    pub(crate) add_re_associate_pass: Symbol<'lib, LLVMPY_AddReassociatePass>,
    pub(crate) add_demote_register_to_memory_pass: Symbol<'lib, LLVMPY_AddDemoteRegisterToMemoryPass>,
    pub(crate) add_sink_pass: Symbol<'lib, LLVMPY_AddSinkPass>,
    pub(crate) add_strip_symbols_pass: Symbol<'lib, LLVMPY_AddStripSymbolsPass>,
    pub(crate) add_strip_dead_debug_info_pass: Symbol<'lib, LLVMPY_AddStripDeadDebugInfoPass>,
    pub(crate) add_strip_dead_prototypes_pass: Symbol<'lib, LLVMPY_AddStripDeadPrototypesPass>,
    pub(crate) add_strip_debug_declare_prototypes_pass: Symbol<'lib, LLVMPY_AddStripDebugDeclarePrototypesPass>,
    pub(crate) add_strip_nondebug_symbols_pass: Symbol<'lib, LLVMPY_AddStripNondebugSymbolsPass>,
    pub(crate) add_tail_call_elimination_pass: Symbol<'lib, LLVMPY_AddTailCallEliminationPass>,
    pub(crate) add_cfg_simplification_pass: Symbol<'lib, LLVMPY_AddCFGSimplificationPass>,
    pub(crate) add_gvn_pass: Symbol<'lib, LLVMPY_AddGVNPass>,
    pub(crate) add_instruction_combining_pass: Symbol<'lib, LLVMPY_AddInstructionCombiningPass>,
    pub(crate) add_licm_pass: Symbol<'lib, LLVMPY_AddLICMPass>,
    pub(crate) add_sccp_pass: Symbol<'lib, LLVMPY_AddSCCPPass>,
    pub(crate) add_sroa_pass: Symbol<'lib, LLVMPY_AddSROAPass>,
    pub(crate) add_type_based_alias_analysis_pass: Symbol<'lib, LLVMPY_AddTypeBasedAliasAnalysisPass>,
    pub(crate) add_basic_alias_analysis_pass: Symbol<'lib, LLVMPY_AddBasicAliasAnalysisPass>,
    pub(crate) llvm_add_loop_rotate_pass: Symbol<'lib, LLVMPY_LLVMAddLoopRotatePass>,
    pub(crate) get_process_triple: Symbol<'lib, LLVMPY_GetProcessTriple>,
    pub(crate) get_host_c_p_u_features: Symbol<'lib, LLVMPY_GetHostCPUFeatures>,
    pub(crate) get_default_target_triple: Symbol<'lib, LLVMPY_GetDefaultTargetTriple>,
    pub(crate) get_host_c_p_u_name: Symbol<'lib, LLVMPY_GetHostCPUName>,
    pub(crate) get_triple_object_format: Symbol<'lib, LLVMPY_GetTripleObjectFormat>,
    pub(crate) create_target_data: Symbol<'lib, LLVMPY_CreateTargetData>,
    pub(crate) copy_string_rep_of_target_data: Symbol<'lib, LLVMPY_CopyStringRepOfTargetData>,
    pub(crate) dispose_target_data: Symbol<'lib, LLVMPY_DisposeTargetData>,
    pub(crate) abi_size_of_type: Symbol<'lib, LLVMPY_ABISizeOfType>,
    pub(crate) offset_of_element: Symbol<'lib, LLVMPY_OffsetOfElement>,
    pub(crate) abi_size_of_element_type: Symbol<'lib, LLVMPY_ABISizeOfElementType>,
    pub(crate) abi_alignment_of_element_type: Symbol<'lib, LLVMPY_ABIAlignmentOfElementType>,
    pub(crate) get_target_from_triple: Symbol<'lib, LLVMPY_GetTargetFromTriple>,
    pub(crate) get_target_name: Symbol<'lib, LLVMPY_GetTargetName>,
    pub(crate) get_target_description: Symbol<'lib, LLVMPY_GetTargetDescription>,
    pub(crate) create_target_machine: Symbol<'lib, LLVMPY_CreateTargetMachine>,
    pub(crate) dispose_target_machine: Symbol<'lib, LLVMPY_DisposeTargetMachine>,
    pub(crate) get_target_machine_triple: Symbol<'lib, LLVMPY_GetTargetMachineTriple>,
    pub(crate) set_target_machine_asm_verbosity: Symbol<'lib, LLVMPY_SetTargetMachineAsmVerbosity>,
    pub(crate) target_machine_emit_to_memory: Symbol<'lib, LLVMPY_TargetMachineEmitToMemory>,
    pub(crate) create_target_machine_data: Symbol<'lib, LLVMPY_CreateTargetMachineData>,
    pub(crate) add_analysis_passes: Symbol<'lib, LLVMPY_AddAnalysisPasses>,
    pub(crate) get_buffer_start: Symbol<'lib, LLVMPY_GetBufferStart>,
    pub(crate) get_buffer_size: Symbol<'lib, LLVMPY_GetBufferSize>,
    pub(crate) dispose_memory_buffer: Symbol<'lib, LLVMPY_DisposeMemoryBuffer>,
    pub(crate) has_svml_support: Symbol<'lib, LLVMPY_HasSVMLSupport>,
    pub(crate) pass_manager_builder_create: Symbol<'lib, LLVMPY_PassManagerBuilderCreate>,
    pub(crate) pass_manager_builder_dispose: Symbol<'lib, LLVMPY_PassManagerBuilderDispose>,
    pub(crate) pass_manager_builder_get_opt_level: Symbol<'lib, LLVMPY_PassManagerBuilderGetOptLevel>,
    pub(crate) pass_manager_builder_set_opt_level: Symbol<'lib, LLVMPY_PassManagerBuilderSetOptLevel>,
    pub(crate) pass_manager_builder_get_size_level: Symbol<'lib, LLVMPY_PassManagerBuilderGetSizeLevel>,
    pub(crate) pass_manager_builder_set_size_level: Symbol<'lib, LLVMPY_PassManagerBuilderSetSizeLevel>,
    pub(crate) pass_manager_builder_get_disable_unroll_loops: Symbol<'lib, LLVMPY_PassManagerBuilderGetDisableUnrollLoops>,
    pub(crate) pass_manager_builder_set_disable_unroll_loops: Symbol<'lib, LLVMPY_PassManagerBuilderSetDisableUnrollLoops>,
    pub(crate) pass_manager_builder_use_inliner_with_threshold: Symbol<'lib, LLVMPY_PassManagerBuilderUseInlinerWithThreshold>,
    pub(crate) pass_manager_builder_populate_function_pass_manager: Symbol<'lib, LLVMPY_PassManagerBuilderPopulateFunctionPassManager>,
    pub(crate) pass_manager_builder_set_loop_vectorize: Symbol<'lib, LLVMPY_PassManagerBuilderSetLoopVectorize>,
    pub(crate) pass_manager_builder_get_loop_vectorize: Symbol<'lib, LLVMPY_PassManagerBuilderGetLoopVectorize>,
    pub(crate) pass_manager_builder_set_s_l_p_vectorize: Symbol<'lib, LLVMPY_PassManagerBuilderSetSLPVectorize>,
    pub(crate) pass_manager_builder_get_s_l_p_vectorize: Symbol<'lib, LLVMPY_PassManagerBuilderGetSLPVectorize>,
    pub(crate) function_attributes_iter: Symbol<'lib, LLVMPY_FunctionAttributesIter>,
    pub(crate) argument_attributes_iter: Symbol<'lib, LLVMPY_ArgumentAttributesIter>,
    pub(crate) call_inst_attributes_iter: Symbol<'lib, LLVMPY_CallInstAttributesIter>,
    pub(crate) invoke_inst_attributes_iter: Symbol<'lib, LLVMPY_InvokeInstAttributesIter>,
    pub(crate) global_attributes_iter: Symbol<'lib, LLVMPY_GlobalAttributesIter>,
    pub(crate) function_blocks_iter: Symbol<'lib, LLVMPY_FunctionBlocksIter>,
    pub(crate) function_arguments_iter: Symbol<'lib, LLVMPY_FunctionArgumentsIter>,
    pub(crate) block_instructions_iter: Symbol<'lib, LLVMPY_BlockInstructionsIter>,
    pub(crate) instruction_operands_iter: Symbol<'lib, LLVMPY_InstructionOperandsIter>,
    pub(crate) attribute_list_iter_next: Symbol<'lib, LLVMPY_AttributeListIterNext>,
    pub(crate) blocks_iter_next: Symbol<'lib, LLVMPY_BlocksIterNext>,
    pub(crate) arguments_iter_next: Symbol<'lib, LLVMPY_ArgumentsIterNext>,
    pub(crate) instructions_iter_next: Symbol<'lib, LLVMPY_InstructionsIterNext>,
    pub(crate) operands_iter_next: Symbol<'lib, LLVMPY_OperandsIterNext>,
    pub(crate) dispose_attribute_list_iter: Symbol<'lib, LLVMPY_DisposeAttributeListIter>,
    pub(crate) dispose_attribute_set_iter: Symbol<'lib, LLVMPY_DisposeAttributeSetIter>,
    pub(crate) dispose_blocks_iter: Symbol<'lib, LLVMPY_DisposeBlocksIter>,
    pub(crate) dispose_arguments_iter: Symbol<'lib, LLVMPY_DisposeArgumentsIter>,
    pub(crate) dispose_instructions_iter: Symbol<'lib, LLVMPY_DisposeInstructionsIter>,
    pub(crate) dispose_operands_iter: Symbol<'lib, LLVMPY_DisposeOperandsIter>,
    pub(crate) print_value_to_string: Symbol<'lib, LLVMPY_PrintValueToString>,
    pub(crate) get_value_name: Symbol<'lib, LLVMPY_GetValueName>,
    pub(crate) set_value_name: Symbol<'lib, LLVMPY_SetValueName>,
    pub(crate) get_global_parent: Symbol<'lib, LLVMPY_GetGlobalParent>,
    pub(crate) type_of: Symbol<'lib, LLVMPY_TypeOf>,
    pub(crate) print_type: Symbol<'lib, LLVMPY_PrintType>,
    pub(crate) get_type_name: Symbol<'lib, LLVMPY_GetTypeName>,
    pub(crate) type_is_pointer: Symbol<'lib, LLVMPY_TypeIsPointer>,
    pub(crate) get_element_type: Symbol<'lib, LLVMPY_GetElementType>,
    pub(crate) set_linkage: Symbol<'lib, LLVMPY_SetLinkage>,
    pub(crate) get_linkage: Symbol<'lib, LLVMPY_GetLinkage>,
    pub(crate) set_visibility: Symbol<'lib, LLVMPY_SetVisibility>,
    pub(crate) get_visibility: Symbol<'lib, LLVMPY_GetVisibility>,
    pub(crate) set_dll_storage_class: Symbol<'lib, LLVMPY_SetDLLStorageClass>,
    pub(crate) get_dll_storage_class: Symbol<'lib, LLVMPY_GetDLLStorageClass>,
    pub(crate) get_enum_attribute_kind_for_name: Symbol<'lib, LLVMPY_GetEnumAttributeKindForName>,
    pub(crate) add_function_attr: Symbol<'lib, LLVMPY_AddFunctionAttr>,
    pub(crate) is_declaration: Symbol<'lib, LLVMPY_IsDeclaration>,
    pub(crate) write_cfg: Symbol<'lib, LLVMPY_WriteCFG>,
    pub(crate) get_opcode_name: Symbol<'lib, LLVMPY_GetOpcodeName>,
}

impl<'lib> LLVMMethods<'lib> {
    unsafe fn load(lib: &'lib Library) -> Result<Self, LoadError> {
        Ok(Self {
            set_cmd_line: load_fn(lib, b"LLVMPY_SetCommandLine")?,
            parse_assembly: load_fn(lib, b"LLVMPY_ParseAssembly")?,
            write_bitcode_to_string: load_fn(lib, b"LLVMPY_WriteBitcodeToString")?,
            parse_bitcode: load_fn(lib, b"LLVMPY_ParseBitcode")?,
            create_string: load_fn(lib, b"LLVMPY_CreateString")?,
            create_byte_string: load_fn(lib, b"LLVMPY_CreateByteString")?,
            dispose_string: load_fn(lib, b"LLVMPY_DisposeString")?,
            get_global_context: load_fn(lib, b"LLVMPY_GetGlobalContext")?,
            context_create: load_fn(lib, b"LLVMPY_ContextCreate")?,
            context_dispose: load_fn(lib, b"LLVMPY_ContextDispose")?,
            add_ref_prune_pass: load_fn(lib, b"LLVMPY_AddRefPrunePass")?,
            dump_ref_prune_stats: load_fn(lib, b"LLVMPY_DumpRefPruneStats")?,
            search_address_of_symbol: load_fn(lib, b"LLVMPY_SearchAddressOfSymbol")?,
            add_symbol: load_fn(lib, b"LLVMPY_AddSymbol")?,
            load_library_permanently: load_fn(lib, b"LLVMPY_LoadLibraryPermanently")?,
            link_in_mc_jit: load_fn(lib, b"LLVMPY_LinkInMCJIT")?,
            dispose_execution_engine: load_fn(lib, b"LLVMPY_DisposeExecutionEngine")?,
            add_module: load_fn(lib, b"LLVMPY_AddModule")?,
            remove_module: load_fn(lib, b"LLVMPY_RemoveModule")?,
            finalize_object: load_fn(lib, b"LLVMPY_FinalizeObject")?,
            create_mc_jit_compiler: load_fn(lib, b"LLVMPY_CreateMCJITCompiler")?,
            get_global_value_address: load_fn(lib, b"LLVMPY_GetGlobalValueAddress")?,
            get_function_address: load_fn(lib, b"LLVMPY_GetFunctionAddress")?,
            run_static_constructors: load_fn(lib, b"LLVMPY_RunStaticConstructors")?,
            run_static_destructors: load_fn(lib, b"LLVMPY_RunStaticDestructors")?,
            add_global_mapping: load_fn(lib, b"LLVMPY_AddGlobalMapping")?,
            get_execution_engine_target_data: load_fn(lib, b"LLVMPY_GetExecutionEngineTargetData")?,
            try_allocate_executable_memory: load_fn(lib, b"LLVMPY_TryAllocateExecutableMemory")?,
            enable_jit_events: load_fn(lib, b"LLVMPY_EnableJITEvents")?,
            mc_jit_add_object_file: load_fn(lib, b"LLVMPY_MCJITAddObjectFile")?,
            create_object_cache: load_fn(lib, b"LLVMPY_CreateObjectCache")?,
            dispose_object_cache: load_fn(lib, b"LLVMPY_DisposeObjectCache")?,
            set_object_cache: load_fn(lib, b"LLVMPY_SetObjectCache")?,
            shutdown: load_fn(lib, b"LLVMPY_Shutdown")?,
            get_version_info: load_fn(lib, b"LLVMPY_GetVersionInfo")?,
            link_modules: load_fn(lib, b"LLVMPY_LinkModules")?,
            dispose_module: load_fn(lib, b"LLVMPY_DisposeModule")?,
            print_module_to_string: load_fn(lib, b"LLVMPY_PrintModuleToString")?,
            get_module_source_file_name: load_fn(lib, b"LLVMPY_GetModuleSourceFileName")?,
            get_module_name: load_fn(lib, b"LLVMPY_GetModuleName")?,
            set_module_name: load_fn(lib, b"LLVMPY_SetModuleName")?,
            get_named_function: load_fn(lib, b"LLVMPY_GetNamedFunction")?,
            get_named_global_variable: load_fn(lib, b"LLVMPY_GetNamedGlobalVariable")?,
            get_named_struct_type: load_fn(lib, b"LLVMPY_GetNamedStructType")?,
            verify_module: load_fn(lib, b"LLVMPY_VerifyModule")?,
            get_data_layout: load_fn(lib, b"LLVMPY_GetDataLayout")?,
            set_data_layout: load_fn(lib, b"LLVMPY_SetDataLayout")?,
            get_target: load_fn(lib, b"LLVMPY_GetTarget")?,
            set_target: load_fn(lib, b"LLVMPY_SetTarget")?,
            module_globals_iter: load_fn(lib, b"LLVMPY_ModuleGlobalsIter")?,
            module_functions_iter: load_fn(lib, b"LLVMPY_ModuleFunctionsIter")?,
            module_types_iter: load_fn(lib, b"LLVMPY_ModuleTypesIter")?,
            globals_iter_next: load_fn(lib, b"LLVMPY_GlobalsIterNext")?,
            functions_iter_next: load_fn(lib, b"LLVMPY_FunctionsIterNext")?,
            types_iter_next: load_fn(lib, b"LLVMPY_TypesIterNext")?,
            dispose_globals_iter: load_fn(lib, b"LLVMPY_DisposeGlobalsIter")?,
            dispose_functions_iter: load_fn(lib, b"LLVMPY_DisposeFunctionsIter")?,
            dispose_types_iter: load_fn(lib, b"LLVMPY_DisposeTypesIter")?,
            clone_module: load_fn(lib, b"LLVMPY_CloneModule")?,
            create_object_file: load_fn(lib, b"LLVMPY_CreateObjectFile")?,
            dispose_object_file: load_fn(lib, b"LLVMPY_DisposeObjectFile")?,
            get_sections: load_fn(lib, b"LLVMPY_GetSections")?,
            dispose_section_iterator: load_fn(lib, b"LLVMPY_DisposeSectionIterator")?,
            move_to_next_section: load_fn(lib, b"LLVMPY_MoveToNextSection")?,
            is_section_iterator_at_end: load_fn(lib, b"LLVMPY_IsSectionIteratorAtEnd")?,
            get_section_name: load_fn(lib, b"LLVMPY_GetSectionName")?,
            get_section_address: load_fn(lib, b"LLVMPY_GetSectionAddress")?,
            get_section_contents: load_fn(lib, b"LLVMPY_GetSectionContents")?,
            get_section_size: load_fn(lib, b"LLVMPY_GetSectionSize")?,
            is_section_text: load_fn(lib, b"LLVMPY_IsSectionText")?,
            set_time_passes: load_fn(lib, b"LLVMPY_SetTimePasses")?,
            report_and_reset_timings: load_fn(lib, b"LLVMPY_ReportAndResetTimings")?,
            create_pass_manager: load_fn(lib, b"LLVMPY_CreatePassManager")?,
            dispose_pass_manager: load_fn(lib, b"LLVMPY_DisposePassManager")?,
            create_function_pass_manager: load_fn(lib, b"LLVMPY_CreateFunctionPassManager")?,
            run_pass_manager_with_remarks: load_fn(lib, b"LLVMPY_RunPassManagerWithRemarks")?,
            run_pass_manager: load_fn(lib, b"LLVMPY_RunPassManager")?,
            run_function_pass_manager_with_remarks: load_fn(lib, b"LLVMPY_RunFunctionPassManagerWithRemarks")?,
            run_function_pass_manager: load_fn(lib, b"LLVMPY_RunFunctionPassManager")?,
            initialize_function_pass_manager: load_fn(lib, b"LLVMPY_InitializeFunctionPassManager")?,
            finalize_function_pass_manager: load_fn(lib, b"LLVMPY_FinalizeFunctionPassManager")?,
            add_a_a_eval_pass: load_fn(lib, b"LLVMPY_AddAAEvalPass")?,
            add_basic_a_a_wrapper_pass: load_fn(lib, b"LLVMPY_AddBasicAAWrapperPass")?,
            add_dependence_analysis_pass: load_fn(lib, b"LLVMPY_AddDependenceAnalysisPass")?,
            add_call_graph_dot_printer_pass: load_fn(lib, b"LLVMPY_AddCallGraphDOTPrinterPass")?,
            add_dot_dom_printer_pass: load_fn(lib, b"LLVMPY_AddDotDomPrinterPass")?,
            add_globals_mod_ref_a_a_pass: load_fn(lib, b"LLVMPY_AddGlobalsModRefAAPass")?,
            add_dot_post_dom_printer_pass: load_fn(lib, b"LLVMPY_AddDotPostDomPrinterPass")?,
            add_cfg_printer_pass: load_fn(lib, b"LLVMPY_AddCFGPrinterPass")?,
            add_constant_merge_pass: load_fn(lib, b"LLVMPY_AddConstantMergePass")?,
            add_dead_store_elimination_pass: load_fn(lib, b"LLVMPY_AddDeadStoreEliminationPass")?,
            add_reverse_post_order_function_attrs_pass: load_fn(lib, b"LLVMPY_AddReversePostOrderFunctionAttrsPass")?,
            add_dead_arg_elimination_pass: load_fn(lib, b"LLVMPY_AddDeadArgEliminationPass")?,
            add_instruction_count_pass: load_fn(lib, b"LLVMPY_AddInstructionCountPass")?,
            add_iv_users_pass: load_fn(lib, b"LLVMPY_AddIVUsersPass")?,
            add_lazy_value_info_pass: load_fn(lib, b"LLVMPY_AddLazyValueInfoPass")?,
            add_lint_pass: load_fn(lib, b"LLVMPY_AddLintPass")?,
            add_module_debug_info_printer_pass: load_fn(lib, b"LLVMPY_AddModuleDebugInfoPrinterPass")?,
            add_region_info_pass: load_fn(lib, b"LLVMPY_AddRegionInfoPass")?,
            add_scalar_evolution_a_a_pass: load_fn(lib, b"LLVMPY_AddScalarEvolutionAAPass")?,
            add_aggressive_d_c_e_pass: load_fn(lib, b"LLVMPY_AddAggressiveDCEPass")?,
            add_always_inliner_pass: load_fn(lib, b"LLVMPY_AddAlwaysInlinerPass")?,
            add_arg_promotion_pass: load_fn(lib, b"LLVMPY_AddArgPromotionPass")?,
            add_break_critical_edges_pass: load_fn(lib, b"LLVMPY_AddBreakCriticalEdgesPass")?,
            add_function_attrs_pass: load_fn(lib, b"LLVMPY_AddFunctionAttrsPass")?,
            add_function_inlining_pass: load_fn(lib, b"LLVMPY_AddFunctionInliningPass")?,
            add_global_optimizer_pass: load_fn(lib, b"LLVMPY_AddGlobalOptimizerPass")?,
            add_global_dce_pass: load_fn(lib, b"LLVMPY_AddGlobalDCEPass")?,
            add_ipsccp_pass: load_fn(lib, b"LLVMPY_AddIPSCCPPass")?,
            add_dead_code_elimination_pass: load_fn(lib, b"LLVMPY_AddDeadCodeEliminationPass")?,
            add_aggressive_instruction_combining_pass: load_fn(lib, b"LLVMPY_AddAggressiveInstructionCombiningPass")?,
            add_internalize_pass: load_fn(lib, b"LLVMPY_AddInternalizePass")?,
            add_jump_threading_pass: load_fn(lib, b"LLVMPY_AddJumpThreadingPass")?,
            add_lcssa_pass: load_fn(lib, b"LLVMPY_AddLCSSAPass")?,
            add_loop_deletion_pass: load_fn(lib, b"LLVMPY_AddLoopDeletionPass")?,
            add_single_loop_extractor_pass: load_fn(lib, b"LLVMPY_AddSingleLoopExtractorPass")?,
            add_loop_strength_reduce_pass: load_fn(lib, b"LLVMPY_AddLoopStrengthReducePass")?,
            add_loop_simplification_pass: load_fn(lib, b"LLVMPY_AddLoopSimplificationPass")?,
            add_loop_unroll_pass: load_fn(lib, b"LLVMPY_AddLoopUnrollPass")?,
            add_loop_unroll_and_jam_pass: load_fn(lib, b"LLVMPY_AddLoopUnrollAndJamPass")?,
            add_loop_unswitch_pass: load_fn(lib, b"LLVMPY_AddLoopUnswitchPass")?,
            add_lower_atomic_pass: load_fn(lib, b"LLVMPY_AddLowerAtomicPass")?,
            add_lower_invoke_pass: load_fn(lib, b"LLVMPY_AddLowerInvokePass")?,
            add_lower_switch_pass: load_fn(lib, b"LLVMPY_AddLowerSwitchPass")?,
            add_mem_cpy_optimization_pass: load_fn(lib, b"LLVMPY_AddMemCpyOptimizationPass")?,
            add_merge_functions_pass: load_fn(lib, b"LLVMPY_AddMergeFunctionsPass")?,
            add_merge_returns_pass: load_fn(lib, b"LLVMPY_AddMergeReturnsPass")?,
            add_partial_inlining_pass: load_fn(lib, b"LLVMPY_AddPartialInliningPass")?,
            add_prune_exception_handling_pass: load_fn(lib, b"LLVMPY_AddPruneExceptionHandlingPass")?,
            add_re_associate_pass: load_fn(lib, b"LLVMPY_AddReassociatePass")?,
            add_demote_register_to_memory_pass: load_fn(lib, b"LLVMPY_AddDemoteRegisterToMemoryPass")?,
            add_sink_pass: load_fn(lib, b"LLVMPY_AddSinkPass")?,
            add_strip_symbols_pass: load_fn(lib, b"LLVMPY_AddStripSymbolsPass")?,
            add_strip_dead_debug_info_pass: load_fn(lib, b"LLVMPY_AddStripDeadDebugInfoPass")?,
            add_strip_dead_prototypes_pass: load_fn(lib, b"LLVMPY_AddStripDeadPrototypesPass")?,
            add_strip_debug_declare_prototypes_pass: load_fn(lib, b"LLVMPY_AddStripDebugDeclarePrototypesPass")?,
            add_strip_nondebug_symbols_pass: load_fn(lib, b"LLVMPY_AddStripNondebugSymbolsPass")?,
            add_tail_call_elimination_pass: load_fn(lib, b"LLVMPY_AddTailCallEliminationPass")?,
            add_cfg_simplification_pass: load_fn(lib, b"LLVMPY_AddCFGSimplificationPass")?,
            add_gvn_pass: load_fn(lib, b"LLVMPY_AddGVNPass")?,
            add_instruction_combining_pass: load_fn(lib, b"LLVMPY_AddInstructionCombiningPass")?,
            add_licm_pass: load_fn(lib, b"LLVMPY_AddLICMPass")?,
            add_sccp_pass: load_fn(lib, b"LLVMPY_AddSCCPPass")?,
            add_sroa_pass: load_fn(lib, b"LLVMPY_AddSROAPass")?,
            add_type_based_alias_analysis_pass: load_fn(lib, b"LLVMPY_AddTypeBasedAliasAnalysisPass")?,
            add_basic_alias_analysis_pass: load_fn(lib, b"LLVMPY_AddBasicAliasAnalysisPass")?,
            llvm_add_loop_rotate_pass: load_fn(lib, b"LLVMPY_LLVMAddLoopRotatePass")?,
            get_process_triple: load_fn(lib, b"LLVMPY_GetProcessTriple")?,
            get_host_c_p_u_features: load_fn(lib, b"LLVMPY_GetHostCPUFeatures")?,
            get_default_target_triple: load_fn(lib, b"LLVMPY_GetDefaultTargetTriple")?,
            get_host_c_p_u_name: load_fn(lib, b"LLVMPY_GetHostCPUName")?,
            get_triple_object_format: load_fn(lib, b"LLVMPY_GetTripleObjectFormat")?,
            create_target_data: load_fn(lib, b"LLVMPY_CreateTargetData")?,
            copy_string_rep_of_target_data: load_fn(lib, b"LLVMPY_CopyStringRepOfTargetData")?,
            dispose_target_data: load_fn(lib, b"LLVMPY_DisposeTargetData")?,
            abi_size_of_type: load_fn(lib, b"LLVMPY_ABISizeOfType")?,
            offset_of_element: load_fn(lib, b"LLVMPY_OffsetOfElement")?,
            abi_size_of_element_type: load_fn(lib, b"LLVMPY_ABISizeOfElementType")?,
            abi_alignment_of_element_type: load_fn(lib, b"LLVMPY_ABIAlignmentOfElementType")?,
            get_target_from_triple: load_fn(lib, b"LLVMPY_GetTargetFromTriple")?,
            get_target_name: load_fn(lib, b"LLVMPY_GetTargetName")?,
            get_target_description: load_fn(lib, b"LLVMPY_GetTargetDescription")?,
            create_target_machine: load_fn(lib, b"LLVMPY_CreateTargetMachine")?,
            dispose_target_machine: load_fn(lib, b"LLVMPY_DisposeTargetMachine")?,
            get_target_machine_triple: load_fn(lib, b"LLVMPY_GetTargetMachineTriple")?,
            set_target_machine_asm_verbosity: load_fn(lib, b"LLVMPY_SetTargetMachineAsmVerbosity")?,
            target_machine_emit_to_memory: load_fn(lib, b"LLVMPY_TargetMachineEmitToMemory")?,
            create_target_machine_data: load_fn(lib, b"LLVMPY_CreateTargetMachineData")?,
            add_analysis_passes: load_fn(lib, b"LLVMPY_AddAnalysisPasses")?,
            get_buffer_start: load_fn(lib, b"LLVMPY_GetBufferStart")?,
            get_buffer_size: load_fn(lib, b"LLVMPY_GetBufferSize")?,
            dispose_memory_buffer: load_fn(lib, b"LLVMPY_DisposeMemoryBuffer")?,
            has_svml_support: load_fn(lib, b"LLVMPY_HasSVMLSupport")?,
            pass_manager_builder_create: load_fn(lib, b"LLVMPY_PassManagerBuilderCreate")?,
            pass_manager_builder_dispose: load_fn(lib, b"LLVMPY_PassManagerBuilderDispose")?,
            pass_manager_builder_get_opt_level: load_fn(lib, b"LLVMPY_PassManagerBuilderGetOptLevel")?,
            pass_manager_builder_set_opt_level: load_fn(lib, b"LLVMPY_PassManagerBuilderSetOptLevel")?,
            pass_manager_builder_get_size_level: load_fn(lib, b"LLVMPY_PassManagerBuilderGetSizeLevel")?,
            pass_manager_builder_set_size_level: load_fn(lib, b"LLVMPY_PassManagerBuilderSetSizeLevel")?,
            pass_manager_builder_get_disable_unroll_loops: load_fn(lib, b"LLVMPY_PassManagerBuilderGetDisableUnrollLoops")?,
            pass_manager_builder_set_disable_unroll_loops: load_fn(lib, b"LLVMPY_PassManagerBuilderSetDisableUnrollLoops")?,
            pass_manager_builder_use_inliner_with_threshold: load_fn(lib, b"LLVMPY_PassManagerBuilderUseInlinerWithThreshold")?,
            pass_manager_builder_populate_function_pass_manager: load_fn(lib, b"LLVMPY_PassManagerBuilderPopulateFunctionPassManager")?,
            pass_manager_builder_set_loop_vectorize: load_fn(lib, b"LLVMPY_PassManagerBuilderSetLoopVectorize")?,
            pass_manager_builder_get_loop_vectorize: load_fn(lib, b"LLVMPY_PassManagerBuilderGetLoopVectorize")?,
            pass_manager_builder_set_s_l_p_vectorize: load_fn(lib, b"LLVMPY_PassManagerBuilderSetSLPVectorize")?,
            pass_manager_builder_get_s_l_p_vectorize: load_fn(lib, b"LLVMPY_PassManagerBuilderGetSLPVectorize")?,
            function_attributes_iter: load_fn(lib, b"LLVMPY_FunctionAttributesIter")?,
            argument_attributes_iter: load_fn(lib, b"LLVMPY_ArgumentAttributesIter")?,
            call_inst_attributes_iter: load_fn(lib, b"LLVMPY_CallInstAttributesIter")?,
            invoke_inst_attributes_iter: load_fn(lib, b"LLVMPY_InvokeInstAttributesIter")?,
            global_attributes_iter: load_fn(lib, b"LLVMPY_GlobalAttributesIter")?,
            function_blocks_iter: load_fn(lib, b"LLVMPY_FunctionBlocksIter")?,
            function_arguments_iter: load_fn(lib, b"LLVMPY_FunctionArgumentsIter")?,
            block_instructions_iter: load_fn(lib, b"LLVMPY_BlockInstructionsIter")?,
            instruction_operands_iter: load_fn(lib, b"LLVMPY_InstructionOperandsIter")?,
            attribute_list_iter_next: load_fn(lib, b"LLVMPY_AttributeListIterNext")?,
            blocks_iter_next: load_fn(lib, b"LLVMPY_BlocksIterNext")?,
            arguments_iter_next: load_fn(lib, b"LLVMPY_ArgumentsIterNext")?,
            instructions_iter_next: load_fn(lib, b"LLVMPY_InstructionsIterNext")?,
            operands_iter_next: load_fn(lib, b"LLVMPY_OperandsIterNext")?,
            dispose_attribute_list_iter: load_fn(lib, b"LLVMPY_DisposeAttributeListIter")?,
            dispose_attribute_set_iter: load_fn(lib, b"LLVMPY_DisposeAttributeSetIter")?,
            dispose_blocks_iter: load_fn(lib, b"LLVMPY_DisposeBlocksIter")?,
            dispose_arguments_iter: load_fn(lib, b"LLVMPY_DisposeArgumentsIter")?,
            dispose_instructions_iter: load_fn(lib, b"LLVMPY_DisposeInstructionsIter")?,
            dispose_operands_iter: load_fn(lib, b"LLVMPY_DisposeOperandsIter")?,
            print_value_to_string: load_fn(lib, b"LLVMPY_PrintValueToString")?,
            get_value_name: load_fn(lib, b"LLVMPY_GetValueName")?,
            set_value_name: load_fn(lib, b"LLVMPY_SetValueName")?,
            get_global_parent: load_fn(lib, b"LLVMPY_GetGlobalParent")?,
            type_of: load_fn(lib, b"LLVMPY_TypeOf")?,
            print_type: load_fn(lib, b"LLVMPY_PrintType")?,
            get_type_name: load_fn(lib, b"LLVMPY_GetTypeName")?,
            type_is_pointer: load_fn(lib, b"LLVMPY_TypeIsPointer")?,
            get_element_type: load_fn(lib, b"LLVMPY_GetElementType")?,
            set_linkage: load_fn(lib, b"LLVMPY_SetLinkage")?,
            get_linkage: load_fn(lib, b"LLVMPY_GetLinkage")?,
            set_visibility: load_fn(lib, b"LLVMPY_SetVisibility")?,
            get_visibility: load_fn(lib, b"LLVMPY_GetVisibility")?,
            set_dll_storage_class: load_fn(lib, b"LLVMPY_SetDLLStorageClass")?,
            get_dll_storage_class: load_fn(lib, b"LLVMPY_GetDLLStorageClass")?,
            get_enum_attribute_kind_for_name: load_fn(lib, b"LLVMPY_GetEnumAttributeKindForName")?,
            add_function_attr: load_fn(lib, b"LLVMPY_AddFunctionAttr")?,
            is_declaration: load_fn(lib, b"LLVMPY_IsDeclaration")?,
            write_cfg: load_fn(lib, b"LLVMPY_WriteCFG")?,
            get_opcode_name: load_fn(lib, b"LLVMPY_GetOpcodeName")?,
        })
    }
}

unsafe fn load_fn<'lib, T>(lib: &'lib Library, name: &[u8]) -> Result<Symbol<'lib, T>, LoadError> {
    let pretty_name = String::from_utf8_lossy(name);

    lib.get(name)
        .map_err(|e| LoadError::LoadFunction {
            inner: e,
            target: pretty_name.to_string(),
        })
}
