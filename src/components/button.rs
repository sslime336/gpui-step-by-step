use gpui::{
    Context, CursorStyle, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Window, div, rgb,
};

pub struct Button {
    text: SharedString,
}

impl Button {
    pub fn new(text: &str) -> Button {
        Button {
            text: SharedString::from(text.to_owned()),
        }
    }
}

impl Render for Button {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id(self.text.clone())
            .flex()
            .justify_center()
            .items_center()
            .w_16()
            .h_8()
            .bg(rgb(0x8F98BD))
            .text_color(rgb(0xffffff))
            .rounded_sm()
            .hover(|style| style.cursor(CursorStyle::PointingHand).bg(rgb(0x615F73)))
            .child(self.text.clone())
            .on_click(|_, _window, _cx| {
                println!("I have been clicked!");
            })
    }
}
