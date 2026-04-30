use gpui::{FontWeight, IntoElement, div, px, prelude::*, relative};
use gpui_component::scroll::ScrollableElement;

use crate::{app::EmailClientApp, ui::theme};

impl EmailClientApp {
    pub(crate) fn render_sidebar(&mut self, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div()
            .w(relative(0.25))
            .h_full()
            .bg(theme::c(theme::PANEL_BG))
            .border_r_1()
            .border_color(theme::c(theme::BORDER_STRONG))
            .flex()
            .flex_col()
            .child(
                theme::text_caption(
                    div()
                        .h(px(48.0))
                        .px_3()
                        .flex()
                        .items_center()
                        .border_b_1()
                        .border_color(theme::c(theme::BORDER_STRONG)),
                )
                .child("Emails"),
            )
            .child(
                div().flex_1().overflow_y_scrollbar().children(
                    self.state.emails.iter().enumerate().map(|(ix, email)| {
                        let is_selected = ix == self.state.selected_email_idx;
                        div()
                            .id(("email-row", ix))
                            .cursor_pointer()
                            .px_3()
                            .py_2()
                            .border_b_1()
                            .border_color(theme::c(theme::BORDER_STRONG))
                            .when(is_selected, |this| this.bg(theme::c(theme::ROW_SELECTED)))
                            .hover(|this| this.bg(theme::c(theme::ROW_HOVER)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.select_email(ix, cx);
                            }))
                            .child(
                                theme::text_body(div().flex().justify_between())
                                    .child(email.sender)
                                    .child(email.time),
                            )
                            .child(
                                theme::text_title(div().pt_1().font_weight(FontWeight::BOLD))
                                    .child(email.subject),
                            )
                            .child(
                                theme::text_caption(div().pt_1().text_color(theme::c(theme::TEXT_SUBTLE)))
                                    .child(email.preview),
                            )
                    }),
                ),
            )
    }
}
