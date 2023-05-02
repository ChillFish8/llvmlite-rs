use llvmlite_sys::{LLVMPY_ContextCreate, LLVMPY_ContextDispose};

#[test]
pub fn test_works() -> anyhow::Result<()> {


    let ctx = unsafe { LLVMPY_ContextCreate() };

    unsafe {
        LLVMPY_ContextDispose(ctx)
    }

    Ok(())
}