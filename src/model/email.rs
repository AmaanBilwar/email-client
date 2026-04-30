#[derive(Clone)]
pub struct EmailSummary {
    pub sender: &'static str,
    pub subject: &'static str,
    pub preview: &'static str,
    pub time: &'static str,
    pub body: &'static str,
}
