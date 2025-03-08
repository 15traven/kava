use std::path::Path;
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
    }, MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent
};

mod helpers;

enum UserEvent {
    TrayIconEvent(TrayIconEvent),
    MenuEvent(MenuEvent)
}

fn main() {
    let icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/light_icon.png");

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
   
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                let icon = helpers::load_icon(Path::new(icon_path));

                tray_icon = Some(
                    TrayIconBuilder::new()
                        .with_menu(Box::new(tray_menu.clone()))
                        .with_menu_on_left_click(false)
                        .with_icon(icon)
                        .build()
                        .unwrap()
                );
            }

            Event::UserEvent(UserEvent::TrayIconEvent(event)) => {
                match event {
                    TrayIconEvent::Click { button, ..  } => {
                        if button == MouseButton::Left {
                            println!("clicked");
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
