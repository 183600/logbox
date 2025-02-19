use windows::Win32::Foundation::HCURSOR;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetCursorPos, GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW,
    HHOOK, HHOOKPROC, KBDLLHOOKSTRUCT, MSLLHOOKSTRUCT, PeekMessageA, PM_NOREMOVE, PM_REMOVE,
    SetWindowsHookExA, UnhookWindowsHookEx, WH_KEYBOARD_LL,
};