use windows::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};
use windows::core::HSTRING;

pub(crate) struct StartupDialog;

impl StartupDialog {
    pub(crate) fn show(error: &str) {
        let text = format!(
            "Aurion could not start / Aurion не запустился:\n\n{error}\n\n\
             Check that Microsoft Edge WebView2 Runtime is installed.\n\
             Проверьте, что установлен Microsoft Edge WebView2 Runtime.\n\
             https://developer.microsoft.com/microsoft-edge/webview2/"
        );

        // SAFETY: a modal message box without an owner window; both strings outlive the call.
        unsafe {
            MessageBoxW(
                None,
                &HSTRING::from(text),
                &HSTRING::from("Aurion"),
                MB_OK | MB_ICONERROR,
            );
        }
    }
}
