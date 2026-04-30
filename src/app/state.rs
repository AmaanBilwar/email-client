use crate::model::{email::EmailSummary, seed};

pub struct AppState {
    pub accounts: Vec<&'static str>,
    pub selected_account_idx: usize,
    pub account_menu_open: bool,
    pub emails: Vec<EmailSummary>,
    pub selected_email_idx: usize,
    pub rem_size_px: f32,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            accounts: seed::placeholder_accounts(),
            selected_account_idx: 0,
            account_menu_open: false,
            emails: seed::placeholder_emails(),
            selected_email_idx: 0,
            rem_size_px: 18.0,
        }
    }

    pub fn selected_email(&self) -> &EmailSummary {
        &self.emails[self.selected_email_idx]
    }

    pub fn selected_account(&self) -> &'static str {
        self.accounts[self.selected_account_idx]
    }
}
