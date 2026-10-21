use super::http_handle::HttpHandle;
use crate::core::icons::IconCache;
use crate::core::icons::IconName;
use crate::core::icons::IconResponse;
use crate::core::icons::IconTransport;
use std::ffi::c_void;
use std::ptr;
use std::time::Duration;
use std::time::Instant;
use windows::Win32::Networking::WinHttp::*;
use windows::core::PCWSTR;
use windows::core::w;

pub(crate) struct WindowsIconTransport;

impl WindowsIconTransport {
    const TIMEOUT_MS: i32 = 10_000;

    fn request(&self, name: &str) -> Result<IconResponse, String> {
        if !IconName::component(name) {
            return Ok(IconResponse::Retry("Invalid CDN icon name".into()));
        }

        let path: Vec<u16> = format!("/static-aion2-gamedata/resources/{name}.png\0")
            .encode_utf16()
            .collect();
        let started = Instant::now();

        // All handles stay local to this call and outlive the child request. TLS certificate
        // validation is WinHTTP's default; redirects, cookies and authentication are disabled.
        unsafe {
            let session = HttpHandle::new(WinHttpOpen(
                w!("Aurion/0.1"),
                WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY,
                PCWSTR::null(),
                PCWSTR::null(),
                0,
            ))?;

            WinHttpSetTimeouts(
                session.raw(),
                Self::TIMEOUT_MS,
                Self::TIMEOUT_MS,
                Self::TIMEOUT_MS,
                Self::TIMEOUT_MS,
            )
            .map_err(|error| error.to_string())?;

            let connection = HttpHandle::new(WinHttpConnect(
                session.raw(),
                w!("assets.playnccdn.com"),
                443,
                0,
            ))?;

            let request = HttpHandle::new(WinHttpOpenRequest(
                connection.raw(),
                w!("GET"),
                PCWSTR(path.as_ptr()),
                PCWSTR::null(),
                PCWSTR::null(),
                ptr::null(),
                WINHTTP_FLAG_SECURE,
            ))?;

            let disabled = WINHTTP_DISABLE_REDIRECTS
                | WINHTTP_DISABLE_COOKIES
                | WINHTTP_DISABLE_AUTHENTICATION;

            WinHttpSetOption(
                Some(request.raw()),
                WINHTTP_OPTION_DISABLE_FEATURE,
                Some(&disabled.to_ne_bytes()),
            )
            .map_err(|error| error.to_string())?;

            WinHttpSendRequest(request.raw(), None, None, 0, 0, 0)
                .map_err(|error| error.to_string())?;

            WinHttpReceiveResponse(request.raw(), ptr::null_mut())
                .map_err(|error| error.to_string())?;

            let mut status = 0_u32;
            let mut length = size_of::<u32>() as u32;

            WinHttpQueryHeaders(
                request.raw(),
                WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
                PCWSTR::null(),
                Some(ptr::from_mut(&mut status).cast::<c_void>()),
                &mut length,
                ptr::null_mut(),
            )
            .map_err(|error| error.to_string())?;

            if status == 404 {
                return Ok(IconResponse::Missing);
            }

            if status != 200 {
                return Ok(IconResponse::Retry(format!("CDN HTTP {status}: {name}")));
            }

            let mut bytes = Vec::new();
            let mut buffer = [0_u8; 16 * 1024];

            loop {
                if started.elapsed() > Duration::from_millis(Self::TIMEOUT_MS as u64) {
                    return Ok(IconResponse::Retry(format!(
                        "CDN download deadline: {name}"
                    )));
                }

                let mut read = 0;
                WinHttpReadData(
                    request.raw(),
                    buffer.as_mut_ptr().cast(),
                    buffer.len() as u32,
                    &mut read,
                )
                .map_err(|error| error.to_string())?;

                if read == 0 {
                    break;
                }

                bytes.extend_from_slice(&buffer[..read as usize]);

                if bytes.len() > IconCache::MAX_BYTES {
                    return Ok(IconResponse::Retry(format!("CDN icon too large: {name}")));
                }
            }

            Ok(IconResponse::Image(bytes))
        }
    }
}

impl IconTransport for WindowsIconTransport {
    fn fetch(&self, name: &str) -> IconResponse {
        self.request(name).unwrap_or_else(IconResponse::Offline)
    }
}
