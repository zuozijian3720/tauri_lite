use serde_json::json;
use wry::application::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopWindowTarget},
    window::Theme,
};

use crate::window_manager::WebViewRef;

use super::CallbackEvent;

fn send_event<S, D>(main_webview_warper: WebViewRef, event: S, data: D)
where
    S: Into<String>,
    D: Into<serde_json::Value>,
{
    let event = event.into();
    let data_str = serde_json::to_string::<serde_json::Value>(&data.into()).unwrap();
    main_webview_warper
        .lock()
        .unwrap()
        .evaluate_script(&format!("TauriLite.__emit__(\"{event}\", {data_str})"))
        .unwrap();
}

pub fn handle_window_event(
    main_webview_warper: WebViewRef,
    event: &WindowEvent,
    control_flow: &mut ControlFlow,
) {
    match event {
        WindowEvent::Focused(focused) => send_event(
            main_webview_warper,
            "window.focused",
            json!({ "focused": focused }),
        ),
        WindowEvent::ScaleFactorChanged {
            scale_factor,
            new_inner_size,
        } => send_event(
            main_webview_warper,
            "window.scaleFactorChanged",
            json!({ "scaleFactor": scale_factor, "newInnerSize": new_inner_size }),
        ),
        WindowEvent::ThemeChanged(theme) => send_event(
            main_webview_warper,
            "window.themeChanged",
            json!({ "theme": match theme {
                Theme::Dark => "dark",
                Theme::Light => "light",
                _ => "",
            }}),
        ),
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        _ => (),
    }
}

pub fn handle(
    main_webview_warper: WebViewRef,
    event: Event<CallbackEvent>,
    target: &EventLoopWindowTarget<CallbackEvent>,
    control_flow: &mut ControlFlow,
) {
    *control_flow = ControlFlow::Wait;

    // TODO: Send other events to webview
    match event {
        Event::NewEvents(_) => (), //

        Event::WindowEvent { event, .. } => {
            handle_window_event(main_webview_warper, &event, control_flow)
        }

        Event::MenuEvent { menu_id, .. } => send_event(
            main_webview_warper,
            "menu.clicked",
            json!({ "menuId": menu_id.0 }),
        ),

        Event::UserEvent(callback) => {
            callback.call(&main_webview_warper.lock().unwrap(), target, control_flow)
        }

        _ => (),
    }
}
