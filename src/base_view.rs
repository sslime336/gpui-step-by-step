use gpui::{
    AppContext, Context, Entity, FocusHandle, InteractiveElement, IntoElement, MouseButton,
    MouseDownEvent, ParentElement, Render, Styled, Window, div, rgb,
};

use crate::counter::Counter;

pub struct BaseView {
    counter_1: Entity<Counter>,
    counter_2: Entity<Counter>,
    focus_handle: FocusHandle,
}
impl BaseView {
    pub fn new(cx: &mut Context<Self>) -> BaseView {
        let counter_1 = cx.new(|cx| Counter::new(cx));
        let counter_2 = cx.new(|cx| Counter::new(cx));
        BaseView {
            counter_1,
            counter_2,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for BaseView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_2()
            .size_full()
            .p_20()
            .items_center()
            .justify_center()
            .bg(rgb(0xffffff))
            .child(self.counter_1.clone())
            .child(self.counter_2.clone())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _: &MouseDownEvent, window, _cx| {
                    window.focus(&this.focus_handle);
                }),
            )
    }
}
