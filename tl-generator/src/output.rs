pub(crate) struct Output {
    output: String,
    indent_size: usize,
    current_indent: usize,
}

impl Output {
    pub(crate) fn new(indent_size: usize, current_indent: usize) -> Self {
        Self {
            output: String::new(),
            indent_size,
            current_indent,
        }
    }

    pub(crate) fn destruct(self) -> String {
        self.output
    }

    pub(crate) fn write(&mut self, string: &str) {
        self.output.push_str(string);
    }
    
    pub(crate) fn write_line<F: Fn(&mut Self)>(&mut self, f: F) {
        self.write_indent();
        f(self);
        self.write("\n");
    }
    
    pub(crate) fn with_indent<F: Fn(&mut Self)>(&mut self, f: F) {
        self.increase_indent();
        f(self);
        self.decrease_indent();
    }

    fn write_indent(&mut self) {
        for _ in 0..self.current_indent * self.indent_size {
            self.output.push(' ');
        }
    }

    fn increase_indent(&mut self) {
        self.current_indent += 1;
    }

    fn decrease_indent(&mut self) {
        assert!(self.current_indent > 0);
        self.current_indent -= 1;
    }
}
