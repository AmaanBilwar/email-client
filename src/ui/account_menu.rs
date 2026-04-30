use gpui::{IntoElement, div, px, prelude::*};

use crate::{app::EmailClientApp, ui::theme};

impl EmailClientApp {
    pub(crate) fn render_account_menu(&mut self, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div()
            .id("account_menu")
            .absolute()
            .top(px(58.0))
            .right_4()
            .w(px(200.0))
            .rounded_md()
            .border_1()
            .border_color(theme::c(theme::BORDER_SOFT))
            .bg(theme::c(theme::PANEL_BG))
            .shadow_lg()
            .p_1()
            .children(self.state.accounts.iter().enumerate().map(|(ix, account_name)| {
                let is_selected = ix == self.state.selected_account_idx;
                theme::text_body(
                    div()
                        .id(("account", ix))
                        .cursor_pointer()
                        .px_3()
                        .py_2()
                        .rounded_sm()
                        .when(is_selected, |row| row.bg(theme::c(theme::ROW_SELECTED)))
                        .hover(|row| row.bg(theme::c(theme::ROW_HOVER)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.select_account(ix, cx);
                        })),
                )
                .child(*account_name)
            }))
    }
}
