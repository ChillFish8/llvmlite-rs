use std::ffi::{c_char, c_void};

use crate::types::*;

extern "C" {
    pub fn LLVMPY_ContextDispose(context: LLVMContextRef);

    pub fn LLVMPY_SetCommandLine(name: *const c_char, option: *const c_char);

    pub fn LLVMPY_ParseAssembly(
        context: LLVMContextRef,
        ir: *const c_char,
        out_msg: *mut *const c_char,
    ) -> LLVMContextRef;
    
    pub fn LLVMPY_WriteBitcodeToString(
        context: LLVMContextRef,
        out_buf: *mut *const c_char,
        out_len: *mut usize,
    );

    pub fn LLVMPY_ParseBitcode(
        context: LLVMContextRef,
        bit_code: *const c_char,
        bit_code_len: usize,
        out_msg: *mut *const c_char,
    ) -> LLVMModuleRef;
    
    pub fn LLVMPY_CreateString(msg: *const c_char) -> *const c_char;
    
    pub fn LLVMPY_CreateByteString(msg: *const c_char, len: usize) -> *const c_char;
    
    pub fn LLVMPY_DisposeString(msg: *const c_char);
    
    pub fn LLVMPY_GetGlobalContext() -> LLVMContextRef;
    
    pub fn LLVMPY_ContextCreate() -> LLVMContextRef;
    
    pub fn LLVMPY_AddRefPrunePass(pass_manager: LLVMPassManagerRef, sub_passes: i32, subgraph_limit: usize);
    
    pub fn LLVMPY_DumpRefPruneStats(buf: *mut PruneStats, display: bool);
    
    pub fn LLVMPY_SearchAddressOfSymbol(name: *const c_char) -> *const c_void;
    
    pub fn LLVMPY_AddSymbol(name: *const c_char, addr: *const c_void);
    
    pub fn LLVMPY_LoadLibraryPermanently(filename: *const c_char, out_error: *mut *const c_char) -> bool;
    
    pub fn LLVMPY_LinkInMCJIT();

    pub fn LLVMPY_DisposeExecutionEngine(execution_engine: LLVMExecutionEngineRef);

    pub fn LLVMPY_AddModule(execution_engine: LLVMExecutionEngineRef, module: LLVMModuleRef);

    pub fn LLVMPY_RemoveModule(
        execution_engine: LLVMExecutionEngineRef,
        module: LLVMModuleRef,
        out_error: *mut *mut c_char
    ) -> i32;

    pub fn LLVMPY_FinalizeObject(execution_engine: LLVMExecutionEngineRef);

    pub fn LLVMPY_CreateMCJITCompiler(
        execution_engine: LLVMExecutionEngineRef,
        target_machine: LLVMTargetMachineRef,
        out_error: *mut *const c_char,
    ) -> LLVMExecutionEngineRef;

    pub fn LLVMPY_GetGlobalValueAddress(
        execution_engine: LLVMExecutionEngineRef,
        name: *const c_char,
    ) -> u64;

    pub fn LLVMPY_GetFunctionAddress(
        execution_engine: LLVMExecutionEngineRef,
        name: *const c_char,
    ) -> u64;

    pub fn LLVMPY_RunStaticConstructors(execution_engine: LLVMExecutionEngineRef);

    pub fn LLVMPY_RunStaticDestructors(execution_engine: LLVMExecutionEngineRef);

    pub fn LLVMPY_AddGlobalMapping(execution_engine: LLVMExecutionEngineRef);

    pub fn LLVMPY_GetExecutionEngineTargetData(
        execution_engine: LLVMExecutionEngineRef
    ) -> LLVMTargetDataRef;

    pub fn LLVMPY_TryAllocateExecutableMemory() -> i32;

    pub fn LLVMPY_EnableJITEvents(execution_engine: LLVMExecutionEngineRef) -> bool;

    pub fn LLVMPY_MCJITAddObjectFile(
        execution_engine: LLVMExecutionEngineRef,
        object_file: LLVMObjectFileRef,
    );

    pub fn LLVMPY_CreateObjectCache(
        notify_func: ObjectCacheNotifyFunc,
        get_object_func: ObjectCacheGetObjectFunc,
        user_data: *mut c_void,
    ) -> LLVMObjectCacheRef;

    pub fn LLVMPY_DisposeObjectCache(cache: LLVMObjectCacheRef);

    pub fn LLVMPY_SetObjectCache(
        execution_engine: LLVMExecutionEngineRef,
        cache: LLVMObjectCacheRef
    );
    
    pub fn LLVMPY_Shutdown();

    pub fn LLVMPY_GetVersionInfo() -> u32;
    
    pub fn LLVMPY_LinkModules(dest: LLVMModuleRef, src: LLVMModuleRef, err: *mut *const c_char) -> i32;
    
    pub fn LLVMPY_DisposeModule(module: LLVMModuleRef);

    pub fn LLVMPY_PrintModuleToString(module: LLVMModuleRef, out_str: *mut *const c_char);

    pub fn LLVMPY_GetModuleSourceFileName(module: LLVMModuleRef) -> *const c_char;

    pub fn LLVMPY_GetModuleName(module: LLVMModuleRef) -> *const c_char;

    pub fn LLVMPY_SetModuleName(module: LLVMModuleRef, name: *const c_char);

    pub fn LLVMPY_GetNamedFunction(module: LLVMModuleRef, name: *const c_char) -> LLVMValueRef;

    pub fn LLVMPY_GetNamedGlobalVariable(module: LLVMModuleRef, name: *const c_char) -> LLVMValueRef;

    pub fn LLVMPY_GetNamedStructType(module: LLVMModuleRef, name: *const c_char) -> LLVMTypeRef;

    pub fn LLVMPY_VerifyModule(module: LLVMModuleRef, out_msg: *mut *const c_char) -> i32;

    pub fn LLVMPY_GetDataLayout(module: LLVMModuleRef, layout: *mut *const c_char);

    pub fn LLVMPY_SetDataLayout(module: LLVMModuleRef, layout: *const c_char);

    pub fn LLVMPY_GetTarget(module: LLVMModuleRef, triple: *mut *const c_char);

    pub fn LLVMPY_SetTarget(module: LLVMModuleRef, triple: *const c_char);

    pub fn LLVMPY_ModuleGlobalsIter(module: LLVMModuleRef) -> LLVMGlobalsIteratorRef;

    pub fn LLVMPY_ModuleFunctionsIter(module: LLVMModuleRef) -> LLVMFunctionsIteratorRef;

    pub fn LLVMPY_ModuleTypesIter(module: LLVMModuleRef) -> LLVMTypesIteratorRef;

    pub fn LLVMPY_GlobalsIterNext(module: LLVMGlobalsIteratorRef) -> LLVMValueRef;

    pub fn LLVMPY_FunctionsIterNext(iter: LLVMFunctionsIteratorRef) -> LLVMValueRef;

    pub fn LLVMPY_TypesIterNext(iter: LLVMTypesIteratorRef) -> LLVMTypeRef;

    pub fn LLVMPY_DisposeGlobalsIter(iter: LLVMGlobalsIteratorRef);

    pub fn LLVMPY_DisposeFunctionsIter(iter: LLVMFunctionsIteratorRef);

    pub fn LLVMPY_DisposeTypesIter(iter: LLVMTypesIteratorRef);

    pub fn LLVMPY_CloneModule(module: LLVMModuleRef) -> LLVMModuleRef;
    
    pub fn LLVMPY_CreateObjectFile(buf: *const c_char, n: usize) -> LLVMObjectFileRef;

    pub fn LLVMPY_DisposeObjectFile(object: LLVMObjectFileRef);

    pub fn LLVMPY_GetSections(object: LLVMObjectFileRef) -> LLVMSectionIteratorRef;

    pub fn LLVMPY_DisposeSectionIterator(section_iter: LLVMSectionIteratorRef);

    pub fn LLVMPY_MoveToNextSection(section_iter: LLVMSectionIteratorRef);

    pub fn LLVMPY_IsSectionIteratorAtEnd(
        object: LLVMObjectFileRef,
        section_iter: LLVMSectionIteratorRef,
    ) -> bool;

    pub fn LLVMPY_GetSectionName(section_iter: LLVMSectionIteratorRef) -> *const c_char;

    pub fn LLVMPY_GetSectionAddress(section_iter: LLVMSectionIteratorRef) -> u64;

    pub fn LLVMPY_GetSectionContents(section_iter: LLVMSectionIteratorRef) -> *const c_char;

    pub fn LLVMPY_GetSectionSize(section_iter: LLVMSectionIteratorRef) -> u64;

    pub fn LLVMPY_IsSectionText(section_iter: LLVMSectionIteratorRef) -> bool;
    
    pub fn LLVMPY_SetTimePasses(enable: bool);

    pub fn LLVMPY_ReportAndResetTimings(out_msg: *mut *const c_char);

    pub fn LLVMPY_CreatePassManager() -> LLVMPassManagerRef;

    pub fn LLVMPY_DisposePassManager(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_CreateFunctionPassManager(module: LLVMModuleRef) -> LLVMPassManagerRef;

    pub fn LLVMPY_RunPassManagerWithRemarks(
        pass_manager: LLVMPassManagerRef,
        module: LLVMModuleRef,
        remarks_format: *const c_char,
        remarks_filter: *const c_char,
        record_filename: *const c_char,
    ) -> i32;

    pub fn LLVMPY_RunPassManager(
        pass_manager: LLVMPassManagerRef,
        module: LLVMModuleRef,
    ) -> i32;

    pub fn LLVMPY_RunFunctionPassManagerWithRemarks(
        pass_manager: LLVMPassManagerRef,
        value: LLVMValueRef,
        remarks_format: *const c_char,
        remarks_filter: *const c_char,
        record_filename: *const c_char,
    ) -> i32;

    pub fn LLVMPY_RunFunctionPassManager(
        pass_manager: LLVMPassManagerRef,
        value: LLVMValueRef,
    ) -> i32;

    pub fn LLVMPY_InitializeFunctionPassManager(pass_manager: LLVMPassManagerRef) -> i32;

    pub fn LLVMPY_FinalizeFunctionPassManager(pass_manager: LLVMPassManagerRef) -> i32;

    pub fn LLVMPY_AddAAEvalPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddBasicAAWrapperPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddDependenceAnalysisPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddCallGraphDOTPrinterPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddDotDomPrinterPass(pass_manager: LLVMPassManagerRef, show_body: bool);

    pub fn LLVMPY_AddGlobalsModRefAAPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddDotPostDomPrinterPass(pass_manager: LLVMPassManagerRef, show_body: bool);

    pub fn LLVMPY_AddCFGPrinterPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddConstantMergePass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddDeadStoreEliminationPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddReversePostOrderFunctionAttrsPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddDeadArgEliminationPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddInstructionCountPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddIVUsersPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddLazyValueInfoPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddLintPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddModuleDebugInfoPrinterPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddRegionInfoPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddScalarEvolutionAAPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddAggressiveDCEPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddAlwaysInlinerPass(pass_manager: LLVMPassManagerRef, insert_lifetime: bool);

    pub fn LLVMPY_AddArgPromotionPass(pass_manager: LLVMPassManagerRef, max_elements: u32);

    pub fn LLVMPY_AddBreakCriticalEdgesPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddFunctionAttrsPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddFunctionInliningPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddGlobalOptimizerPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddGlobalDCEPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddIPSCCPPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddDeadCodeEliminationPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddAggressiveInstructionCombiningPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddInternalizePass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddJumpThreadingPass(pass_manager: LLVMPassManagerRef, threshold: i32);

    pub fn LLVMPY_AddLCSSAPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddLoopDeletionPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddSingleLoopExtractorPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddLoopStrengthReducePass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddLoopSimplificationPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddLoopUnrollPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddLoopUnrollAndJamPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddLoopUnswitchPass(
        pass_manager: LLVMPassManagerRef,
        optimise_for_size: bool,
        has_branch_divergence: bool,
    );

    pub fn LLVMPY_AddLowerAtomicPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddLowerInvokePass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddLowerSwitchPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddMemCpyOptimizationPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddMergeFunctionsPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddMergeReturnsPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddPartialInliningPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddPruneExceptionHandlingPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddReassociatePass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddDemoteRegisterToMemoryPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddSinkPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddStripSymbolsPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddStripDeadDebugInfoPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddStripDeadPrototypesPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddStripDebugDeclarePrototypesPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddStripNondebugSymbolsPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddTailCallEliminationPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddCFGSimplificationPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddGVNPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddInstructionCombiningPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddLICMPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddSCCPPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddSROAPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddTypeBasedAliasAnalysisPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_AddBasicAliasAnalysisPass(pass_manager: LLVMPassManagerRef);

    pub fn LLVMPY_LLVMAddLoopRotatePass(pass_manager: LLVMPassManagerRef);
    
    pub fn LLVMPY_GetProcessTriple(out: *mut *const c_char);

    pub fn LLVMPY_GetHostCPUFeatures(out: *mut *const c_char) -> i32;

    pub fn LLVMPY_GetDefaultTargetTriple(out: *mut *const c_char);

    pub fn LLVMPY_GetHostCPUName(out: *mut *const c_char);

    pub fn LLVMPY_GetTripleObjectFormat(triple_str: *mut *const c_char) -> i32;

    pub fn LLVMPY_CreateTargetData(str_rep: *mut *const c_char) -> LLVMTargetDataRef;

    pub fn LLVMPY_CopyStringRepOfTargetData(
        target_data: LLVMTargetDataRef,
        out: *mut *mut c_char,
    );

    pub fn LLVMPY_DisposeTargetData(target_data: LLVMTargetDataRef);

    pub fn LLVMPY_ABISizeOfType(target_data: LLVMTargetDataRef, ty: LLVMTypeRef) -> i64;

    pub fn LLVMPY_OffsetOfElement(target_data: LLVMTargetDataRef, element: i32) -> i64;

    pub fn LLVMPY_ABISizeOfElementType(target_data: LLVMTargetDataRef, ty: LLVMTypeRef) -> i64;

    pub fn LLVMPY_ABIAlignmentOfElementType(target_data: LLVMTargetDataRef, ty: LLVMTypeRef) -> i64;

    pub fn LLVMPY_GetTargetFromTriple(
        triple: *mut *const c_char,
        err_out: *mut *const c_char,
    ) -> LLVMTargetRef;

    pub fn LLVMPY_GetTargetName(target: LLVMTargetRef) -> *const c_char;

    pub fn LLVMPY_GetTargetDescription(target: LLVMTargetRef) -> *const c_char;

    pub fn LLVMPY_CreateTargetMachine(
        target: LLVMTargetRef,
        triple: *const c_char,
        cpu: *const c_char,
        features: *const c_char,
        opt_level: i32,
        reloc_model: *const c_char,
        code_model: *const c_char,
        print_mc: i32,
        jit: i32,
        abi_name: *const c_char,
    ) -> LLVMTargetMachineRef;

    pub fn LLVMPY_DisposeTargetMachine(target_machine: LLVMTargetMachineRef);

    pub fn LLVMPY_GetTargetMachineTriple(
        target_machine: LLVMTargetMachineRef,
        out: *mut *const c_char,
    );

    pub fn LLVMPY_SetTargetMachineAsmVerbosity(
        target_machine: LLVMTargetMachineRef,
        verbose: i32,
    );

    pub fn LLVMPY_TargetMachineEmitToMemory(
        target_machine: LLVMTargetMachineRef,
        module: LLVMModuleRef,
        use_object: i32,
        err_out: *mut *const c_char,
    ) -> LLVMMemoryBufferRef;

    pub fn LLVMPY_CreateTargetMachineData(
        target_machine: LLVMTargetMachineRef,
    ) -> LLVMTargetDataRef;

    pub fn LLVMPY_AddAnalysisPasses(
        target_machine: LLVMTargetMachineRef,
        pass_manager: LLVMPassManagerRef,
    );

    pub fn LLVMPY_GetBufferStart(buffer: LLVMMemoryBufferRef) -> *const c_void;

    pub fn LLVMPY_GetBufferSize(buffer: LLVMMemoryBufferRef) -> usize;  // TODO: may need to be isize for size_t

    pub fn LLVMPY_DisposeMemoryBuffer(buffer: LLVMMemoryBufferRef);

    pub fn LLVMPY_HasSVMLSupport() -> i32;
    
        pub fn LLVMPY_PassManagerBuilderCreate() -> LLVMPassManagerBuilderRef;

    pub fn LLVMPY_PassManagerBuilderDispose(builder: LLVMPassManagerBuilderRef);

    pub fn LLVMPY_PassManagerBuilderGetOptLevel(builder: LLVMPassManagerBuilderRef) -> usize;

    pub fn LLVMPY_PassManagerBuilderSetOptLevel(
        builder: LLVMPassManagerBuilderRef,
        level: usize,
    );

    pub fn LLVMPY_PassManagerBuilderGetSizeLevel(builder: LLVMPassManagerBuilderRef) -> usize;

    pub fn LLVMPY_PassManagerBuilderSetSizeLevel(
        builder: LLVMPassManagerBuilderRef,
        level: usize
    );

    pub fn LLVMPY_PassManagerBuilderGetDisableUnrollLoops(builder: LLVMPassManagerBuilderRef) -> i32;

    pub fn LLVMPY_PassManagerBuilderSetDisableUnrollLoops(
        builder: LLVMPassManagerBuilderRef,
        value: LLVMBool,
    );

    pub fn LLVMPY_PassManagerBuilderUseInlinerWithThreshold(
        builder: LLVMPassManagerBuilderRef,
        threshold: usize,
    );

    pub fn LLVMPY_PassManagerBuilderPopulateFunctionPassManager(
        builder: LLVMPassManagerBuilderRef,
        manager: LLVMPassManagerRef,
    );

    pub fn LLVMPY_PassManagerBuilderSetLoopVectorize(
        builder: LLVMPassManagerBuilderRef,
        value: i32,
    );

    pub fn LLVMPY_PassManagerBuilderGetLoopVectorize(
        builder: LLVMPassManagerBuilderRef,
    ) -> i32;

    pub fn LLVMPY_PassManagerBuilderSetSLPVectorize(
        builder: LLVMPassManagerBuilderRef,
        value: i32,
    );

    pub fn LLVMPY_PassManagerBuilderGetSLPVectorize(
        builder: LLVMPassManagerBuilderRef,
    ) -> i32;
    
    
    pub fn LLVMPY_FunctionAttributesIter(
        value: LLVMValueRef
    ) -> LLVMAttributeListIteratorRef;

    pub fn LLVMPY_ArgumentAttributesIter(
        value: LLVMValueRef,
    ) -> LLVMAttributeSetIteratorRef;

    pub fn LLVMPY_CallInstAttributesIter(
        value: LLVMValueRef,
    ) -> LLVMAttributeListIteratorRef;

    pub fn LLVMPY_InvokeInstAttributesIter(
        value: LLVMValueRef,
    ) -> LLVMAttributeListIteratorRef;

    pub fn LLVMPY_GlobalAttributesIter(
        value: LLVMValueRef,
    ) -> LLVMAttributeSetIteratorRef;

    pub fn LLVMPY_FunctionBlocksIter(
        value: LLVMValueRef,
    ) -> LLVMBlocksIteratorRef;

    pub fn LLVMPY_FunctionArgumentsIter(
        value: LLVMValueRef,
    ) -> LLVMArgumentsIteratorRef;

    pub fn LLVMPY_BlockInstructionsIter(
        value: LLVMValueRef,
    ) -> LLVMInstructionsIteratorRef;

    pub fn LLVMPY_InstructionOperandsIter(
        value: LLVMValueRef,
    ) -> LLVMOperandsIteratorRef;

    pub fn LLVMPY_AttributeListIterNext(
        iter: LLVMAttributeListIteratorRef,
    ) -> *const c_char;

    pub fn LLVMPY_BlocksIterNext(
        iter: LLVMBlocksIteratorRef,
    ) -> LLVMValueRef;

    pub fn LLVMPY_ArgumentsIterNext(
        iter: LLVMArgumentsIteratorRef,
    ) -> LLVMValueRef;

    pub fn LLVMPY_InstructionsIterNext(
        iter: LLVMInstructionsIteratorRef
    ) -> LLVMValueRef;

    pub fn LLVMPY_OperandsIterNext(
        iter: LLVMOperandsIteratorRef,
    ) -> LLVMValueRef;

    pub fn LLVMPY_DisposeAttributeListIter(iter: LLVMAttributeListIteratorRef);

    pub fn LLVMPY_DisposeAttributeSetIter(iter: LLVMAttributeSetIteratorRef);

    pub fn LLVMPY_DisposeBlocksIter(iter: LLVMBlocksIteratorRef);

    pub fn LLVMPY_DisposeArgumentsIter(iter: LLVMArgumentsIteratorRef);

    pub fn LLVMPY_DisposeInstructionsIter(iter: LLVMInstructionsIteratorRef);

    pub fn LLVMPY_DisposeOperandsIter(iter: LLVMOperandsIteratorRef);

    pub fn LLVMPY_PrintValueToString(val: LLVMValueRef, out: *mut *const c_char);

    pub fn LLVMPY_GetValueName(val: LLVMValueRef) -> *const c_char;

    pub fn LLVMPY_SetValueName(val: LLVMValueRef, name: *const c_char);

    pub fn LLVMPY_GetGlobalParent(val: LLVMValueRef) -> LLVMModuleRef;

    pub fn LLVMPY_TypeOf(val: LLVMValueRef) -> LLVMTypeRef;

    pub fn LLVMPY_PrintType(ty: LLVMTypeRef) -> *const c_char;

    pub fn LLVMPY_GetTypeName(ty: LLVMTypeRef) -> *const c_char;

    pub fn LLVMPY_TypeIsPointer(ty: LLVMTypeRef) -> bool;

    pub fn LLVMPY_GetElementType(ty: LLVMTypeRef) -> LLVMTypeRef;

    pub fn LLVMPY_SetLinkage(ty: LLVMTypeRef, linkage: i32);

    pub fn LLVMPY_GetLinkage(ty: LLVMTypeRef) -> i32;

    pub fn LLVMPY_SetVisibility(ty: LLVMTypeRef, visibility: i32);

    pub fn LLVMPY_GetVisibility(ty: LLVMTypeRef) -> i32;

    pub fn LLVMPY_SetDLLStorageClass(ty: LLVMTypeRef, class: i32);

    pub fn LLVMPY_GetDLLStorageClass(ty: LLVMTypeRef) -> i32;

    pub fn LLVMPY_GetEnumAttributeKindForName(name: *const c_char, len: usize) -> usize;

    pub fn LLVMPY_AddFunctionAttr(val: LLVMValueRef, kind: usize);

    pub fn LLVMPY_IsDeclaration(val: LLVMValueRef) -> i32;

    pub fn LLVMPY_WriteCFG(val: LLVMValueRef, out_str: *mut *const c_char, show_inst: i32);

    pub fn LLVMPY_GetOpcodeName(val: LLVMValueRef) -> *const c_char;
}