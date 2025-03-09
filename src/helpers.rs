use std::path::Path;
use tray_icon::Icon;

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    tray_icon::Icon::from_rgba(
        icon_rgba, 
        icon_width, 
        icon_height
    ).expect("Failed to open icon")
}

pub fn load_icons() -> (Icon, Icon) {
    let light_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/light_icon.png");
    let light_icon_active_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/light_icon_active.png");

    let light_icon = load_icon(Path::new(light_icon_path));
    let light_icon_active = load_icon(Path::new(light_icon_active_path));

    (
        light_icon,
        light_icon_active
    )
}