use std::fmt::Write;

pub fn format_sexp(sexp: &String) -> String {
    format_sexp_indented(sexp, 0)
}

pub fn format_sexp_indented(sexp: &String, initial_indent_level: u32) -> String {
    let mut formatted = String::new();

    let mut indent_level = initial_indent_level;
    let mut has_field = false;
    let mut s_iter = sexp.split(|c| c == ' ' || c == ')');
    while let Some(s) = s_iter.next() {
        if s.is_empty() {
            // ")"
            indent_level -= 1;
            write!(formatted, ")").unwrap();
        } else if s.starts_with('(') {
            if has_field {
                has_field = false;
            } else {
                if indent_level > 0 {
                    writeln!(formatted, "").unwrap();
                    for _ in 0..indent_level {
                        write!(formatted, "  ").unwrap();
                    }
                }
                indent_level += 1;
            }

            // "(node_name"
            write!(formatted, "{}", s).unwrap();

            // "(MISSING node_name" or "(UNEXPECTED 'x'"
            if s.starts_with("(MISSING") || s.starts_with("(UNEXPECTED") {
                let s = s_iter.next().unwrap();
                write!(formatted, " {}", s).unwrap();
            }
        } else if s.ends_with(':') {
            // "field:"
            writeln!(formatted, "").unwrap();
            for _ in 0..indent_level {
                write!(formatted, "  ").unwrap();
            }
            write!(formatted, "{} ", s).unwrap();
            has_field = true;
            indent_level += 1;
        }
    }

    formatted
}
