use ratatui::prelude::{Color, Line, Span};
use crate::resource_type::ResourceType;

pub(crate) const HELP_HINTS: &[&str] = &[
    "q quit",
    "/ search",
    "r resource",
    "0..9 sort",
    "↑/↓ scroll",
];
const EXPAND_NETWORK: &str = "n network";
const EXPAND_DATASTORE: &str = "d datastore";
const EXPAND_HOST: &str = "h host";
const EXPAND_VM: &str = "v vm";
const CLUSTER_EXPAND_HINTS: &[&str] = &[
    EXPAND_NETWORK,
    EXPAND_DATASTORE,
    EXPAND_HOST,
    EXPAND_VM,
];
const HOST_EXPAND_HINTS: &[&str] = &[
    EXPAND_NETWORK,
    EXPAND_DATASTORE,
    EXPAND_VM,
];
const DATASTORE_EXPAND_HINTS: &[&str] = &[
    EXPAND_HOST,
    EXPAND_VM,
];
const NETWORK_EXPAND_HINTS: &[&str] = &[
    EXPAND_HOST,
    EXPAND_VM,
];

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

pub(crate) fn get_expand_hint(resource_type: ResourceType) -> &'static [&'static str] {
    match resource_type {
        ResourceType::Cluster => CLUSTER_EXPAND_HINTS,
        ResourceType::Host => HOST_EXPAND_HINTS,
        ResourceType::Datastore => DATASTORE_EXPAND_HINTS,
        ResourceType::Network => NETWORK_EXPAND_HINTS,
        _ => &[],
    }
}