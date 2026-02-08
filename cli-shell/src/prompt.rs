use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::{cursor, execute};
use std::io::{stdout, Write};

pub fn draw_prompt() {
    let mut out = stdout();
    let _ = execute!(
        out,
        SetForegroundColor(Color::Cyan),
        Print("[MT6883-VSC] > "),
        ResetColor
    );
    let _ = out.flush();
}

pub fn print_result(ok: bool, message: &str, payload: &Option<serde_json::Value>) {
    let mut out = stdout();
    let color = if ok { Color::Green } else { Color::Red };
    let _ = execute!(
        out,
        cursor::MoveToNextLine(1),
        SetForegroundColor(color),
        Print(message),
        ResetColor
    );
    if let Some(p) = payload {
        let pretty = serde_json::to_string_pretty(p).unwrap_or_default();
        let _ = execute!(
            out,
            cursor::MoveToNextLine(1),
            SetForegroundColor(Color::DarkGrey),
            Print(pretty),
            ResetColor
        );
    }
    let _ = execute!(out, cursor::MoveToNextLine(1));
}
