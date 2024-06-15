

pub trait Printer {
    fn println(&mut self, s: &str);
    fn newline(&mut self);
    fn print(&mut self, s: &str);
    fn printIndent(&mut self);
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
    Tab,
}

impl StdoutPrinter {
    /// Create a new printer. By default uses 4 spaces for indentation.
    pub fn new(indent_type: Option<IndentType>, indent_size: Option<usize>) -> Self {
        let indent_string = match indent_type.unwrap_or(IndentType::Space) {
            IndentType::Space => " ".repeat(indent_size.unwrap_or(4)),
            IndentType::Tab => "\t".repeat(indent_size.unwrap_or(1)),
        };
        Self { indent: 0, indent_string }
    }
}
impl Printer for StdoutPrinter{
    /// Print a string with a newline.
    fn println(&mut self, s: &str) {
        self.printIndent();
        self.print(s);
        self.newline();
    }

    /// Print a newline.
    fn newline(&mut self) {
        println!();
    }

    /// Print a string.
    fn print(&mut self, s: &str) {
        print!("{}", s);
    }

    /// Print the current indentation.
    fn printIndent(&mut self) {
        for _ in 0..self.indent {
            print!("{}", self.indent_string);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer() {
        let mut printer = StdoutPrinter::new(None, None);
        assert_eq!(printer.indent, 0);
        assert_eq!(printer.indent_string, "    ");
        printer.println("Hello, world!");
        assert_eq!(printer.indent, 0);
        printer.indent();
        printer.println("Indented");
        assert_eq!(printer.indent, 1);
        printer.dedent();
        printer.println("Dedented");
        assert_eq!(printer.indent, 0);
    }
}