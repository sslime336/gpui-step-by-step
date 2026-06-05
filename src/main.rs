mod base_view;
mod components;

use gpui::{AppContext, Application, Bounds, Point, Size, WindowBounds, WindowOptions, px};

use crate::base_view::BaseView;

fn main() {
    let app = Application::new();
    app.run(|cx| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point::default(),
                    size: Size::new(px(600.), px(480.)),
                })),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| BaseView::new(cx)),
        )
        .ok();
    });
}
