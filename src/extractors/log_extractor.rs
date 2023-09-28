use super::extract_contents_in_brackets;
use syn::Macro;

pub struct LogExtractor;

impl syn::visit::Visit<'_> for LogExtractor {
    fn visit_macro(&mut self, node: &Macro) {
        if let Some(ident) = node.path.get_ident() {
            // Determine if the macro is println!
            if ident == "error"
                || ident == "warn"
                || ident == "info"
                || ident == "debug"
                || ident == "trace"
            {
                let lit = node.tokens.to_string();

                let format_string = extract_contents_in_brackets(lit);
                if let Some(format_string) = format_string {
                    println!("Found format string: {}", format_string);
                }
            }
        }
    }
}
