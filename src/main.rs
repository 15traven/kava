use tao::{
    event::{
        Event, 
        StartCause, 
        WindowEvent
    },
    event_loop::{
        ControlFlow, 
        EventLoop, 
        EventLoopBuilder, 
        EventLoopProxy
    }, 
    window::{Window, WindowBuilder}
};
use tray_icon::{
    menu::{
        CheckMenuItem, 
        Menu, 
        MenuEvent, 
        MenuItem, 
        PredefinedMenuItem
    }, 
    MouseButton, 
    MouseButtonState, 
    TrayIcon, 
    TrayIconBuilder,
    TrayIconEvent,
};

mod helpers;
mod keepawake;

use keepawake::KeepAwake;

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

    let tray_menu: Menu = Menu::new();
    let autolaunch_item = CheckMenuItem::new("Run at startup", true, true, None);
    let quit_item: MenuItem = MenuItem::new("Quit", true, None);
    let _ = tray_menu.append_items(&[
        &autolaunch_item,
        &PredefinedMenuItem::separator(),
        &quit_item
    ]);

    let mut window: Option<Window> = None;
    let mut tray_icon: Option<TrayIcon> = None;
    
    let mut keepawake: Option<KeepAwake> = None;
    let mut is_activated: bool = false;

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
                helpers::icons::set_icon(
                    tray_icon.clone().unwrap(), 
                    window.as_ref().unwrap().theme(), 
                    is_activated
                );
                let _ = helpers::autolaunch::register();
                autolaunch_item.set_checked(helpers::autolaunch::is_enabled().unwrap());

                keepawake = Some(KeepAwake::new().unwrap());
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ThemeChanged(theme) => helpers::icons::set_icon(tray_icon.clone().unwrap(), theme, is_activated),
                _ => {}
            }

            Event::UserEvent(UserEvent::TrayIconEvent(event)) => {
                match event {
                    TrayIconEvent::Click {  button, button_state, ..  } => {
                        if button == MouseButton::Left && button_state == MouseButtonState::Up {
                            if !is_activated {
                                if keepawake.as_mut().unwrap().activate().is_ok() {
                                    helpers::icons::set_icon(
                                        tray_icon.clone().unwrap(), 
                                        window.as_ref().unwrap().theme(),
                                        true
                                    );
                                }
                            } else {
                                drop(keepawake.clone().unwrap());
                                helpers::icons::set_icon(
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
                if event.id == autolaunch_item.id() {
                    let _ = match helpers::autolaunch::is_enabled().unwrap() {
                        true => helpers::autolaunch::disable(),
                        false => helpers::autolaunch::enable()
                    };
                }

                if event.id == quit_item.id() {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => {}
        }
    });
}
