use gpui::{
    App, Application, Bounds, Context, Entity, IntoElement, Render, TitlebarOptions, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, size,
};
use gpui_component::{
    ActiveTheme, Root,
    input::{Input, InputState},
};

struct Papyrus {
    input: Entity<InputState>,
}

impl Render for Papyrus {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .p_6()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(div().text_3xl().child("Papyrus"))
            .child(
                div()
                    .w_full()
                    .max_w(px(320.))
                    .child(Input::new(&self.input)),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        let bounds = Bounds::centered(None, size(px(800.), px(600.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Papyrus".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                let input = cx.new(|cx| InputState::new(window, cx).placeholder("Type here..."));
                input.update(cx, |input, cx| input.focus(window, cx));
                let view = cx.new(|_| Papyrus { input });
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
        .expect("failed to open Papyrus window");
        cx.activate(true);
    });
}
