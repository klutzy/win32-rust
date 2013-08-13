use ll::platform::*;
use ll::windef::*;

// extern "stdcall" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT
pub type WNDPROC = *u8;

pub struct SECURITY_ATTRIBUTES {
    nLength: DWORD,
    lpSecurityDescriptor: LPVOID,
    bInheritHandle: BOOL,
}

pub struct PROCESS_INFORMATION {
    hProcess: HANDLE,
    hTread: HANDLE,
    dwProcessId: DWORD,
    dwThreadId: DWORD,
}

pub struct STARTUPINFO {
    cb: DWORD,
    lpReserved: LPWSTR,
    lpDesktop: LPWSTR,
    lpTitle: LPWSTR,
    dwX: DWORD,
    dwY: DWORD,
    dwXSize: DWORD,
    dwYSize: DWORD,
    dwXCountChars: DWORD,
    dwYCountChars: DWORD,
    dwFillAttribute: DWORD,
    dwFlags: DWORD,
    wShowWindow: WORD,
    cbReserved2: WORD,
    lpReserved2: LPBYTE,
    hStdInput: HANDLE,
    hStdOutput: HANDLE,
    hStdError: HANDLE,
}

pub struct WNDCLASSEX {
    cbSize: UINT,
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
    hIconSm: HICON,
}

pub struct CREATESTRUCT {
    lpCreateParams: LPVOID,
    hInstance: HINSTANCE,
    hMenu: HMENU,
    hwndParent: HWND,
    cy: c_int,
    cx: c_int,
    y: c_int,
    x: c_int,
    style: LONG,
    lpszName: LPCWSTR,
    lpszClass: LPCWSTR,
    dwExStyle: DWORD,
}

pub struct POINT {
    x: LONG,
    y: LONG,
}

pub struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
}

pub struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}

pub struct PAINTSTRUCT {
    hdc: HDC,
    fErase: BOOL,
    rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserved: *[BYTE, ..32],
}

pub mod kernel32 {
    use ll::*;
    extern "stdcall" {
        unsafe fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

        unsafe fn CreateProcessW(
            lpApplicationName: LPCWSTR, lpCommandLine: LPWSTR,
            lpProcessAttributes: *SECURITY_ATTRIBUTES,
            lpThreadAttributes: *SECURITY_ATTRIBUTES,
            bInheritHandles: BOOL,
            dwCreationFlags: DWORD,
            lpEnvironment: LPVOID,
            lpCurrentDirectory: LPCWSTR,
            lpStartupInfo: LPSTARTUPINFO,
            lpProcessInformation: LPPROCESS_INFORMATION
        ) -> BOOL;
    }
}

pub mod user32 {
    use ll::*;
    extern "stdcall" {
        unsafe fn CreateWindowExW(extrastyle: DWORD, classname: LPCWSTR,
                windowname: LPCWSTR, style: DWORD,
                x: c_int, y: c_int, width: c_int, height: c_int,
                parent: HWND, menu: HMENU, instance: HINSTANCE, param: LPVOID
        ) -> HWND;

        unsafe fn ShowWindow(hwnd: HWND, nCmdShow: c_int) -> BOOL;

        unsafe fn UpdateWindow(hwnd: HWND) -> BOOL;

        unsafe fn BeginPaint(hwnd: HWND, lpPaint: *PAINTSTRUCT) -> HDC;

        unsafe fn EndPaint(hwnd: HWND, lpPaint: *PAINTSTRUCT) -> BOOL;

        unsafe fn MessageBoxW(
                hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT
        ) -> c_int;

        unsafe fn RegisterClassExW(lpwcx: *WNDCLASSEX) -> ATOM;

        unsafe fn DefWindowProcW(
                hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM
        ) -> LRESULT;

        unsafe fn GetMessageW(
                lpMsg: *MSG, hWnd: HWND,
                wMsgFilterMin: UINT, wMsgFilterMAx: UINT
        ) -> BOOL;

        unsafe fn PeekMessageW(
                lpMsg: *MSG, hWnd: HWND,
                wMsgFilterMin: UINT, wMsgFilterMAx: UINT, wRemoveMsg: UINT
        ) -> BOOL;

        unsafe fn PostMessageW(
                hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM
        ) -> BOOL;

        unsafe fn PostQuitMessage(nExitCode: c_int);

        unsafe fn TranslateMessage(lpMsg: *MSG) -> BOOL;

        unsafe fn DispatchMessageW(lpMsg: *MSG) -> LRESULT;

        // 32-bit only
        unsafe fn GetClassLongW(hwnd: HWND, nIndex: c_int) -> DWORD;

        // 32-bit only
        unsafe fn SetClassLongW(
                hwnd: HWND, nIndex: c_int, dwNewLong: LONG
        ) -> DWORD;
    }
}

pub mod gdi32 {
    use ll::*;
    #[link_args = "-lgdi32"]
    extern "stdcall" {
        unsafe fn TextOutW(
                hdc: HDC, nXStart: c_int, nYStart: c_int,
                lpString: LPWSTR, cchString: c_int
        ) -> BOOL;
    }
}