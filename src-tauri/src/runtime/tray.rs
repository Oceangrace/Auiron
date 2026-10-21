use crate::core::localization::LocaleRegistry;
use crate::runtime::QuitRequest;
use std::sync::Arc;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, Manager};

pub(crate) struct Tray {
    title: String,
    icon: &'static [u8],
}

impl Tray {
    pub(crate) fn new(title: String, icon: &'static [u8]) -> Self {
        Self { title, icon }
    }

    pub(crate) fn start(&self, app: &AppHandle) -> tauri::Result<()> {
        let menu = Self::menu(app, app.state::<Arc<LocaleRegistry>>().default_locale())?;

        TrayIconBuilder::with_id("overlay-tray")
            .icon(tauri::image::Image::from_bytes(self.icon)?)
            .tooltip(&self.title)
            .menu(&menu)
            .on_menu_event(|app, event| {
                if event.id.as_ref() == "quit" {
                    if let Some(window) = app.get_webview_window("editor") {
                        let _ = window.unminimize();

                        let _ = window.show();

                        let _ = window.set_focus();
                    }

                    QuitRequest::send(app);
                }

                if event.id.as_ref() == "editor"
                    && let Some(window) = app.get_webview_window("editor")
                {
                    let _ = window.unminimize();

                    let _ = window.show();

                    let _ = window.set_focus();

                    let _ = app.emit_to("editor", "interfaces:open-requested", ());
                }
            })
            .build(app)?;

        Ok(())
    }

    fn menu(app: &AppHandle, locale: &str) -> tauri::Result<Menu<tauri::Wry>> {
        let messages = app.state::<Arc<LocaleRegistry>>();

        let quit = MenuItem::with_id(
            app,
            "quit",
            messages.text(locale, "quit"),
            true,
            None::<&str>,
        )?;

        let editor = MenuItem::with_id(
            app,
            "editor",
            messages.text(locale, "editor"),
            true,
            None::<&str>,
        )?;

        Menu::with_items(app, &[&editor, &quit])
    }

    pub(crate) fn localize(app: &AppHandle, locale: &str) -> tauri::Result<()> {
        if let Some(tray) = app.tray_by_id("overlay-tray") {
            tray.set_menu(Some(Self::menu(app, locale)?))?;
        }

        if let Some(window) = app.get_webview_window("editor") {
            window.set_title(
                &app.state::<Arc<LocaleRegistry>>()
                    .text(locale, "windowTitle"),
            )?;
        }

        Ok(())
    }

    pub(crate) fn dispose(&self, app: &AppHandle) {
        app.remove_tray_by_id("overlay-tray");
    }
}
