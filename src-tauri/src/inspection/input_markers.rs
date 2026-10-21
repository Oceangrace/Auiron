use crate::runtime::GameWindow;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::{Duration, Instant};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::WindowsAndMessaging::{
    CURSOR_SHOWING, CURSORINFO, GetCursorInfo, GetForegroundWindow,
};

pub(super) struct InputMarkers {
    output: BufWriter<File>,
    process_name: String,
    started: Instant,
    sampled: Option<Instant>,
    foreground: usize,
    game_foreground: Option<bool>,
    refreshed: Option<Instant>,
    held: [bool; 3],
    maximum_gap_ms: u128,
    samples: u64,
    key_transitions: u64,
}

impl InputMarkers {
    pub(super) fn open(directory: &Path, process_name: &str) -> Result<Self, String> {
        let output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(directory.join("input-markers.jsonl"))
            .map(BufWriter::new)
            .map_err(|error| error.to_string())?;

        Ok(Self {
            output,
            process_name: process_name.to_owned(),
            started: Instant::now(),
            sampled: None,
            foreground: 0,
            game_foreground: None,
            refreshed: None,
            held: [false; 3],
            maximum_gap_ms: 0,
            samples: 0,
            key_transitions: 0,
        })
    }

    pub(super) fn sample(&mut self, at_us: u64) -> Result<(), String> {
        if self
            .sampled
            .is_some_and(|previous| previous.elapsed() < Duration::from_millis(10))
        {
            return Ok(());
        }

        if let Some(previous) = self.sampled {
            self.maximum_gap_ms = self.maximum_gap_ms.max(previous.elapsed().as_millis());
        }

        self.sampled = Some(Instant::now());
        self.samples += 1;

        // SAFETY: read-only query without caller-owned pointers.
        let window = unsafe { GetForegroundWindow() };

        let foreground = window.0 as usize;

        if foreground != self.foreground
            || self
                .refreshed
                .is_none_or(|previous| previous.elapsed() >= Duration::from_millis(100))
        {
            let game_foreground = GameWindow::owns(window, &self.process_name);

            if foreground != self.foreground || game_foreground != self.game_foreground {
                self.write(
                    at_us,
                    serde_json::json!({
                        "kind": "focus", "gameForeground": game_foreground,
                        "heldKeysReset": true,
                    }),
                )?;
                self.held = [false; 3];
            }

            self.foreground = foreground;
            self.game_foreground = game_foreground;
            self.refreshed = Some(Instant::now());

            let mut cursor = CURSORINFO {
                cbSize: std::mem::size_of::<CURSORINFO>() as u32,
                ..Default::default()
            };

            // SAFETY: correctly sized, initialized, writable Windows structure.
            let known = unsafe { GetCursorInfo(&mut cursor) }.is_ok();

            self.write(
                at_us,
                serde_json::json!({
                    "kind": "cursor", "gameForeground": game_foreground,
                    "known": known,
                    "shown": known.then_some(cursor.flags.0 & CURSOR_SHOWING.0 != 0),
                    "handle": known.then_some(cursor.hCursor.0 as usize),
                }),
            )?;
        }

        if self.game_foreground != Some(true) {
            return Ok(());
        }

        for (index, (key, code)) in [("F", 0x46), ("Escape", 0x1b), ("Alt", 0x12)]
            .iter()
            .enumerate()
        {
            // SAFETY: read-only state for diagnostic keys and the HUD visibility modifier.
            // Only the high bit is used; this does not consume key events.
            let down = unsafe { GetAsyncKeyState(*code) } < 0;

            if down != self.held[index] {
                self.held[index] = down;
                self.key_transitions += 1;
                self.write(
                    at_us,
                    serde_json::json!({
                        "kind": "key", "key": key, "down": down,
                        "gameForeground": true,
                    }),
                )?;
            }
        }

        Ok(())
    }

    pub(super) fn finish(&mut self, at_us: u64) -> Result<(), String> {
        self.write(at_us, serde_json::json!({
            "kind": "finished", "samples": self.samples,
            "maximumSampleGapMs": self.maximum_gap_ms,
            "requestedSampleIntervalMs": 10,
            "keyTransitions": self.key_transitions,
            "inputObservationVerified": false,
            "method": "foreground-only-key-state-polling",
            "limitation": "Key state may be inaccessible or transitions may fall between samples. No transitions does not prove no input. Key presses do not prove a panel opened.",
        }))
    }

    fn write(&mut self, at_us: u64, mut row: serde_json::Value) -> Result<(), String> {
        row["atUs"] = at_us.into();
        row["elapsedMs"] =
            (self.started.elapsed().as_millis().min(u128::from(u64::MAX)) as u64).into();
        serde_json::to_writer(&mut self.output, &row).map_err(|error| error.to_string())?;
        self.output
            .write_all(b"\n")
            .map_err(|error| error.to_string())?;
        self.output.flush().map_err(|error| error.to_string())
    }
}
