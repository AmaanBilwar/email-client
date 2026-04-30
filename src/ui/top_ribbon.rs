use gpui::{FontWeight, IntoElement, Window, div, px, prelude::*};

use crate::{app::EmailClientApp, ui::theme};

impl EmailClientApp {
    pub(crate) fn render_top_ribbon(&mut self, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let selected_account = self.state.selected_account();

        div()
            .h(px(64.0))
            .w_full()
            .px_4()
            .border_b_1()
            .border_color(theme::c(theme::BORDER_STRONG))
            .bg(theme::c(theme::PANEL_BG))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        theme::text_title(div())
                            .font_weight(FontWeight::BOLD)
                            .child("MailDesk"),
                    )
                    .child(
                        theme::text_caption(
                            div()
                                .px_3()
                                .py_1()
                                .rounded_md()
                                .bg(theme::c(theme::BADGE_BG)),
                        )
                        .child("Inbox"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div().relative().child(
                            theme::text_body(
                                div()
                                    .id("account_toggle")
                                    .cursor_pointer()
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .bg(theme::c(theme::BTN_BG))
                                    .text_color(theme::c(0xf9fafb)),
                            )
                            .child(format!("{} v", selected_account))
                            .hover(|this| this.bg(theme::c(theme::BTN_HOVER)))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toggle_account_menu(cx);
                            })),
                        ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .id("minimize_btn")
                                    .cursor_pointer()
                                    .w(px(22.0))
                                    .h(px(22.0))
                                    .rounded_sm()
                                    .bg(theme::c(theme::BTN_BG))
                                    .text_xs()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child("_")
                                    .hover(|this| this.bg(theme::c(theme::BTN_HOVER)))
                                    .on_click(|_, window: &mut Window, _| {
                                        window.minimize_window();
                                    }),
                            )
                            .child(
                                div()
                                    .id("maximize_btn")
                                    .cursor_pointer()
                                    .w(px(22.0))
                                    .h(px(22.0))
                                    .rounded_sm()
                                    .bg(theme::c(theme::BTN_BG))
                                    .text_xs()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child("[]")
                                    .hover(|this| this.bg(theme::c(theme::BTN_HOVER)))
                                    .on_click(|_, window: &mut Window, _| {
                                        window.zoom_window();
                                    }),
                            )
                            .child(
                                div()
                                    .id("close_btn")
                                    .cursor_pointer()
                                    .w(px(22.0))
                                    .h(px(22.0))
                                    .rounded_sm()
                                    .bg(theme::c(theme::BTN_CLOSE_BG))
                                    .text_xs()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child("X")
                                    .hover(|this| this.bg(theme::c(theme::BTN_CLOSE_HOVER)))
                                    .on_click(|_, window: &mut Window, _| {
                                        window.remove_window();
                                    }),
                            ),
                    ),
            )
    }
}
