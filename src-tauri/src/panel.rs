use tauri::{Emitter, Manager, Runtime, WebviewWindow};
use tauri_nspanel::{
    cocoa::{
        appkit::{NSView, NSWindowCollectionBehavior},
        base::{id, YES},
        foundation::{NSPoint, NSRect, NSSize},
    },
    objc::{msg_send, sel, sel_impl},
    panel_delegate, Panel, WebviewWindowExt as PanelWebviewWindowExt,
};
use thiserror::Error;

type TauriError = tauri::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("Unable to convert window to panel")]
    Panel,
    #[error("Monitor with cursor not found")]
    MonitorNotFound,
}

pub trait WebviewWindowExt {
    fn to_spotlight_panel(&self) -> tauri::Result<Panel>;

    fn center_at_cursor_monitor(&self) -> tauri::Result<()>;
}

impl<R: Runtime> WebviewWindowExt for WebviewWindow<R> {
    fn to_spotlight_panel(&self) -> tauri::Result<Panel> {
        // Convert window to panel
        let panel = self
            .to_panel()
            .map_err(|_| TauriError::Anyhow(Error::Panel.into()))?;

        // Set panel level to a high level (3 is NSFloatingWindowLevel in macOS)
        panel.set_level(5);

        // Prevent the panel from activating the application
        #[allow(non_upper_case_globals)]
        const NSWindowStyleMaskNonactivatingPanel: i32 = 1 << 7;
        const NS_WINDOW_STYLE_MASK_RESIZABLE: i32 = 1 << 3;

        // Set style mask to prevent app activation and allow resizing
        panel.set_style_mask(NSWindowStyleMaskNonactivatingPanel | NS_WINDOW_STYLE_MASK_RESIZABLE);

        // Set collection behavior to make the panel transient and prevent it from activating the app
        panel.set_collection_behaviour(
            NSWindowCollectionBehavior::NSWindowCollectionBehaviorTransient
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
        );

        // Set maximum and minimum size for the panel
        unsafe {
            if let Ok(handle) = self.ns_window() {
                let handle = handle as id;
                let max_size = NSSize::new(900.0, 1200.0);
                let min_size = NSSize::new(300.0, 200.0);
                #[allow(unexpected_cfgs)]
                let _: () = msg_send![handle, setMaxSize: max_size];
                #[allow(unexpected_cfgs)]
                let _: () = msg_send![handle, setMinSize: min_size];
            }
        }

        // Additional macOS-specific settings
        unsafe {
            if let Ok(handle) = self.ns_window() {
                let handle = handle as id;
                #[allow(unexpected_cfgs)]
                let _: () = msg_send![handle, setCanHide: 0];
                #[allow(unexpected_cfgs)]
                let _: () = msg_send![handle, setHidesOnDeactivate: 0];
            }
        }

        #[allow(unexpected_cfgs)]
        let panel_delegate = panel_delegate!(SpotlightPanelDelegate {
            window_did_resign_key,
            window_did_become_key
        });

        let app_handle = self.app_handle().clone();

        let label = self.label().to_string();

        panel_delegate.set_listener(Box::new(move |delegate_name: String| {
            match delegate_name.as_str() {
                "window_did_become_key" => {
                    let _ = app_handle.emit(format!("{}_panel_did_become_key", label).as_str(), ());
                }
                "window_did_resign_key" => {
                    let _ = app_handle.emit(format!("{}_panel_did_resign_key", label).as_str(), ());
                }
                _ => (),
            }
        }));

        panel.set_delegate(panel_delegate);

        Ok(panel)
    }

    fn center_at_cursor_monitor(&self) -> tauri::Result<()> {
        let monitor = monitor::get_monitor_with_cursor()
            .ok_or(TauriError::Anyhow(Error::MonitorNotFound.into()))?;

        let monitor_scale_factor = monitor.scale_factor();

        let monitor_size = monitor.size().to_logical::<f64>(monitor_scale_factor);

        let monitor_position = monitor.position().to_logical::<f64>(monitor_scale_factor);

        let window_handle: id = self.ns_window().unwrap() as _;

        let window_frame: NSRect = unsafe { window_handle.frame() };

        let rect = NSRect {
            origin: NSPoint {
                x: (monitor_position.x + (monitor_size.width / 2.0))
                    - (window_frame.size.width / 2.0),
                y: (monitor_position.y + (monitor_size.height / 2.0))
                    - (window_frame.size.height / 2.0),
            },
            size: window_frame.size,
        };

        #[allow(unexpected_cfgs)]
        let _: () = unsafe { msg_send![window_handle, setFrame: rect display: YES] };

        Ok(())
    }
}
