mod app;
mod model;
mod ui;

use app::EmailClientApp;
use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, px, size};
use gpui::prelude::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(1200.0), px(760.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| EmailClientApp::new()),
        )
        .expect("failed to open app window");

        cx.activate(true);
    });
}
