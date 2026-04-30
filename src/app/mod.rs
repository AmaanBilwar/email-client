pub mod actions;
pub mod state;

use gpui::{Context, IntoElement, Render, Window, div, prelude::*, px};

use crate::ui::theme;

pub struct EmailClientApp {
    pub state: state::AppState,
}

impl EmailClientApp {
    pub fn new() -> Self {
        Self {
            state: state::AppState::new(),
        }
    }
}

impl Render for EmailClientApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        window.set_rem_size(px(self.state.rem_size_px));
        let selected_email = self.state.selected_email().clone();

        div()
            .size_full()
            .relative()
            .bg(theme::c(theme::APP_BG))
            .text_color(theme::c(theme::TEXT_PRIMARY))
            .flex()
            .flex_col()
            .child(self.render_top_ribbon(cx))
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .flex()
                    .child(self.render_sidebar(cx))
                    .child(self.render_detail_pane(&selected_email)),
            )
            .when(self.state.account_menu_open, |this| {
                this.child(self.render_account_menu(cx))
            })
    }
}
