use std::convert::TryInto;
use std::ffi::c_void;
use std::fmt::Debug;
use std::mem::{self, MaybeUninit, size_of};
use std::process::exit;
use std::ptr::{self, null, null_mut};

use bindings::Windows::Win32;
use bindings::Windows::Win32::Foundation::{CloseHandle, HINSTANCE, PWSTR};
use bindings::Windows::Win32::System::Diagnostics::Debug::{DebugActiveProcessStop, GetLastError};
use bindings::Windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use bindings::Windows::Win32::System::Memory::SEC_PARTITION_OWNER_HANDLE;
use bindings::Windows::Win32::System::Threading::{CreateProcessW, DEBUG_ONLY_THIS_PROCESS, OpenThread, STARTF_USESHOWWINDOW, STARTUPINFOW};
use windows::Result;

const MAX_PATH : usize = 260;
struct WinReplContext {
    pinfo: Win32::System::Threading::PROCESS_INFORMATION,
    lp_start_addr: c_void,
    nmemsize : usize,
    prev : Win32::System::Diagnostics::Debug::CONTEXT,
    curr : Win32::System::Diagnostics::Debug::CONTEXT,
}

impl WinReplContext {
    pub fn new() -> Self {
        unimplemented!();
    }

    fn create_debuggee(&mut self) -> Result<()> {
        let mut buf: [u16; MAX_PATH];
        buf = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        unsafe{ GetModuleFileNameW(HINSTANCE::NULL, PWSTR(buf.as_mut_ptr()), MAX_PATH as u32) };

        let mut si = STARTUPINFOW::default();
        si.dwFlags = STARTF_USESHOWWINDOW;
        si.wShowWindow = 0;
        si.cb = size_of::<STARTUPINFOW>() as u32;
        
        let success = unsafe { CreateProcessW(PWSTR(buf.as_mut_ptr()), 
            PWSTR::NULL,
            ptr::null_mut(),
            ptr::null_mut(),
            false, 
            DEBUG_ONLY_THIS_PROCESS, 
            ptr::null_mut(), PWSTR::NULL,&mut si , &mut self.pinfo)};
        if !success.as_bool()  {
            panic!("{:?}", unsafe {GetLastError()});
        }

        unsafe {CloseHandle(self.pinfo.hThread);} //workaround for a bug on startup (Windows 8.1 x64), SetThreadContext would fail for some reason
        Ok(())
    }

    pub fn start_repl_loop(&self) -> bool{
        unimplemented!()
    }

    pub fn eval(&self) -> bool {
        unimplemented!()
    }

    pub fn write_shellcode(&self, encode: &[u8], size: usize) -> bool{
        unimplemented!()
    }

    pub fn debug_shellcode(&self) {
        unimplemented!()
    }

    pub fn print_pid(&self) {
        unimplemented!()
    }

    pub fn print_register(&self) {

    }

    pub fn print_register_all(&self) {

    }

    pub fn print_assembly(encode: &[u8]) {

    }

    pub fn print_bytes(addr: &[u8], len: u32, start_addr: u64) {

    }
}

impl Drop for WinReplContext {
    fn drop(&mut self) {
        unsafe{ DebugActiveProcessStop(self.pinfo.dwProcessId); }
    }
}