fn main() {
    windows::build! {
        Windows::Win32::System::Threading::{CreateProcessW, OpenThread},
        Windows::Win32::System::Diagnostics::Debug::{WaitForDebugEvent, ContinueDebugEvent, GetThreadContext, SetThreadContext, DebugActiveProcessStop, GetLastError},
        Windows::Win32::System::LibraryLoader::GetModuleFileNameW,
        Windows::Win32::Foundation::{PWSTR, PSTR, HINSTANCE, BOOL, HANDLE},
        Windows::Win32::Foundation::{CloseHandle},
        Windows::Win32::Security::SECURITY_ATTRIBUTES,
        Windows::Win32::UI::WindowsAndMessaging::MessageBoxA,
        Windows::Win32::System::Memory::{VirtualAllocEx, VIRTUAL_ALLOCATION_TYPE, PAGE_PROTECTION_FLAGS},
    }
}