use syn::Macro;

pub struct StdOutputExtractor;

impl syn::visit::Visit<'_> for StdOutputExtractor {
    fn visit_macro(&mut self, node: &Macro) {
        if let Some(ident) = node.path.get_ident() {
            // Determine if the macro is println!
            if ident == "println" || ident == "eprintln" {
                // Parses the contents of the println!
                let macro_tokens = node.tokens.to_string();

                // Extract the contents of the curly brackets
                if let Some(start) = macro_tokens.find('"') {
                    if let Some(end) = macro_tokens.rfind('"') {
                        let format_string = &macro_tokens[start + 1..end];
                        println!("Found format string: {}", format_string);
                    }
                }
            }
        }
    }
}
