use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    div, rgb,
};

use crate::components::button::{Button, ClickButtonEvent};

pub struct BaseView {
    button: Entity<Button>,
    count: isize,
    #[allow(dead_code)]
    subscription: Subscription,
}
impl BaseView {
    pub fn new(cx: &mut Context<Self>) -> BaseView {
        let button = cx.new(|_| Button::new("click me"));
        let subscription = cx.subscribe(
            &button,
            |base_view, _button, event: &ClickButtonEvent, cx| {
                match event {
                    ClickButtonEvent::LeftClick => {
                        base_view.count = base_view.count.saturating_add(1)
                    }
                    ClickButtonEvent::RightClick => {
                        base_view.count = base_view.count.saturating_sub(1)
                    }
                    ClickButtonEvent::MiddleClick => base_view.count = 0,
                };
                cx.notify();
            },
        );
        BaseView {
            button,
            count: 0,
            subscription,
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
            .child(format!("当前计数 {}", self.count))
            .child(self.button.clone())
    }
}
