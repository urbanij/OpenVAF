use std::ffi::CString;
use std::fmt;

use libc::{c_char, c_int};

use crate::{Error, Module, OptLevel, PassBuilderOptions, TargetMachine};

extern "C" {
    // New pass manager API (replaces the old PassManagerBuilder)
    pub fn LLVMCreatePassBuilderOptions() -> &'static mut PassBuilderOptions;
    pub fn LLVMDisposePassBuilderOptions(Options: &'static mut PassBuilderOptions);
    pub fn LLVMPassBuilderOptionsSetSLPVectorization(
        Options: &PassBuilderOptions,
        SLPVectorization: crate::Bool,
    );
    pub fn LLVMPassBuilderOptionsSetLoopUnrolling(
        Options: &PassBuilderOptions,
        LoopUnrolling: crate::Bool,
    );
    pub fn LLVMPassBuilderOptionsSetInlinerThreshold(Options: &PassBuilderOptions, Threshold: c_int);

    /// Run passes on a module. Returns null on success, or an Error on failure.
    pub fn LLVMRunPasses(
        M: &Module,
        Passes: *const c_char,
        TM: Option<&TargetMachine>,
        Options: &PassBuilderOptions,
    ) -> Option<&'static mut Error>;

    // Error handling
    pub fn LLVMConsumeError(Err: &'static mut Error);
    pub fn LLVMGetErrorMessage(Err: &'static mut Error) -> *mut c_char;
    pub fn LLVMDisposeErrorMessage(ErrMsg: *mut c_char);
}

/// Map an OptLevel to the new pass manager pipeline string.
fn opt_level_pipeline(opt_lvl: OptLevel) -> &'static str {
    match opt_lvl {
        OptLevel::None => "default<O0>",
        OptLevel::Less => "default<O1>",
        OptLevel::Default => "default<O2>",
        OptLevel::Aggressive => "default<O3>",
    }
}

/// # Safety
/// This function calls the LLVM C API. The module must be valid.
pub unsafe fn run_passes(
    module: &Module,
    opt_lvl: OptLevel,
    target_machine: Option<&TargetMachine>,
) -> Result<(), String> {
    let options = LLVMCreatePassBuilderOptions();
    if opt_lvl > OptLevel::Less {
        LLVMPassBuilderOptionsSetSLPVectorization(options, crate::True);
    }
    LLVMPassBuilderOptionsSetLoopUnrolling(options, crate::True);

    let pipeline = CString::new(opt_level_pipeline(opt_lvl)).unwrap();
    let err = LLVMRunPasses(module, pipeline.as_ptr(), target_machine, options);
    LLVMDisposePassBuilderOptions(options);

    if let Some(err) = err {
        let msg_ptr = LLVMGetErrorMessage(err);
        let msg = if msg_ptr.is_null() {
            "unknown LLVM error".to_string()
        } else {
            let msg = std::ffi::CStr::from_ptr(msg_ptr).to_string_lossy().into_owned();
            LLVMDisposeErrorMessage(msg_ptr);
            msg
        };
        Err(msg)
    } else {
        Ok(())
    }
}

// Re-export for compatibility
pub use crate::module::function_iter;

/// Wrapper for displaying pass errors
pub struct PassError(pub String);

impl fmt::Display for PassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LLVM pass error: {}", self.0)
    }
}
