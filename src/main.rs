#![allow(non_snake_case)]

mod block;
mod blockchain;
mod cli;
mod server;
mod transaction;
mod utxoset;
mod wallets;

#[macro_use]
extern crate log;

pub type Result<T> = std::result::Result<T, failure::Error>;

use crate::cli::Cli;
use env_logger::Env;

fn main() {
    print!("Redstone Node v0.1\n");
    println!("Commands:\n");
    print!(">createwallet\n");
    print!(">createblockchain <address> \n");
    print!(">send <from> <to> <amount> <chain> -m \n");
    print!(">startnode <port>\n");
    print!(">startminer <port> <address>\n");
    print!(">listaddresses\n");
    print!(">reindex\n");



    env_logger::from_env(Env::default().default_filter_or("warning")).init();

    let mut cli = Cli::new();
    if let Err(e) = cli.run() {
        println!("Error: {}", e);
    }
}
