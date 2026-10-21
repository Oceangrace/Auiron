use super::github_release::GitHubRelease;
use super::http_handle::HttpHandle;
use crate::core::updates::ReleaseInfo;
use std::ffi::c_void;
use std::ptr;
use std::time::{Duration, Instant};
use windows::Win32::Networking::WinHttp::*;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::{HSTRING, PCWSTR, w};

pub(crate) struct WindowsReleases;

impl WindowsReleases {
    // Keep background release checks bounded, including unexpectedly large release notes.
    const TIMEOUT_MS: i32 = 5_000;
    const MAX_BYTES: usize = 256 * 1024;
    const DEADLINE: Duration = Duration::from_secs(15);
    // ShellExecute reports success with values greater than 32.
    const SHELL_ERROR_LIMIT: isize = 32;

    pub(crate) fn check() -> Result<ReleaseInfo, String> {
        let release = Self::request()?
            .map(|bytes| serde_json::from_slice::<GitHubRelease>(&bytes))
            .transpose()
            .map_err(|_| "Invalid GitHub release response".to_owned())?;

        ReleaseInfo::new(
            env!("CARGO_PKG_VERSION"),
            release.as_ref().and_then(GitHubRelease::version),
        )
    }

    pub(crate) fn open() -> Result<(), String> {
        // SAFETY: only a fixed HTTPS page is opened; IPC accepts no arbitrary URL or command.
        let result = unsafe {
            ShellExecuteW(
                None,
                w!("open"),
                w!("https://github.com/Oceangrace/Aurion/releases/latest"),
                PCWSTR::null(),
                PCWSTR::null(),
                SW_SHOWNORMAL,
            )
        };

        if result.0 as isize <= Self::SHELL_ERROR_LIMIT {
            return Err("Could not open the release page".into());
        }

        Ok(())
    }

    fn request() -> Result<Option<Vec<u8>>, String> {
        let started = Instant::now();

        let agent = HSTRING::from(format!("Aurion/{}", env!("CARGO_PKG_VERSION")));

        let headers: Vec<u16> =
            "Accept: application/vnd.github+json\r\nX-GitHub-Api-Version: 2022-11-28\r\n"
                .encode_utf16()
                .collect();

        // SAFETY: handles and UTF-16 strings remain local and outlive the request.
        // WinHTTP validates TLS; redirects, cookies and authentication are disabled.
        unsafe {
            let session = HttpHandle::new(WinHttpOpen(
                &agent,
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

            let connection =
                HttpHandle::new(WinHttpConnect(session.raw(), w!("api.github.com"), 443, 0))?;

            // The repository ID stays the same when its name or owner changes.
            let request = HttpHandle::new(WinHttpOpenRequest(
                connection.raw(),
                w!("GET"),
                w!("/repositories/1427707629/releases/latest"),
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

            WinHttpSendRequest(request.raw(), Some(&headers), None, 0, 0, 0)
                .map_err(|error| error.to_string())?;

            WinHttpReceiveResponse(request.raw(), ptr::null_mut())
                .map_err(|error| error.to_string())?;

            match Self::status(&request)? {
                200 => Self::body(&request, started).map(Some),
                404 => Ok(None),
                status => Err(format!("GitHub release check HTTP {status}")),
            }
        }
    }

    fn status(request: &HttpHandle) -> Result<u32, String> {
        let mut status = 0_u32;

        let mut length = size_of::<u32>() as u32;

        // SAFETY: the output pointer addresses a live u32 of the declared size.
        unsafe {
            WinHttpQueryHeaders(
                request.raw(),
                WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
                PCWSTR::null(),
                Some(ptr::from_mut(&mut status).cast::<c_void>()),
                &mut length,
                ptr::null_mut(),
            )
        }
        .map_err(|error| error.to_string())?;

        Ok(status)
    }

    fn body(request: &HttpHandle, started: Instant) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();

        let mut buffer = [0_u8; 4096];

        loop {
            if started.elapsed() > Self::DEADLINE {
                return Err("GitHub release check deadline exceeded".into());
            }

            let mut read = 0;

            // SAFETY: WinHTTP writes at most the declared buffer length.
            unsafe {
                WinHttpReadData(
                    request.raw(),
                    buffer.as_mut_ptr().cast(),
                    buffer.len() as u32,
                    &mut read,
                )
            }
            .map_err(|error| error.to_string())?;

            if read == 0 {
                return Ok(bytes);
            }

            if bytes.len() + read as usize > Self::MAX_BYTES {
                return Err("GitHub release response is too large".into());
            }

            bytes.extend_from_slice(&buffer[..read as usize]);
        }
    }
}
