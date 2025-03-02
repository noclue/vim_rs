use std::fmt::{Display, Formatter};
use std::io::Write;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

// Result is a type alias for handling errors.
pub type Result<T> = std::result::Result<T, Error>;
pub trait Printer {
    fn println(&mut self, s: &str) -> Result<()>;
    fn newline(&mut self) -> Result<()>;
    fn print(&mut self, s: &str) -> Result<()>;
    fn print_indent(&mut self) -> Result<()>;
    fn indent(&mut self);
    fn dedent(&mut self);
}

/// Source code printer. Maintains indentation level.
pub struct StdoutPrinter {
    indent: usize,
    indent_string: String,
}

pub enum IndentType {
    Space,
    #[allow(dead_code)]
    Tab,
}

impl StdoutPrinter {
    /// Create a new printer. By default uses 4 spaces for indentation.
    #[allow(dead_code)]
    pub fn new(indent_type: Option<IndentType>, indent_size: Option<usize>) -> Self {
        let indent_string = match indent_type.unwrap_or(IndentType::Space) {
            IndentType::Space => " ".repeat(indent_size.unwrap_or(4)),
            IndentType::Tab => "\t".repeat(indent_size.unwrap_or(1)),
        };
        Self {
            indent: 0,
            indent_string,
        }
    }
}

impl Printer for StdoutPrinter {
    /// Print a string with a newline.
    fn println(&mut self, s: &str) -> Result<()> {
        self.print_indent()?;
        self.print(s)?;
        self.newline()?;
        Ok(())
    }

    /// Print a newline.
    fn newline(&mut self) -> Result<()> {
        println!();
        Ok(())
    }

    /// Print a string.
    fn print(&mut self, s: &str) -> Result<()> {
        print!("{}", s);
        Ok(())
    }

    /// Print the current indentation.
    fn print_indent(&mut self) -> Result<()> {
        for _ in 0..self.indent {
            print!("{}", self.indent_string);
        }
        Ok(())
    }

    /// Increase the indentation level.
    fn indent(&mut self) {
        self.indent += 1;
    }

    /// Decrease the indentation level.
    fn dedent(&mut self) {
        self.indent -= 1;
    }
} // impl Printer

/// A printer that writes to a file.
pub struct FilePrinter {
    indent: usize,
    indent_string: String,
    file: std::fs::File,
}

impl FilePrinter {
    /// Create a new printer that writes to a file. By default uses 4 spaces for indentation.
    pub fn new(
        file: std::fs::File,
        indent_type: Option<IndentType>,
        indent_size: Option<usize>,
    ) -> Self {
        let indent_string = match indent_type.unwrap_or(IndentType::Space) {
            IndentType::Space => " ".repeat(indent_size.unwrap_or(4)),
            IndentType::Tab => "\t".repeat(indent_size.unwrap_or(1)),
        };
        Self {
            indent: 0,
            indent_string,
            file,
        }
    }
}

impl Printer for FilePrinter {
    /// Print a string with a newline.
    fn println(&mut self, s: &str) -> Result<()> {
        self.print_indent()?;
        self.print(s)?;
        self.newline()?;
        Ok(())
    }

    /// Print a newline.
    fn newline(&mut self) -> Result<()> {
        writeln!(self.file)?;
        Ok(())
    }

    /// Print a string.
    fn print(&mut self, s: &str) -> Result<()> {
        write!(self.file, "{}", s)?;
        Ok(())
    }

    /// Print the current indentation.
    fn print_indent(&mut self) -> Result<()> {
        for _ in 0..self.indent {
            write!(self.file, "{}", self.indent_string)?;
        }
        Ok(())
    }

    /// Increase the indentation level.
    fn indent(&mut self) {
        self.indent += 1;
    }

    /// Decrease the indentation level.
    fn dedent(&mut self) {
        self.indent -= 1;
    }
} // impl Printer

pub struct StringPrinter {
    content: String,
    indent: usize,
    indent_string: String,
}

impl StringPrinter {
    /// Create a new printer that writes to a file. By default uses 4 spaces for indentation.
    #[allow(dead_code)]
    pub fn new(indent_type: Option<IndentType>, indent_size: Option<usize>) -> Self {
        let indent_string = match indent_type.unwrap_or(IndentType::Space) {
            IndentType::Space => " ".repeat(indent_size.unwrap_or(4)),
            IndentType::Tab => "\t".repeat(indent_size.unwrap_or(1)),
        };
        Self {
            content: "".into(),
            indent: 0,
            indent_string,
        }
    }
}

impl Display for StringPrinter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)?;
        Ok(())
    }
}

impl Printer for StringPrinter {
    /// Print a string with a newline.
    fn println(&mut self, s: &str) -> Result<()> {
        self.print_indent()?;
        self.print(s)?;
        self.newline()?;
        Ok(())
    }

    /// Print a newline.
    fn newline(&mut self) -> Result<()> {
        self.content.push('\n');
        Ok(())
    }

    /// Print a string.
    fn print(&mut self, s: &str) -> Result<()> {
        self.content.push_str(s);
        Ok(())
    }

    /// Print the current indentation.
    fn print_indent(&mut self) -> Result<()> {
        for _ in 0..self.indent {
            self.content.push_str(&self.indent_string);
        }
        Ok(())
    }

    /// Increase the indentation level.
    fn indent(&mut self) {
        self.indent += 1;
    }

    /// Decrease the indentation level.
    fn dedent(&mut self) {
        self.indent -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer() {
        let mut printer = StdoutPrinter::new(None, None);
        assert_eq!(printer.indent, 0);
        assert_eq!(printer.indent_string, "    ");
        printer.println("Hello, world!").unwrap();
        assert_eq!(printer.indent, 0);
        printer.indent();
        printer.println("Indented").unwrap();
        assert_eq!(printer.indent, 1);
        printer.dedent();
        printer.println("Dedented").unwrap();
        assert_eq!(printer.indent, 0);
    }

    #[test]
    fn test_indent_type() {
        let mut printer = StdoutPrinter::new(Some(IndentType::Tab), Some(2));
        assert_eq!(printer.indent, 0);
        assert_eq!(printer.indent_string, "\t\t");
        printer.println("Hello, world!").unwrap();
        assert_eq!(printer.indent, 0);
        printer.indent();
        printer.println("Indented").unwrap();
        assert_eq!(printer.indent, 1);
        printer.dedent();
        printer.println("Dedented").unwrap();
        assert_eq!(printer.indent, 0);
    }
}
