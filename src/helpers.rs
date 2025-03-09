use std::path::Path;
use tao::window::Theme;
use tray_icon::{Icon, TrayIcon};

const LIGHT_ICON_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/light_icon.png");
const LIGHT_ICON_ACTIVE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/light_icon_active.png");
const DARK_ICON_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/dark_icon.png");
const DARK_ICON_ACTIVE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/dark_icon_active.png");

fn load_icon(path: &std::path::Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    Icon::from_rgba(
        icon_rgba, 
        icon_width, 
        icon_height
    ).expect("Failed to open icon")
}


pub fn set_icon(tray_icon: TrayIcon, theme: Theme, is_activated: bool) {
    let icon: Option<Icon> = match theme {
        Theme::Light => {
            if is_activated {
                Some(load_icon(Path::new(DARK_ICON_ACTIVE_PATH)))
            } else {
                Some(load_icon(Path::new(DARK_ICON_PATH)))
            }
        },
        Theme::Dark => {
            if is_activated {
                Some(load_icon(Path::new(LIGHT_ICON_ACTIVE_PATH)))
            } else {
                Some(load_icon(Path::new(LIGHT_ICON_PATH)))
            }
        },
        _ => None,
    };

    let _ = tray_icon.set_icon(icon);
}