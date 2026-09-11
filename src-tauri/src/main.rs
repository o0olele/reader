// Prevents an extra console window on Windows in release builds. Without this
// the binary is linked as a console app (PE subsystem 3), and Windows opens a
// terminal whenever the GUI is launched. DO NOT REMOVE.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    reader_desktop_lib::run()
}
