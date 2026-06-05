use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, rgb,
};

use crate::components::button::Button;

pub struct BaseView {
    button: Entity<Button>,
}
impl BaseView {
    pub fn new(cx: &mut App) -> BaseView {
        BaseView {
            button: cx.new(|_| Button::new("click me")),
        }
    }
}

impl Render for BaseView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(0xffffff))
            .child("Hello, world!")
            .child(self.button.clone())
    }
}
