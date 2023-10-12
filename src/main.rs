mod extractors;

use extractors::{
    clap_extractor::ClapExtractor, log_extractor::LogExtractor,
    std_output_extractor::StdOutputExtractor, thiserror_extractor::ThiserrorExtractor,
};

use syn::visit_mut::{self, VisitMut};
use syn::{
    parse_file, parse_quote, visit::visit_file, visit_mut::visit_file_mut, Expr, Lit, LitInt,
};
use quote::quote;

fn main() {
    // println/eprintln
    let std_ouput = r#"
        fn main() {
            let hello = "Hello, world!";
            println!("Ethan says: {}", hello);
            eprintln!("Customizing consensus parameters for chain spec only works for dev chains.");
        }
    "#;

    // log: error, info, debug, warning, trace
    let log = r#"
        fn main() {
            env_logger::init();
            info!("BlockFilter received exit signal, exit now");
            let block_hash = 255;
            debug!("Latest built block hash {:#x}", block_hash);
            trace!("light-client: new chain root MMR with size = {}", mmr_size);
            warn!("Warning! {}!", warn_description);
            error!("notify update_tx_pool_for_reorg error {}", e);
        }
    "#;

    // thiserror
    let thiserror = r#" 
        #[derive(Error, Debug)]
        pub enum DataStoreError2 {
            #[error("data store disconnected")]
            Disconnect(#[from] std::io::Error),
            #[error("the data for key `{0}` is not available")]
            Redaction(String),
            #[error("invalid header (expected {expected:?}, found {found:?})")]
            InvalidHeader { expected: String, found: String },
            #[error("unknown data store error")]
            Unknown,
        }
    "#;

    // clap
    let clap = r#"
        fn main() { 
            Command::new(BIN_NAME)
                .author("Nervos Core Dev <dev@nervos.org>")
                .about("Nervos CKB - The Common Knowledge Base")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .arg(
                    Arg::new(ARG_CONFIG_DIR)
                        .global(true)
                        .short('C')
                        .value_name("path")
                        .action(clap::ArgAction::Set)
                        .help(
                            "Runs as if ckb was started in <path> instead of the current working directory.",
                        ),
                );
        }
    "#;

    // Parse
    let syntax_tree_std_output = parse_file(std_ouput).unwrap();
    let syntax_tree_log = parse_file(log).unwrap();
    let syntax_tree_thiserror = parse_file(thiserror).unwrap();
    let syntax_tree_clap = parse_file(clap).unwrap();

    // Extractors
    let mut std_output_extractor = StdOutputExtractor;
    let mut log_extractor = LogExtractor;
    let mut thiserror_extractor = ThiserrorExtractor;
    let mut clap_extractor = ClapExtractor;

    // Traversing the syntax tree
    visit_file(&mut std_output_extractor, &syntax_tree_std_output);
    visit_file(&mut log_extractor, &syntax_tree_log);
    visit_file(&mut thiserror_extractor, &syntax_tree_thiserror);
    visit_file(&mut clap_extractor, &syntax_tree_clap);

    // visit mut
    let code = r#"
        fn main() {
            let _ = 999u256;
        }
    "#;

    let mut replace_extractor = BigintReplace;

    let mut syntax_tree_code = parse_file(code).unwrap();
    visit_file_mut(&mut replace_extractor, &mut syntax_tree_code);
    println!("{}", quote!(#syntax_tree_code));

}

struct BigintReplace;

impl VisitMut for BigintReplace {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Lit(expr) = &node {
            if let Lit::Int(int) = &expr.lit {
                if int.suffix() == "u256" {
                    let digits = int.base10_digits();
                    let unsuffixed: LitInt = syn::parse_str(digits).unwrap();
                    *node = parse_quote!(bigint::u256!(#unsuffixed));
                    return;
                }
            }
        }

        // Delegate to the default impl to visit nested expressions.
        visit_mut::visit_expr_mut(self, node);
    }
}
