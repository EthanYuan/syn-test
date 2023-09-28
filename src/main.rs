mod log_extractor;
mod std_output_extractor;

use log_extractor::LogExtractor;
use std_output_extractor::StdOutputExtractor;

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

    // 

    // Parse
    let syntax_tree_std_output = parse_file(std_ouput).unwrap();
    let syntax_tree_log = parse_file(log).unwrap();

    // Extractors
    let mut std_output_extractor = StdOutputExtractor;
    let mut log_extractor = LogExtractor;

    // Traversing the syntax tree
    visit_file(&mut std_output_extractor, &syntax_tree_std_output);
    visit_file(&mut log_extractor, &syntax_tree_log);
}
