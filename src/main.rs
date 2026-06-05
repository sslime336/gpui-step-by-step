mod base_view;

use gpui::{
    AppContext, Application, Bounds, Context, IntoElement, ParentElement, Point, Render, Size,
    Styled, Window, WindowBounds, WindowOptions, div, px, rgb,
};

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
            |_window, cx| cx.new(|_cx| MyView {}),
        )
        .ok();
    });
}
struct MyView {}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(0xffffff))
            .child("Hello, world!")
    }
}
