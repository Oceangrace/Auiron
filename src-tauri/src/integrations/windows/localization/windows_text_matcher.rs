use crate::core::localization::TextMatcher;
use windows::Win32::Foundation::{ERROR_SUCCESS, GetLastError, LPARAM};
use windows::Win32::Globalization::{
    FIND_FROMSTART, FindNLSStringEx, LINGUISTIC_IGNORECASE, LINGUISTIC_IGNOREDIACRITIC,
    ResolveLocaleName,
};
use windows::Win32::System::SystemServices::LOCALE_NAME_MAX_LENGTH;
use windows::core::PCWSTR;

pub(crate) struct WindowsTextMatcher;

impl WindowsTextMatcher {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl TextMatcher for WindowsTextMatcher {
    fn contains(&self, locale: &str, text: &str, query: &str) -> Result<bool, String> {
        if query.is_empty() {
            return Ok(true);
        }

        if text.is_empty() {
            return Ok(false);
        }

        let locale: Vec<u16> = locale.encode_utf16().chain(Some(0)).collect();

        let mut resolved = [0; LOCALE_NAME_MAX_LENGTH as usize];

        if unsafe { ResolveLocaleName(PCWSTR(locale.as_ptr()), Some(&mut resolved)) } == 0 {
            return Err("Search locale is not supported by Windows".into());
        }

        let text: Vec<u16> = text.encode_utf16().collect();

        let query: Vec<u16> = query.encode_utf16().collect();

        let position = unsafe {
            FindNLSStringEx(
                PCWSTR(resolved.as_ptr()),
                FIND_FROMSTART | (LINGUISTIC_IGNORECASE | LINGUISTIC_IGNOREDIACRITIC).0,
                &text,
                &query,
                None,
                None,
                None,
                LPARAM::default(),
            )
        };

        if position >= 0 {
            return Ok(true);
        }

        let error = unsafe { GetLastError() };

        if error != ERROR_SUCCESS {
            return Err(format!("Windows text search failed: {error:?}"));
        }

        Ok(false)
    }
}
