use gpui::{FontWeight, IntoElement, SharedString, div, prelude::*, relative};

use crate::{app::EmailClientApp, model::email::EmailSummary, ui::theme};

impl EmailClientApp {
    pub(crate) fn render_detail_pane(&self, selected_email: &EmailSummary) -> impl IntoElement {
        div()
            .w(relative(0.75))
            .h_full()
            .p_6()
            .bg(theme::c(theme::APP_BG))
            .flex()
            .flex_col()
            .gap_4()
            .child(
                theme::text_headline(div().font_weight(FontWeight::BOLD)).child(selected_email.subject),
            )
            .child(
                theme::text_body(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .text_color(theme::c(theme::TEXT_MUTED)),
                )
                .child(format!("From: {}", selected_email.sender))
                .child(selected_email.time),
            )
            .child(
                div()
                    .rounded_md()
                    .border_1()
                    .border_color(theme::c(theme::BORDER_SOFT))
                    .bg(theme::c(theme::PANEL_BG))
                    .p_4()
                    .line_height(relative(1.5))
                    .child(SharedString::from(selected_email.body)),
            )
    }
}
