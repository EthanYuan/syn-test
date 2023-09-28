mod extractors;

use extractors::{
    log_extractor::LogExtractor, std_output_extractor::StdOutputExtractor,
    thiserror_extractor::ThiserrorExtractor,
};

use syn::{parse_file, visit::visit_file};

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
        fn main() {
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
        }
    "#;

    // Parse
    let syntax_tree_std_output = parse_file(std_ouput).unwrap();
    let syntax_tree_log = parse_file(log).unwrap();
    let syntax_tree_thiserror = parse_file(thiserror).unwrap();

    // Extractors
    let mut std_output_extractor = StdOutputExtractor;
    let mut log_extractor = LogExtractor;
    let mut thiserror_extractor = ThiserrorExtractor;

    // Traversing the syntax tree
    visit_file(&mut std_output_extractor, &syntax_tree_std_output);
    visit_file(&mut log_extractor, &syntax_tree_log);
    visit_file(&mut thiserror_extractor, &syntax_tree_thiserror);
}
