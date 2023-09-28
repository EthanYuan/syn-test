use super::extract_contents_in_brackets;
use syn::Expr::{self, Lit};
use syn::Lit::Str;

pub struct ThiserrorExtractor;

impl syn::visit::Visit<'_> for ThiserrorExtractor {
    fn visit_attribute(&mut self, attr: &syn::Attribute) {
        if attr.path().is_ident("error") {
            let precondition: Expr = attr.parse_args().unwrap();
            if let Lit(lit) = precondition {
                if let Str(lit) = lit.lit {
                    let lit = lit.token().to_string();

                    let format_string = extract_contents_in_brackets(lit);
                    if let Some(format_string) = format_string {
                        println!("Found format string: {}", format_string);
                    }
                }
            }
        }
    }
}
