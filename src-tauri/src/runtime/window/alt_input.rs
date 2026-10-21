use super::alt_keys::AltKeys;
use std::time::{Duration, Instant};
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LMENU, VK_MENU, VK_RMENU};
use windows::Win32::UI::Input::{
    GetRawInputData, GetRegisteredRawInputDevices, HRAWINPUT, RAWINPUT, RAWINPUTDEVICE,
    RAWINPUTHEADER, RAWKEYBOARD, RID_INPUT, RIDEV_DEVNOTIFY, RIDEV_INPUTSINK, RIDEV_REMOVE,
    RIM_TYPEKEYBOARD, RegisterRawInputDevices,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, GIDC_REMOVAL, HWND_MESSAGE, MSG, PM_REMOVE,
    PeekMessageW, WINDOW_EX_STYLE, WINDOW_STYLE, WM_INPUT, WM_INPUT_DEVICE_CHANGE,
};
use windows::core::w;

pub(super) struct AltInput {
    window: Option<HWND>,
    previous: Option<RAWINPUTDEVICE>,
    checked: Option<Instant>,
    keys: AltKeys,
}

impl AltInput {
    pub(super) fn new() -> Self {
        Self {
            window: None,
            previous: None,
            checked: None,
            keys: AltKeys::default(),
        }
    }

    pub(super) fn start(&mut self) -> Result<(), String> {
        self.previous = Self::registered()?;

        // SAFETY: predefined STATIC class; message-only window never displays or takes focus.
        let window = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("STATIC"),
                w!("Aurion Alt input"),
                WINDOW_STYLE::default(),
                0,
                0,
                0,
                0,
                Some(HWND_MESSAGE),
                None,
                None,
                None,
            )
        }
        .map_err(|error| format!("Alt input window unavailable: {error}"))?;

        if let Err(error) = Self::register(window) {
            // SAFETY: uniquely owned window created on this thread.
            let _ = unsafe { DestroyWindow(window) };

            return Err(error);
        }

        self.window = Some(window);

        Ok(())
    }

    pub(super) fn observe(&mut self) -> bool {
        if let Some(window) = self.window {
            self.refresh(window);

            let mut message = MSG::default();

            for _ in 0..256 {
                // SAFETY: valid message output; only our message-only window is drained.
                if !unsafe { PeekMessageW(&mut message, Some(window), 0, 0, PM_REMOVE) }.as_bool() {
                    break;
                }

                if message.message == WM_INPUT {
                    self.read(message.lParam);
                }

                if message.message == WM_INPUT_DEVICE_CHANGE
                    && message.wParam.0 == GIDC_REMOVAL as usize
                {
                    self.keys = AltKeys::default();
                }

                // SAFETY: default cleanup, including WM_INPUT; no event is suppressed in the game.
                unsafe { DefWindowProcW(window, message.message, message.wParam, message.lParam) };
            }
        }

        self.keys.held()
            || [VK_MENU, VK_LMENU, VK_RMENU].iter().any(|key| {
                // SAFETY: read-only fallback, high bit only. Zero never clears a raw Alt press.
                (unsafe { GetAsyncKeyState(i32::from(key.0)) }) < 0
            })
    }

    fn read(&mut self, parameter: LPARAM) {
        let mut input = RAWINPUT::default();

        let mut size = std::mem::size_of::<RAWINPUT>() as u32;

        let header = std::mem::size_of::<RAWINPUTHEADER>() as u32;

        // SAFETY: WM_INPUT handle; initialized aligned buffer and its actual capacity.
        let read = unsafe {
            GetRawInputData(
                HRAWINPUT(parameter.0 as _),
                RID_INPUT,
                Some((&mut input as *mut RAWINPUT).cast()),
                &mut size,
                header,
            )
        };

        if read == u32::MAX
            || read < header + std::mem::size_of::<RAWKEYBOARD>() as u32
            || input.header.dwType != RIM_TYPEKEYBOARD.0
        {
            return;
        }

        // SAFETY: validated keyboard discriminator and complete keyboard payload above.
        let keyboard = unsafe { input.data.keyboard };

        self.keys.update(keyboard.VKey, keyboard.Flags);
    }

    fn refresh(&mut self, window: HWND) {
        if self
            .checked
            .is_some_and(|last| last.elapsed() < Duration::from_secs(1))
        {
            return;
        }

        self.checked = Some(Instant::now());

        if let Ok(previous) = Self::registered()
            && previous.is_none_or(|entry| entry.hwndTarget != window)
        {
            self.previous = previous;

            let _ = Self::register(window);
        }
    }

    fn register(window: HWND) -> Result<(), String> {
        let device = RAWINPUTDEVICE {
            usUsagePage: 1,
            usUsage: 6,
            dwFlags: RIDEV_INPUTSINK | RIDEV_DEVNOTIFY,
            hwndTarget: window,
        };

        // SAFETY: valid device slice; does not request NOLEGACY or consume game input.
        unsafe { RegisterRawInputDevices(&[device], std::mem::size_of::<RAWINPUTDEVICE>() as u32) }
            .map_err(|error| format!("Alt input registration unavailable: {error}"))
    }

    fn registered() -> Result<Option<RAWINPUTDEVICE>, String> {
        let mut devices = [RAWINPUTDEVICE::default(); 32];

        let mut count = devices.len() as u32;

        // SAFETY: initialized output array and its actual element count.
        let read = unsafe {
            GetRegisteredRawInputDevices(
                Some(devices.as_mut_ptr()),
                &mut count,
                std::mem::size_of::<RAWINPUTDEVICE>() as u32,
            )
        };

        if read == u32::MAX || read as usize > devices.len() {
            return Err("Raw input registrations unavailable".into());
        }

        Ok(devices[..read as usize]
            .iter()
            .find(|entry| entry.usUsagePage == 1 && entry.usUsage == 6)
            .copied())
    }
}

impl Drop for AltInput {
    fn drop(&mut self) {
        let Some(window) = self.window.take() else {
            return;
        };

        if Self::registered()
            .ok()
            .flatten()
            .is_some_and(|entry| entry.hwndTarget == window)
        {
            let previous = self.previous.unwrap_or(RAWINPUTDEVICE {
                usUsagePage: 1,
                usUsage: 6,
                dwFlags: RIDEV_REMOVE,
                hwndTarget: HWND::default(),
            });

            // SAFETY: restore only the registration still owned by this runtime.
            let _ = unsafe {
                RegisterRawInputDevices(&[previous], std::mem::size_of::<RAWINPUTDEVICE>() as u32)
            };
        }

        // SAFETY: uniquely owned message-only window, disposed on its creating thread.
        let _ = unsafe { DestroyWindow(window) };
    }
}
