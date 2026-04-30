use gpui::Context;

use crate::app::EmailClientApp;

impl EmailClientApp {
    pub fn toggle_account_menu(&mut self, cx: &mut Context<Self>) {
        self.state.account_menu_open = !self.state.account_menu_open;
        cx.notify();
    }

    pub fn select_email(&mut self, idx: usize, cx: &mut Context<Self>) {
        self.state.selected_email_idx = idx;
        self.state.account_menu_open = false;
        cx.notify();
    }

    pub fn select_account(&mut self, idx: usize, cx: &mut Context<Self>) {
        self.state.selected_account_idx = idx;
        self.state.account_menu_open = false;
        cx.notify();
    }
}
