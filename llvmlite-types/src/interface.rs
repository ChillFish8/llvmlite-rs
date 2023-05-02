#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_void};

use crate::types::*;

pub type LLVMPY_SetCommandLine = unsafe extern "C" fn(*const c_char, *const c_char);
pub type LLVMPY_ParseAssembly = unsafe extern "C" fn(
    LLVMContextRef,
    *const c_char,
    *mut *const c_char,
) -> LLVMContextRef;
pub type LLVMPY_WriteBitcodeToString =
    unsafe extern "C" fn(LLVMContextRef, *mut *const c_char, *mut usize);
pub type LLVMPY_ParseBitcode = unsafe extern "C" fn(
    LLVMContextRef,
    *const c_char,
    usize,
    *mut *const c_char,
) -> LLVMModuleRef;
pub type LLVMPY_CreateString = unsafe extern "C" fn(*const c_char) -> *const c_char;
pub type LLVMPY_CreateByteString =
    unsafe extern "C" fn(*const c_char, usize) -> *const c_char;
pub type LLVMPY_DisposeString = unsafe extern "C" fn(*const c_char);
pub type LLVMPY_GetGlobalContext = unsafe extern "C" fn() -> LLVMContextRef;
pub type LLVMPY_ContextCreate = unsafe extern "C" fn() -> LLVMContextRef;
pub type LLVMPY_ContextDispose = unsafe extern "C" fn(LLVMContextRef);
pub type LLVMPY_AddRefPrunePass = unsafe extern "C" fn(LLVMPassManagerRef, i32, usize);
pub type LLVMPY_DumpRefPruneStats = unsafe extern "C" fn(*mut PruneStats, bool);
pub type LLVMPY_SearchAddressOfSymbol =
    unsafe extern "C" fn(*const c_char) -> *const c_void;
pub type LLVMPY_AddSymbol = unsafe extern "C" fn(*const c_char, *const c_void);
pub type LLVMPY_LoadLibraryPermanently =
    unsafe extern "C" fn(*const c_char, *mut *const c_char) -> bool;
pub type LLVMPY_LinkInMCJIT = unsafe extern "C" fn();
pub type LLVMPY_DisposeExecutionEngine = unsafe extern "C" fn(LLVMExecutionEngineRef);
pub type LLVMPY_AddModule = unsafe extern "C" fn(LLVMExecutionEngineRef, LLVMModuleRef);
pub type LLVMPY_RemoveModule =
    unsafe extern "C" fn(LLVMExecutionEngineRef, LLVMModuleRef, *mut *mut c_char) -> i32;
pub type LLVMPY_FinalizeObject = unsafe extern "C" fn(LLVMExecutionEngineRef);
pub type LLVMPY_CreateMCJITCompiler = unsafe extern "C" fn(
    LLVMExecutionEngineRef,
    LLVMTargetMachineRef,
    *mut *const c_char,
) -> LLVMExecutionEngineRef;
pub type LLVMPY_GetGlobalValueAddress =
    unsafe extern "C" fn(LLVMExecutionEngineRef, *const c_char) -> u64;
pub type LLVMPY_GetFunctionAddress =
    unsafe extern "C" fn(LLVMExecutionEngineRef, *const c_char) -> u64;
pub type LLVMPY_RunStaticConstructors = unsafe extern "C" fn(LLVMExecutionEngineRef);
pub type LLVMPY_RunStaticDestructors = unsafe extern "C" fn(LLVMExecutionEngineRef);
pub type LLVMPY_AddGlobalMapping = unsafe extern "C" fn(LLVMExecutionEngineRef);
pub type LLVMPY_GetExecutionEngineTargetData =
    unsafe extern "C" fn(LLVMExecutionEngineRef) -> LLVMTargetDataRef;
pub type LLVMPY_TryAllocateExecutableMemory = unsafe extern "C" fn() -> i32;
pub type LLVMPY_EnableJITEvents = unsafe extern "C" fn(LLVMExecutionEngineRef) -> bool;
pub type LLVMPY_MCJITAddObjectFile =
    unsafe extern "C" fn(LLVMExecutionEngineRef, LLVMObjectFileRef);
pub type LLVMPY_CreateObjectCache = unsafe extern "C" fn(
    ObjectCacheNotifyFunc,
    ObjectCacheGetObjectFunc,
    *mut c_void,
) -> LLVMObjectCacheRef;
pub type LLVMPY_DisposeObjectCache = unsafe extern "C" fn(LLVMObjectCacheRef);
pub type LLVMPY_SetObjectCache =
    unsafe extern "C" fn(LLVMExecutionEngineRef, LLVMObjectCacheRef);
pub type LLVMPY_Shutdown = unsafe extern "C" fn();
pub type LLVMPY_GetVersionInfo = unsafe extern "C" fn() -> u32;
pub type LLVMPY_LinkModules =
    unsafe extern "C" fn(LLVMModuleRef, LLVMModuleRef, *mut *const c_char) -> i32;
pub type LLVMPY_DisposeModule = unsafe extern "C" fn(LLVMModuleRef);
pub type LLVMPY_PrintModuleToString =
    unsafe extern "C" fn(LLVMModuleRef, *mut *const c_char);
pub type LLVMPY_GetModuleSourceFileName =
    unsafe extern "C" fn(LLVMModuleRef) -> *const c_char;
pub type LLVMPY_GetModuleName = unsafe extern "C" fn(LLVMModuleRef) -> *const c_char;
pub type LLVMPY_SetModuleName = unsafe extern "C" fn(LLVMModuleRef, *const c_char);
pub type LLVMPY_GetNamedFunction =
    unsafe extern "C" fn(LLVMModuleRef, *const c_char) -> LLVMValueRef;
pub type LLVMPY_GetNamedGlobalVariable =
    unsafe extern "C" fn(LLVMModuleRef, *const c_char) -> LLVMValueRef;
pub type LLVMPY_GetNamedStructType =
    unsafe extern "C" fn(LLVMModuleRef, *const c_char) -> LLVMTypeRef;
pub type LLVMPY_VerifyModule =
    unsafe extern "C" fn(LLVMModuleRef, *mut *const c_char) -> i32;
pub type LLVMPY_GetDataLayout = unsafe extern "C" fn(LLVMModuleRef, *mut *const c_char);
pub type LLVMPY_SetDataLayout = unsafe extern "C" fn(LLVMModuleRef, *const c_char);
pub type LLVMPY_GetTarget = unsafe extern "C" fn(LLVMModuleRef, *mut *const c_char);
pub type LLVMPY_SetTarget = unsafe extern "C" fn(LLVMModuleRef, *const c_char);
pub type LLVMPY_ModuleGlobalsIter =
    unsafe extern "C" fn(LLVMModuleRef) -> LLVMGlobalsIteratorRef;
pub type LLVMPY_ModuleFunctionsIter =
    unsafe extern "C" fn(LLVMModuleRef) -> LLVMFunctionsIteratorRef;
pub type LLVMPY_ModuleTypesIter =
    unsafe extern "C" fn(LLVMModuleRef) -> LLVMTypesIteratorRef;
pub type LLVMPY_GlobalsIterNext =
    unsafe extern "C" fn(LLVMGlobalsIteratorRef) -> LLVMValueRef;
pub type LLVMPY_FunctionsIterNext =
    unsafe extern "C" fn(LLVMFunctionsIteratorRef) -> LLVMValueRef;
pub type LLVMPY_TypesIterNext =
    unsafe extern "C" fn(LLVMTypesIteratorRef) -> LLVMTypeRef;
pub type LLVMPY_DisposeGlobalsIter = unsafe extern "C" fn(LLVMGlobalsIteratorRef);
pub type LLVMPY_DisposeFunctionsIter = unsafe extern "C" fn(LLVMFunctionsIteratorRef);
pub type LLVMPY_DisposeTypesIter = unsafe extern "C" fn(LLVMTypesIteratorRef);
pub type LLVMPY_CloneModule = unsafe extern "C" fn(LLVMModuleRef) -> LLVMModuleRef;
pub type LLVMPY_CreateObjectFile =
    unsafe extern "C" fn(*const c_char, usize) -> LLVMObjectFileRef;
pub type LLVMPY_DisposeObjectFile = unsafe extern "C" fn(LLVMObjectFileRef);
pub type LLVMPY_GetSections =
    unsafe extern "C" fn(LLVMObjectFileRef) -> LLVMSectionIteratorRef;
pub type LLVMPY_DisposeSectionIterator = unsafe extern "C" fn(LLVMSectionIteratorRef);
pub type LLVMPY_MoveToNextSection = unsafe extern "C" fn(LLVMSectionIteratorRef);
pub type LLVMPY_IsSectionIteratorAtEnd =
    unsafe extern "C" fn(LLVMObjectFileRef, LLVMSectionIteratorRef) -> bool;
pub type LLVMPY_GetSectionName =
    unsafe extern "C" fn(LLVMSectionIteratorRef) -> *const c_char;
pub type LLVMPY_GetSectionAddress = unsafe extern "C" fn(LLVMSectionIteratorRef) -> u64;
pub type LLVMPY_GetSectionContents =
    unsafe extern "C" fn(LLVMSectionIteratorRef) -> *const c_char;
pub type LLVMPY_GetSectionSize = unsafe extern "C" fn(LLVMSectionIteratorRef) -> u64;
pub type LLVMPY_IsSectionText = unsafe extern "C" fn(LLVMSectionIteratorRef) -> bool;
pub type LLVMPY_SetTimePasses = unsafe extern "C" fn(bool);
pub type LLVMPY_ReportAndResetTimings = unsafe extern "C" fn(*mut *const c_char);
pub type LLVMPY_CreatePassManager = unsafe extern "C" fn() -> LLVMPassManagerRef;
pub type LLVMPY_DisposePassManager = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_CreateFunctionPassManager =
    unsafe extern "C" fn(LLVMModuleRef) -> LLVMPassManagerRef;
pub type LLVMPY_RunPassManagerWithRemarks = unsafe extern "C" fn(
    LLVMPassManagerRef,
    LLVMModuleRef,
    *const c_char,
    *const c_char,
    *const c_char,
) -> i32;
pub type LLVMPY_RunPassManager =
    unsafe extern "C" fn(LLVMPassManagerRef, LLVMModuleRef) -> i32;
pub type LLVMPY_RunFunctionPassManagerWithRemarks = unsafe extern "C" fn(
    LLVMPassManagerRef,
    LLVMValueRef,
    *const c_char,
    *const c_char,
    *const c_char,
) -> i32;
pub type LLVMPY_RunFunctionPassManager =
    unsafe extern "C" fn(LLVMPassManagerRef, LLVMValueRef) -> i32;
pub type LLVMPY_InitializeFunctionPassManager =
    unsafe extern "C" fn(LLVMPassManagerRef) -> i32;
pub type LLVMPY_FinalizeFunctionPassManager =
    unsafe extern "C" fn(LLVMPassManagerRef) -> i32;
pub type LLVMPY_AddAAEvalPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddBasicAAWrapperPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddDependenceAnalysisPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddCallGraphDOTPrinterPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddDotDomPrinterPass = unsafe extern "C" fn(LLVMPassManagerRef, bool);
pub type LLVMPY_AddGlobalsModRefAAPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddDotPostDomPrinterPass =
    unsafe extern "C" fn(LLVMPassManagerRef, bool);
pub type LLVMPY_AddCFGPrinterPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddConstantMergePass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddDeadStoreEliminationPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddReversePostOrderFunctionAttrsPass =
    unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddDeadArgEliminationPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddInstructionCountPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddIVUsersPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddLazyValueInfoPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddLintPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddModuleDebugInfoPrinterPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddRegionInfoPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddScalarEvolutionAAPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddAggressiveDCEPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddAlwaysInlinerPass = unsafe extern "C" fn(LLVMPassManagerRef, bool);
pub type LLVMPY_AddArgPromotionPass = unsafe extern "C" fn(LLVMPassManagerRef, u32);
pub type LLVMPY_AddBreakCriticalEdgesPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddFunctionAttrsPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddFunctionInliningPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddGlobalOptimizerPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddGlobalDCEPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddIPSCCPPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddDeadCodeEliminationPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddAggressiveInstructionCombiningPass =
    unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddInternalizePass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddJumpThreadingPass = unsafe extern "C" fn(LLVMPassManagerRef, i32);
pub type LLVMPY_AddLCSSAPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddLoopDeletionPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddSingleLoopExtractorPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddLoopStrengthReducePass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddLoopSimplificationPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddLoopUnrollPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddLoopUnrollAndJamPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddLoopUnswitchPass =
    unsafe extern "C" fn(LLVMPassManagerRef, bool, bool);
pub type LLVMPY_AddLowerAtomicPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddLowerInvokePass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddLowerSwitchPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddMemCpyOptimizationPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddMergeFunctionsPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddMergeReturnsPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddPartialInliningPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddPruneExceptionHandlingPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddReassociatePass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddDemoteRegisterToMemoryPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddSinkPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddStripSymbolsPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddStripDeadDebugInfoPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddStripDeadPrototypesPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddStripDebugDeclarePrototypesPass =
    unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddStripNondebugSymbolsPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddTailCallEliminationPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddCFGSimplificationPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddGVNPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddInstructionCombiningPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddLICMPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddSCCPPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddSROAPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddTypeBasedAliasAnalysisPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_AddBasicAliasAnalysisPass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_LLVMAddLoopRotatePass = unsafe extern "C" fn(LLVMPassManagerRef);
pub type LLVMPY_GetProcessTriple = unsafe extern "C" fn(*mut *const c_char);
pub type LLVMPY_GetHostCPUFeatures = unsafe extern "C" fn(*mut *const c_char) -> i32;
pub type LLVMPY_GetDefaultTargetTriple = unsafe extern "C" fn(*mut *const c_char);
pub type LLVMPY_GetHostCPUName = unsafe extern "C" fn(*mut *const c_char);
pub type LLVMPY_GetTripleObjectFormat = unsafe extern "C" fn(*mut *const c_char) -> i32;
pub type LLVMPY_CreateTargetData =
    unsafe extern "C" fn(*mut *const c_char) -> LLVMTargetDataRef;
pub type LLVMPY_CopyStringRepOfTargetData =
    unsafe extern "C" fn(LLVMTargetDataRef, *mut *mut c_char);
pub type LLVMPY_DisposeTargetData = unsafe extern "C" fn(LLVMTargetDataRef);
pub type LLVMPY_ABISizeOfType =
    unsafe extern "C" fn(LLVMTargetDataRef, LLVMTypeRef) -> i64;
pub type LLVMPY_OffsetOfElement = unsafe extern "C" fn(LLVMTargetDataRef, i32) -> i64;
pub type LLVMPY_ABISizeOfElementType =
    unsafe extern "C" fn(LLVMTargetDataRef, LLVMTypeRef) -> i64;
pub type LLVMPY_ABIAlignmentOfElementType =
    unsafe extern "C" fn(LLVMTargetDataRef, LLVMTypeRef) -> i64;
pub type LLVMPY_GetTargetFromTriple =
    unsafe extern "C" fn(*mut *const c_char, *mut *const c_char) -> LLVMTargetRef;
pub type LLVMPY_GetTargetName = unsafe extern "C" fn(LLVMTargetRef) -> *const c_char;
pub type LLVMPY_GetTargetDescription =
    unsafe extern "C" fn(LLVMTargetRef) -> *const c_char;
pub type LLVMPY_CreateTargetMachine = unsafe extern "C" fn(
    LLVMTargetRef,
    *const c_char,
    *const c_char,
    *const c_char,
    i32,
    *const c_char,
    *const c_char,
    i32,
    i32,
    *const c_char,
) -> LLVMTargetMachineRef;
pub type LLVMPY_DisposeTargetMachine = unsafe extern "C" fn(LLVMTargetMachineRef);
pub type LLVMPY_GetTargetMachineTriple =
    unsafe extern "C" fn(LLVMTargetMachineRef, *mut *const c_char);
pub type LLVMPY_SetTargetMachineAsmVerbosity =
    unsafe extern "C" fn(LLVMTargetMachineRef, i32);
pub type LLVMPY_TargetMachineEmitToMemory = unsafe extern "C" fn(
    LLVMTargetMachineRef,
    LLVMModuleRef,
    i32,
    *mut *const c_char,
) -> LLVMMemoryBufferRef;
pub type LLVMPY_CreateTargetMachineData =
    unsafe extern "C" fn(LLVMTargetMachineRef) -> LLVMTargetDataRef;
pub type LLVMPY_AddAnalysisPasses =
    unsafe extern "C" fn(LLVMTargetMachineRef, LLVMPassManagerRef);
pub type LLVMPY_GetBufferStart =
    unsafe extern "C" fn(LLVMMemoryBufferRef) -> *const c_void;
pub type LLVMPY_GetBufferSize = unsafe extern "C" fn(LLVMMemoryBufferRef) -> usize; // TODO: may need to be isize for size_t;
pub type LLVMPY_DisposeMemoryBuffer = unsafe extern "C" fn(LLVMMemoryBufferRef);
pub type LLVMPY_HasSVMLSupport = unsafe extern "C" fn() -> i32;
pub type LLVMPY_PassManagerBuilderCreate =
    unsafe extern "C" fn() -> LLVMPassManagerBuilderRef;
pub type LLVMPY_PassManagerBuilderDispose =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef);
pub type LLVMPY_PassManagerBuilderGetOptLevel =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef) -> usize;
pub type LLVMPY_PassManagerBuilderSetOptLevel =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef, usize);
pub type LLVMPY_PassManagerBuilderGetSizeLevel =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef) -> usize;
pub type LLVMPY_PassManagerBuilderSetSizeLevel =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef, usize);
pub type LLVMPY_PassManagerBuilderGetDisableUnrollLoops =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef) -> i32;
pub type LLVMPY_PassManagerBuilderSetDisableUnrollLoops =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef, LLVMBool);
pub type LLVMPY_PassManagerBuilderUseInlinerWithThreshold =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef, usize);
pub type LLVMPY_PassManagerBuilderPopulateFunctionPassManager =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef, LLVMPassManagerRef);
pub type LLVMPY_PassManagerBuilderSetLoopVectorize =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef, i32);
pub type LLVMPY_PassManagerBuilderGetLoopVectorize =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef) -> i32;
pub type LLVMPY_PassManagerBuilderSetSLPVectorize =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef, i32);
pub type LLVMPY_PassManagerBuilderGetSLPVectorize =
    unsafe extern "C" fn(LLVMPassManagerBuilderRef) -> i32;
pub type LLVMPY_FunctionAttributesIter =
    unsafe extern "C" fn(LLVMValueRef) -> LLVMAttributeListIteratorRef;
pub type LLVMPY_ArgumentAttributesIter =
    unsafe extern "C" fn(LLVMValueRef) -> LLVMAttributeSetIteratorRef;
pub type LLVMPY_CallInstAttributesIter =
    unsafe extern "C" fn(LLVMValueRef) -> LLVMAttributeListIteratorRef;
pub type LLVMPY_InvokeInstAttributesIter =
    unsafe extern "C" fn(LLVMValueRef) -> LLVMAttributeListIteratorRef;
pub type LLVMPY_GlobalAttributesIter =
    unsafe extern "C" fn(LLVMValueRef) -> LLVMAttributeSetIteratorRef;
pub type LLVMPY_FunctionBlocksIter =
    unsafe extern "C" fn(LLVMValueRef) -> LLVMBlocksIteratorRef;
pub type LLVMPY_FunctionArgumentsIter =
    unsafe extern "C" fn(LLVMValueRef) -> LLVMArgumentsIteratorRef;
pub type LLVMPY_BlockInstructionsIter =
    unsafe extern "C" fn(LLVMValueRef) -> LLVMInstructionsIteratorRef;
pub type LLVMPY_InstructionOperandsIter =
    unsafe extern "C" fn(LLVMValueRef) -> LLVMOperandsIteratorRef;
pub type LLVMPY_AttributeListIterNext =
    unsafe extern "C" fn(LLVMAttributeListIteratorRef) -> *const c_char;
pub type LLVMPY_BlocksIterNext =
    unsafe extern "C" fn(LLVMBlocksIteratorRef) -> LLVMValueRef;
pub type LLVMPY_ArgumentsIterNext =
    unsafe extern "C" fn(LLVMArgumentsIteratorRef) -> LLVMValueRef;
pub type LLVMPY_InstructionsIterNext =
    unsafe extern "C" fn(LLVMInstructionsIteratorRef) -> LLVMValueRef;
pub type LLVMPY_OperandsIterNext =
    unsafe extern "C" fn(LLVMOperandsIteratorRef) -> LLVMValueRef;
pub type LLVMPY_DisposeAttributeListIter =
    unsafe extern "C" fn(LLVMAttributeListIteratorRef);
pub type LLVMPY_DisposeAttributeSetIter =
    unsafe extern "C" fn(LLVMAttributeSetIteratorRef);
pub type LLVMPY_DisposeBlocksIter = unsafe extern "C" fn(LLVMBlocksIteratorRef);
pub type LLVMPY_DisposeArgumentsIter = unsafe extern "C" fn(LLVMArgumentsIteratorRef);
pub type LLVMPY_DisposeInstructionsIter =
    unsafe extern "C" fn(LLVMInstructionsIteratorRef);
pub type LLVMPY_DisposeOperandsIter = unsafe extern "C" fn(LLVMOperandsIteratorRef);
pub type LLVMPY_PrintValueToString =
    unsafe extern "C" fn(LLVMValueRef, *mut *const c_char);
pub type LLVMPY_GetValueName = unsafe extern "C" fn(LLVMValueRef) -> *const c_char;
pub type LLVMPY_SetValueName = unsafe extern "C" fn(LLVMValueRef, *const c_char);
pub type LLVMPY_GetGlobalParent = unsafe extern "C" fn(LLVMValueRef) -> LLVMModuleRef;
pub type LLVMPY_TypeOf = unsafe extern "C" fn(LLVMValueRef) -> LLVMTypeRef;
pub type LLVMPY_PrintType = unsafe extern "C" fn(LLVMTypeRef) -> *const c_char;
pub type LLVMPY_GetTypeName = unsafe extern "C" fn(LLVMTypeRef) -> *const c_char;
pub type LLVMPY_TypeIsPointer = unsafe extern "C" fn(LLVMTypeRef) -> bool;
pub type LLVMPY_GetElementType = unsafe extern "C" fn(LLVMTypeRef) -> LLVMTypeRef;
pub type LLVMPY_SetLinkage = unsafe extern "C" fn(LLVMTypeRef, i32);
pub type LLVMPY_GetLinkage = unsafe extern "C" fn(LLVMTypeRef) -> i32;
pub type LLVMPY_SetVisibility = unsafe extern "C" fn(LLVMTypeRef, i32);
pub type LLVMPY_GetVisibility = unsafe extern "C" fn(LLVMTypeRef) -> i32;
pub type LLVMPY_SetDLLStorageClass = unsafe extern "C" fn(LLVMTypeRef, i32);
pub type LLVMPY_GetDLLStorageClass = unsafe extern "C" fn(LLVMTypeRef) -> i32;
pub type LLVMPY_GetEnumAttributeKindForName =
    unsafe extern "C" fn(*const c_char, usize) -> usize;
pub type LLVMPY_AddFunctionAttr = unsafe extern "C" fn(LLVMValueRef, usize);
pub type LLVMPY_IsDeclaration = unsafe extern "C" fn(LLVMValueRef) -> i32;
pub type LLVMPY_WriteCFG = unsafe extern "C" fn(LLVMValueRef, *mut *const c_char, i32);
pub type LLVMPY_GetOpcodeName = unsafe extern "C" fn(LLVMValueRef) -> *const c_char;
