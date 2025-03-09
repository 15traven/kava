use tao::{
    event::{Event, StartCause},
    event_loop::{
        ControlFlow, 
        EventLoop, 
        EventLoopBuilder, 
        EventLoopProxy
    }
};
use tray_icon::{
    menu::{
        Menu,
        MenuEvent,
        MenuItem,
    }, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent
};

mod helpers;
mod keepawake;

use keepawake::KeepAwake;

enum UserEvent {
    TrayIconEvent(TrayIconEvent),
    MenuEvent(MenuEvent)
}

fn main() {
    let (
        light_icon,
        light_icon_active
    ) = helpers::load_icons();

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
    let quit_item: MenuItem = MenuItem::new("Quit", true, None);
    let _ = tray_menu.append_items(&[
        &quit_item
    ]);

    let mut tray_icon: Option<TrayIcon> = None;
    
    let mut keepawake: Option<KeepAwake> = None;
    let mut is_activated: bool = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                tray_icon = Some(
                    TrayIconBuilder::new()
                        .with_menu(Box::new(tray_menu.clone()))
                        .with_menu_on_left_click(false)
                        .with_icon(light_icon.clone())
                        .build()
                        .unwrap()
                );
                
                keepawake = Some(KeepAwake::new().unwrap());
            }

            Event::UserEvent(UserEvent::TrayIconEvent(event)) => {
                match event {
                    TrayIconEvent::Click {  button, button_state, ..  } => {
                        if button == MouseButton::Left && button_state == MouseButtonState::Up {
                            if !is_activated {
                                if keepawake.as_mut().unwrap().activate().is_ok() {
                                    let _ = tray_icon.as_mut().unwrap().set_icon(Some(light_icon_active.clone()));
                                }
                            } else {
                                drop(keepawake.clone().unwrap());
                                let _ = tray_icon.as_mut().unwrap().set_icon(Some(light_icon.clone()));
                            }

                            is_activated = !is_activated;
                        }
                    },
                    _ => {},
                }
            }

            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                if event.id == quit_item.id() {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => {}
        }
    });
}
