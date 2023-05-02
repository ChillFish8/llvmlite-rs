use std::ffi::OsStr;
use once_cell::sync::OnceCell;

static LLVMLITE: OnceCell<libloading::Library> = OnceCell::new();


/// Attempts to init the LLVMLite DLL.
pub unsafe fn try_init(path: impl AsRef<OsStr>) -> Result<(), libloading::Error> {
    if LLVMLITE.get().is_some() {
        return Ok(())
    }

    let path = libloading::library_filename(path.as_ref());
    let lib = libloading::Library::new(path)?;

    LLVMLITE.set(lib).unwrap();

    Ok(())
}


