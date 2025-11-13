mod block;
mod blockchain;
use blockchain::Blockchain;

fn main() {
    // 4. Create a new blockchain instance
    //    (This will automatically create the Genesis Block)
    let mut chain = Blockchain::new();
    println!("Blockchain created!");

    // 5. Add a few new blocks with some test data
    println!("Mining block 1...");
    chain.mine_block(vec!["Transaction Data 1".to_string()]);

    println!("Mining block 2...");
    chain.mine_block(vec!["Transaction Data 2A".to_string(), "Transaction Data 2B".to_string()]);

    // 6. Print the entire blockchain to the console
    println!("\nFull Blockchain:\n{:#?}", chain);
}