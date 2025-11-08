/// Arial font glyph advance widths
/// Used for TOC hierarchy detection via line width clustering

use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref GLYPH_WIDTHS: HashMap<char, f32> = {
        let mut m = HashMap::new();

        // Basic ASCII
        m.insert(' ', 4.4453125);
        m.insert('!', 4.4453125);
        m.insert('"', 5.6796875);
        m.insert('#', 8.8984375);
        m.insert('$', 8.8984375);
        m.insert('%', 14.2265625);
        m.insert('&', 10.671875);
        m.insert('\'', 3.0546875);
        m.insert('(', 5.328125);
        m.insert(')', 5.328125);
        m.insert('*', 6.2265625);
        m.insert('+', 9.34375);
        m.insert(',', 4.4453125);
        m.insert('-', 5.328125);
        m.insert('.', 4.4453125);
        m.insert('/', 4.4453125);

        // Digits
        m.insert('0', 8.8984375);
        m.insert('1', 7.7228125);
        m.insert('2', 8.8984375);
        m.insert('3', 8.8984375);
        m.insert('4', 8.8984375);
        m.insert('5', 8.8984375);
        m.insert('6', 8.8984375);
        m.insert('7', 8.8984375);
        m.insert('8', 8.8984375);
        m.insert('9', 8.8984375);

        m.insert(':', 4.4453125);
        m.insert(';', 4.4453125);
        m.insert('<', 9.34375);
        m.insert('=', 9.34375);
        m.insert('>', 9.34375);
        m.insert('?', 8.8984375);
        m.insert('@', 16.2421875);

        // Uppercase
        m.insert('A', 10.671875);
        m.insert('B', 10.671875);
        m.insert('C', 11.5546875);
        m.insert('D', 11.5546875);
        m.insert('E', 10.671875);
        m.insert('F', 9.7734375);
        m.insert('G', 12.4453125);
        m.insert('H', 11.5546875);
        m.insert('I', 4.4453125);
        m.insert('J', 8.0);
        m.insert('K', 10.671875);
        m.insert('L', 8.8984375);
        m.insert('M', 13.328125);
        m.insert('N', 11.5546875);
        m.insert('O', 12.4453125);
        m.insert('P', 10.671875);
        m.insert('Q', 12.4453125);
        m.insert('R', 11.5546875);
        m.insert('S', 10.671875);
        m.insert('T', 9.7734375);
        m.insert('U', 11.5546875);
        m.insert('V', 10.671875);
        m.insert('W', 15.1015625);
        m.insert('X', 10.671875);
        m.insert('Y', 10.671875);
        m.insert('Z', 9.7734375);

        m.insert('[', 4.4453125);
        m.insert('\\', 4.4453125);
        m.insert(']', 4.4453125);
        m.insert('^', 7.5078125);
        m.insert('_', 8.8984375);
        m.insert('`', 5.328125);

        // Lowercase
        m.insert('a', 8.8984375);
        m.insert('b', 8.8984375);
        m.insert('c', 8.0);
        m.insert('d', 8.8984375);
        m.insert('e', 8.8984375);
        m.insert('f', 4.15921875);
        m.insert('g', 8.8984375);
        m.insert('h', 8.8984375);
        m.insert('i', 3.5546875);
        m.insert('j', 3.5546875);
        m.insert('k', 8.0);
        m.insert('l', 3.5546875);
        m.insert('m', 13.328125);
        m.insert('n', 8.8984375);
        m.insert('o', 8.8984375);
        m.insert('p', 8.8984375);
        m.insert('q', 8.8984375);
        m.insert('r', 5.328125);
        m.insert('s', 8.0);
        m.insert('t', 4.4453125);
        m.insert('u', 8.8984375);
        m.insert('v', 8.0);
        m.insert('w', 11.5546875);
        m.insert('x', 8.0);
        m.insert('y', 8.0);
        m.insert('z', 8.0);

        m.insert('{', 5.34375);
        m.insert('|', 4.15625);
        m.insert('}', 5.34375);
        m.insert('~', 9.34375);

        // Special characters
        m.insert('®', 10.671875);

        m
    };
}

/// Calculate visual width of text using Arial glyph advance widths
pub fn calculate_width(text: &str) -> f32 {
    text.chars()
        .map(|c| *GLYPH_WIDTHS.get(&c).unwrap_or(&8.8984375)) // Default to average
        .sum()
}
