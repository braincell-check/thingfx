use nalgebra::Vector2;

/// Generic trait for controlling on-screen windows
pub trait WindowControls<WindowHandle> {
    /// Set the window to fullscreen
    fn set_fullscreen(&mut self, fullscreen: bool);
    /// Toggle the window fullscreen state
    fn toggle_fullscreen(&mut self);
    /// Get the current window fullscreen status
    fn fullscreen(&self) -> bool;
    /// Check if the window is hidden
    fn hidden(&self) -> bool;
    /// Set the window to maximized
    fn maximize(&mut self);
    /// Get the current window maximized status
    fn maximized(&self) -> bool;
    /// Set the window to minimized
    fn minimize(&mut self);
    /// Get the current window minimized status
    fn minimized(&self) -> bool;
    /// Set the window to restored
    fn restore(&mut self);

    /// Set the window title
    fn set_title(&mut self, title: &str);
    /// Get the current window title
    fn title(&self) -> String;
    // TODO: Set the window icon
    /// Set the window position
    fn set_position(&mut self, position: Vector2<i32>);
    /// Get the current window position
    fn position(&self) -> Vector2<i32>;
    /// Set the window's parent monitor id (only when fullscreen)
    fn set_monitor(&mut self, monitor: i32);
    /// Set the window's minimum size
    fn set_min_size(&mut self, size: Vector2<i32>);
    /// Set the window's size
    fn set_size(&mut self, size: Vector2<i32>);
    /// Get the current window size
    fn size(&self) -> Vector2<i32>;
    /// Get the OS window handle
    fn handle(&self) -> WindowHandle;
}

/// Generic trait for interacting with the window's parent monitor
pub trait WindowMonitor {
    /// Get the monitor count
    fn monitor_count(&self) -> i32;
    /// Get the current monitor id
    fn monitor_id(&self) -> i32;
    /// Get the monitor's physical position
    fn monitor_position(&self, monitor: i32) -> Vector2<i32>;
    /// Get the monitor's physical size
    fn monitor_size(&self, monitor: i32) -> Vector2<u32>;
    /// Get the monitor's scale
    fn monitor_scale(&self, monitor: i32) -> f64;
    /// Get the monitor's name
    fn monitor_name(&self, monitor: i32) -> String;
}

/// Generic trait for controlling the OS cursor
pub trait CursorControls {
    /// Set the cursor visibility
    fn set_cursor_visible(&mut self, visible: bool);
    /// Get the cursor visibility
    fn cursor_visible(&self) -> bool;
    /// Set weather the cursor is enabled
    fn set_cursor_enabled(&mut self, enabled: bool);
    /// Get weather the cursor is enabled
    fn cursor_enabled(&self) -> bool;
    /// Check if the cursor is on the window
    fn cursor_on_window(&self) -> bool;
}

/// Overarching window trait
pub trait Window<WindowHandle>: WindowControls<WindowHandle> + WindowMonitor + CursorControls {}