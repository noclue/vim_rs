use ratatui::style::{Color, Style};
use vim_rs::types::enums::ManagedEntityStatusEnum;
use ratatui::widgets::Cell;

pub(crate) const ID_COLUMN_WIDTH: u16 = 18;
pub(crate) const STATUS_COLUMN_WIDTH: u16 = 4;

pub(crate) const STATUS: &str = "● ";

pub(crate) fn status_color(status: &ManagedEntityStatusEnum) -> Style {
    match status {
        ManagedEntityStatusEnum::Green => Style::new().fg(Color::Green),
        ManagedEntityStatusEnum::Yellow => Style::new().fg(Color::Yellow),
        ManagedEntityStatusEnum::Red => Style::new().fg(Color::Red),
        ManagedEntityStatusEnum::Gray => Style::new().fg(Color::Gray),
        _ => Style::default(),
    }
}

pub fn format_byte_size(bytes: i64) -> Cell<'static> {
    let bytes_f64 = bytes as f64;

    if bytes_f64 < 0.0 {
        return Cell::from("N/A");
    }

    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;
    const TIB: f64 = GIB * 1024.0;
    const PIB: f64 = TIB * 1024.0;

    let (size, unit) = if bytes_f64 >= PIB {
        (bytes_f64 / PIB, "PiB")
    } else if bytes_f64 >= TIB {
        (bytes_f64 / TIB, "TiB")
    } else if bytes_f64 >= GIB {
        (bytes_f64 / GIB, "GiB")
    } else if bytes_f64 >= MIB {
        (bytes_f64 / MIB, "MiB")
    } else if bytes_f64 >= KIB {
        (bytes_f64 / KIB, "KiB")
    } else {
        (bytes_f64, "B")
    };

    Cell::from(format!("{:.2} {}", size, unit))
}