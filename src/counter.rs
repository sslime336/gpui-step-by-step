use gpui::{
    Action, AppContext, Context, Entity, FocusHandle, InteractiveElement, IntoElement, KeyBinding,
    MouseButton, MouseDownEvent, ParentElement, Render, Styled, Subscription, Window, div,
    prelude::FluentBuilder, rgb,
};

use crate::components::button::{Button, ClickButtonEvent};

pub struct Counter {
    button: Entity<Button>,
    count: isize,
    #[allow(dead_code)]
    subscription: Subscription,
    focus_handle: FocusHandle,
}

#[derive(Debug, Clone, PartialEq, Eq, Action)]
#[action(namespace = counter, no_json)] // 如果不需要 json 文件进行映射的话，需要指定 no_json，映射实际类似于 VSCode 的 keymap
// #[action(no_json)] 可以不用 namespace
pub enum CounterAction {
    Up,
    Down,
    Reset,
}

// actions!(counter, [CountUp, CountDown, Reset]);

impl Counter {
    pub fn new(cx: &mut Context<Self>) -> Counter {
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
        cx.bind_keys([
            KeyBinding::new("up", CounterAction::Up, None),
            KeyBinding::new("down", CounterAction::Down, None),
            KeyBinding::new("backspace", CounterAction::Reset, None),
        ]);
        Counter {
            button,
            count: 0,
            subscription,
            focus_handle: cx.focus_handle(),
        }
    }

    fn count_action(
        &mut self,
        counter_action: &CounterAction,
        _window: &mut Window,
        cx: &mut Context<Counter>,
    ) {
        match counter_action {
            CounterAction::Up => self.count = self.count.saturating_add(1),
            CounterAction::Down => self.count = self.count.saturating_sub(1),
            CounterAction::Reset => self.count = 0,
        }
        cx.notify();
    }
}

impl Render for Counter {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .border_2()
            .border_color(rgb(0x98A7BA))
            .rounded_sm()
            .bg(rgb(0xeeeeee))
            .track_focus(&self.focus_handle)
            .when(self.focus_handle.is_focused(window), |counter| {
                counter.border_color(rgb(0x8FC9FC))
            })
            .on_action(cx.listener(Self::count_action))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _: &MouseDownEvent, window, cx| {
                    cx.stop_propagation();
                    window.focus(&this.focus_handle);
                }),
            )
            .child(format!("当前计数 {}", self.count))
            .child(self.button.clone())
    }
}
