use gpui::{
    Context, CursorStyle, EventEmitter, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Render, SharedString, Styled, Window, div, rgb,
};

pub struct Button {
    text: SharedString,
}

pub enum ClickButtonEvent {
    LeftClick,
    RightClick,
    MiddleClick,
}

impl EventEmitter<ClickButtonEvent> for Button {}

impl Button {
    pub fn new(text: &str) -> Button {
        Button {
            text: SharedString::from(text.to_owned()),
        }
    }
}

impl Render for Button {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|_this, _event, _window, cx| {
                    cx.emit(ClickButtonEvent::LeftClick);
                }),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|_this, _event, _window, cx| {
                    cx.emit(ClickButtonEvent::RightClick);
                }),
            )
            .on_mouse_down(
                MouseButton::Middle,
                cx.listener(|_this, _event, _window, cx| {
                    cx.emit(ClickButtonEvent::MiddleClick);
                }),
            )
    }
}
