#![windows_subsystem = "windows"]

use std::sync::mpsc::channel;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{
        ControlFlow, EventLoop, 
        EventLoopBuilder, EventLoopProxy
    }, 
    window::{Window, WindowBuilder}
};
use tray_icon::{
    menu::{
        AboutMetadata, CheckMenuItem, 
        Menu, MenuEvent, MenuItem, 
        Submenu, PredefinedMenuItem
    }, 
    MouseButton, MouseButtonState, TrayIcon, 
    TrayIconBuilder,TrayIconEvent,
};

mod helpers;
mod keepawake;
mod autolaunch;
mod preferences;

use keepawake::KeepAwake;
use preferences::{
    Preferences,
    PREF_RUN_ACTIVATED
};

enum UserEvent {
    TrayIconEvent(TrayIconEvent),
    MenuEvent(MenuEvent)
}

fn main() {
    let event_loop: EventLoop<UserEvent> = EventLoopBuilder::<UserEvent>::with_user_event().build();

    let proxy: EventLoopProxy<UserEvent> = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::TrayIconEvent(event));
    }));

    let proxy: EventLoopProxy<UserEvent> = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let activate_for_submenu: Submenu = Submenu::new("Activate for", true);
    let activate_30_min: MenuItem = MenuItem::new("30 minutes", true, None);
    let activate_45_min: MenuItem = MenuItem::new("45 minutes", true, None);
    let activate_1_hour: MenuItem = MenuItem::new("1 hour", true, None);
    let activate_2_hour: MenuItem = MenuItem::new("2 hour", true, None);
    let _ = activate_for_submenu.append_items(&[
        &activate_30_min,
        &activate_45_min,
        &PredefinedMenuItem::separator(),
        &activate_1_hour,
        &activate_2_hour
    ]);

    let preferences_submenu: Submenu = Submenu::new("Preferences", true);
    let autolaunch_item = CheckMenuItem::new("Run at startup", true, true, None);
    let run_activated_item: CheckMenuItem = CheckMenuItem::new("Run activated", true, true, None);
    let _ = preferences_submenu.append_items(&[
        &run_activated_item,
        &autolaunch_item
    ]);
    
    let tray_menu: Menu = Menu::new();
    let quit_item: MenuItem = MenuItem::new("Quit", true, None);
    let _ = tray_menu.append_items(&[
        &activate_for_submenu,
        &PredefinedMenuItem::separator(),
        &preferences_submenu,
        &PredefinedMenuItem::about(None, Some(AboutMetadata {
            name: Some(env!("CARGO_PKG_NAME").to_string()),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
            ..Default::default()
        })),
        &PredefinedMenuItem::separator(),
        &quit_item
    ]);

    let mut window: Option<Window> = None;
    let mut tray_icon: Option<TrayIcon> = None;
    
    let mut preferences: Option<Preferences> = None;

    let mut keepawake: Option<KeepAwake> = None;
    let mut is_activated: bool = false;
    let (tx, rx) = channel::<()>();

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                window = Some(
                    WindowBuilder::new()
                        .with_visible(false)
                        .build(&event_loop)
                        .unwrap()
                );
                tray_icon = Some(
                    TrayIconBuilder::new()
                        .with_menu(Box::new(tray_menu.clone()))
                        .with_menu_on_left_click(false)
                        .build()
                        .unwrap()
                );
                helpers::set_icon(
                    tray_icon.clone().unwrap(), 
                    window.as_ref().unwrap().theme(), 
                    is_activated
                );

                preferences = Some(Preferences::new().unwrap());
                let _ = preferences.as_ref().unwrap().init();

                if let Ok(val) = preferences.as_ref().unwrap().load_preference(PREF_RUN_ACTIVATED) {
                    run_activated_item.set_checked(val);
                }

                let _ = autolaunch::register();
                autolaunch_item.set_checked(autolaunch::is_enabled().unwrap());

                keepawake = Some(KeepAwake::new().unwrap());
                if run_activated_item.is_checked() {
                    if keepawake.as_mut().unwrap().activate().is_ok() {
                        helpers::set_icon(
                            tray_icon.clone().unwrap(), 
                            window.as_ref().unwrap().theme(),
                            true
                        );

                        is_activated = true;
                    }
                }
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ThemeChanged(theme) => helpers::set_icon(tray_icon.clone().unwrap(), theme, is_activated),
                _ => {}
            }

            Event::UserEvent(UserEvent::TrayIconEvent(event)) => {
                match event {
                    TrayIconEvent::Click {  button, button_state, ..  } => {
                        if button == MouseButton::Left && button_state == MouseButtonState::Up {
                            if !is_activated {
                                if keepawake.as_mut().unwrap().activate().is_ok() {
                                    helpers::set_icon(
                                        tray_icon.clone().unwrap(), 
                                        window.as_ref().unwrap().theme(),
                                        true
                                    );
                                }
                            } else {
                                drop(keepawake.clone().unwrap());
                                helpers::set_icon(
                                    tray_icon.clone().unwrap(), 
                                    window.as_ref().unwrap().theme(), 
                                    false
                                );
                            }

                            is_activated = !is_activated;
                        }
                    },
                    _ => {},
                }
            }

            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                if event.id == activate_30_min.id() {
                    keepawake.as_mut().unwrap().activate_for(
                        5, 
                        tx.clone()
                    );
                    helpers::set_icon(
                        tray_icon.clone().unwrap(), 
                        window.as_ref().unwrap().theme(), 
                        true
                    );

                    is_activated = true;
                }

                if event.id == activate_45_min.id() {
                    keepawake.as_mut().unwrap().activate_for(
                        45 * 60, 
                        tx.clone()
                    );
                    helpers::set_icon(
                        tray_icon.clone().unwrap(), 
                        window.as_ref().unwrap().theme(), 
                        true
                    );

                    is_activated = true;
                }

                if event.id == activate_1_hour.id() {
                    keepawake.as_mut().unwrap().activate_for(
                        1 * 60 * 60, 
                        tx.clone()
                    );
                    helpers::set_icon(
                        tray_icon.clone().unwrap(), 
                        window.as_ref().unwrap().theme(), 
                        true
                    );

                    is_activated = true;
                }

                if event.id == activate_2_hour.id() {
                    keepawake.as_mut().unwrap().activate_for(
                        2 * 60 * 60, 
                        tx.clone()
                    );
                    helpers::set_icon(
                        tray_icon.clone().unwrap(), 
                        window.as_ref().unwrap().theme(), 
                        true
                    );

                    is_activated = true;
                }

                if event.id == run_activated_item.id() {
                    let _ = preferences.as_ref()
                        .unwrap()
                        .toggle_preference(PREF_RUN_ACTIVATED);
                }

                if event.id == autolaunch_item.id() {
                    let _ = match autolaunch::is_enabled().unwrap() {
                        true => autolaunch::disable(),
                        false => autolaunch::enable()
                    };
                }

                if event.id == quit_item.id() {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => {}
        }

        if let Ok(_) = rx.try_recv() {
            drop(keepawake.clone().unwrap());

            helpers::set_icon(
                tray_icon.clone().unwrap(), 
                window.as_ref().unwrap().theme(), 
                false
            );

            is_activated = false;
        }
    });
}
