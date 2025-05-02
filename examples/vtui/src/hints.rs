use ratatui::style::Color;
use ratatui::text::{Line, Span};

/// Decorate hints with a glowing effect for the key letters. It is simple algorithm.
/// 1. Get the first letters of the hint up to the first space.
/// 2. Create white text with the left part of the hint.
/// 3. Create "color" text with the remainder of the hint.
/// 4. Join the two parts together in a Line.
pub(crate) fn decorate_hints<'a>(hints: &'a[&'a str]) -> Vec<Line<'a>> {
    let mut decorated_hints = Vec::new();
    for hint in hints {
        let mut parts = hint.split_whitespace();
        if let Some(first) = parts.next() {
            let first_part = first.to_string();
            let rest_part = parts.collect::<Vec<&str>>().join(" ");
            let padding = padding(first_part.chars().count());
            decorated_hints.push(Line::from(vec![
                Span::styled(first_part, Color::White),
                Span::from(padding),
                Span::from(rest_part),
            ]));
        }
    }
    decorated_hints
}

fn padding(i: usize) -> &'static str {
    match i {
        0 => "     ",
        1 => "    ",
        2 => "   ",
        3 => "  ",
        _ => " ",
    }
}

