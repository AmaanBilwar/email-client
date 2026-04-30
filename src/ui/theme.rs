use gpui::{Rgba, Styled, rgb};

pub const APP_BG: u32 = 0x0b1220;
pub const PANEL_BG: u32 = 0x111827;
pub const BORDER_STRONG: u32 = 0x1f2937;
pub const BORDER_SOFT: u32 = 0x334155;
pub const TEXT_PRIMARY: u32 = 0xe5e7eb;
pub const TEXT_MUTED: u32 = 0x94a3b8;
pub const TEXT_SUBTLE: u32 = 0x9ca3af;
pub const ROW_HOVER: u32 = 0x1f2937;
pub const ROW_SELECTED: u32 = 0x1e3a8a;
pub const BADGE_BG: u32 = 0x1e293b;
pub const BTN_BG: u32 = 0x1f2937;
pub const BTN_HOVER: u32 = 0x334155;
pub const BTN_CLOSE_BG: u32 = 0x7f1d1d;
pub const BTN_CLOSE_HOVER: u32 = 0x991b1b;

pub fn c(hex: u32) -> Rgba {
    rgb(hex)
}

pub fn text_caption<T: Styled>(this: T) -> T {
    this.text_sm()
}

pub fn text_body<T: Styled>(this: T) -> T {
    this.text_base()
}

pub fn text_title<T: Styled>(this: T) -> T {
    this.text_lg()
}

pub fn text_headline<T: Styled>(this: T) -> T {
    this.text_2xl()
}
