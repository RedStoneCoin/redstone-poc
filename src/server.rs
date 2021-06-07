//! server of Blockchain
#![allow(warnings, unused)]
use super::*;
use crate::block::*;
use crate::transaction::*;
use crate::utxoset::*;
use bincode::{deserialize, serialize};
use failure::format_err;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::*;
use std::thread;
use std::time::Duration;
use crate::blockchain::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Message {
    Addr(Vec<String>),
    Version(Versionmsg),
    Tx(Txmsg),
    GetData(GetDatamsg),
    GetBlock(GetBlocksmsg),
    Inv(Invmsg),
    Block(Blockmsg),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Blockmsg {
    addr_from: String,
    block: Block,
    chain: i32,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetBlocksmsg {
    addr_from: String,
    chain: i32,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetDatamsg {
    addr_from: String,
    kind: String,
    id: String,
    chain: i32,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Invmsg {
    addr_from: String,
    kind: String,
    items: Vec<String>,
    chain: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Txmsg {
    addr_from: String,
    transaction: Transaction,
    chain: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Versionmsg {
    addr_from: String,
    version: i32,
    best_height: i32,
    chain: i32,

}

pub struct Server {
    node_address: String,
    mining_address: String,
    inner: Arc<Mutex<ServerInner>>,
}

struct ServerInner {
    known_nodes: HashSet<String>,
    utxo: UTXOSet,
    blocks_in_transit: Vec<String>,
    mempool: HashMap<String, Transaction>,
    utxo1: UTXOSet,
}

const KNOWN_NODE1: &str = "localhost:3000";
const CMD_LEN: usize = 12;
const VERSION: i32 = 1;

impl Server {
    pub fn new(port: &str, miner_address: &str,utxo: UTXOSet,utxo1: UTXOSet) -> Result<Server> {


        let mut node_set = HashSet::new();


        node_set.insert(String::from(KNOWN_NODE1));
        Ok(Server {
            node_address: String::from("localhost:") + port,
            mining_address: miner_address.to_string(),
            inner: Arc::new(Mutex::new(ServerInner {
                known_nodes: node_set,
                utxo,
                blocks_in_transit: Vec::new(),
                mempool: HashMap::new(),
                utxo1,
            })),
        })
    }

    pub fn start_server(&self,chain: i32) -> Result<()> {
        let server1 = Server {
            node_address: self.node_address.clone(),
            mining_address: self.mining_address.clone(),
            inner: Arc::clone(&self.inner),
        };
        let server2 = Server {
            node_address: self.node_address.clone(),
            mining_address: self.mining_address.clone(),
            inner: Arc::clone(&self.inner),
        };
        println!(
            "Started server at {}, minning address: {}",
            &self.node_address, &self.mining_address
        );

        thread::spawn(move || {
            println!(
                "Started chain 1 check");
            thread::sleep(Duration::from_millis(1000));
            if server1.get_best_height(1)? == -1 {
                server1.request_blocks(1)
            }else {
                server1.send_version(KNOWN_NODE1,1)
            }

            
        });
        thread::spawn(move || {
            println!(
                "Started chain 2 check");
            thread::sleep(Duration::from_millis(1000));
            if server2.get_best_height(2)? == -1 {
                server2.request_blocks(2)
            }else {
                server2.send_version(KNOWN_NODE1,2)
            }
        });
        
        // end
        let listener = TcpListener::bind(&self.node_address).unwrap();
        println!("Default chain: {}",chain);
        println!("Default chain is just a number used so function arguments are not empty!");

        println!("Node listen...");


        for stream in listener.incoming() {
            let stream = stream?;

            let server1 = Server {
                node_address: self.node_address.clone(),
                mining_address: self.mining_address.clone(),
                inner: Arc::clone(&self.inner),
            };

            let server2 = Server {
                node_address: self.node_address.clone(),
                mining_address: self.mining_address.clone(),
                inner: Arc::clone(&self.inner),
            };

            thread::spawn(move || server1.handle_connection(stream,1));

        }

        Ok(())
    }

    pub fn send_transaction(tx: &Transaction, utxo: UTXOSet, chain: i32, utxo1: UTXOSet) -> Result<()> {
        let server = Server::new("7000", "",utxo,utxo1)?;
        server.send_tx(KNOWN_NODE1, tx, chain)?;
        Ok(())
    }

    /* ------------------- inner halp functions ----------------------------------*/

    fn remove_node(&self, addr: &str) {
        self.inner.lock().unwrap().known_nodes.remove(addr);
    }

    fn add_nodes(&self, addr: &str) {
        self.inner
            .lock()
            .unwrap()
            .known_nodes
            .insert(String::from(addr));
    }

    fn get_known_nodes(&self) -> HashSet<String> {
        self.inner.lock().unwrap().known_nodes.clone()
    }

    fn node_is_known(&self, addr: &str) -> bool {
        self.inner.lock().unwrap().known_nodes.get(addr).is_some()
    }

    fn replace_in_transit(&self, hashs: Vec<String>) {
        let bit = &mut self.inner.lock().unwrap().blocks_in_transit;
        bit.clone_from(&hashs);
    }

    fn get_in_transit(&self) -> Vec<String> {
        self.inner.lock().unwrap().blocks_in_transit.clone()
    }

    fn get_mempool_tx(&self, addr: &str) -> Option<Transaction> {
        match self.inner.lock().unwrap().mempool.get(addr) {
            Some(tx) => Some(tx.clone()),
            None => None,
        }
    }

    fn get_mempool(&self) -> HashMap<String, Transaction> {
        self.inner.lock().unwrap().mempool.clone()
    }

    fn insert_mempool(&self, tx: Transaction) {
        self.inner.lock().unwrap().mempool.insert(tx.id.clone(), tx);
    }

    fn clear_mempool(&self) {
        self.inner.lock().unwrap().mempool.clear()
    }

    fn get_best_height(&self,chain: i32) -> Result<i32> {
        match chain {
            1 =>{
                self.inner.lock().unwrap().utxo.blockchain.get_best_height()
            }
            2 =>{
                self.inner.lock().unwrap().utxo1.blockchain.get_best_height()
            }_ => panic!("Unknown chain index!")
        }
    }

    fn get_block_hashs(&self,chain: i32) -> Vec<String> {
        match chain {
            1 =>{
                self.inner.lock().unwrap().utxo.blockchain.get_block_hashs()
            }
            
            2 =>{
                self.inner.lock().unwrap().utxo1.blockchain.get_block_hashs()
            }_ => panic!("Unknown chain index!")
        }
    }

    fn get_block(&self, block_hash: &str) -> Result<Block> {
        self.inner
            .lock()
            .unwrap()
            .utxo
            .blockchain
            .get_block(block_hash)
    }

    fn verify_tx(&self, tx: &Transaction) -> Result<bool> {
        self.inner
            .lock()
            .unwrap()
            .utxo
            .blockchain
            .verify_transacton(tx)
    }

    fn add_block(&self, block: Block,chain: i32) -> Result<()> {
        match chain {
            1 =>{
                self.inner.lock().unwrap().utxo.blockchain.add_block(block)
            }
            
            2 =>{
                self.inner.lock().unwrap().utxo1.blockchain.add_block(block)
            }_ => panic!("Unknown chain index!")
        }
    }

    fn mine_block(&self, txs: Vec<Transaction,>,chain: i32) -> Result<Block> {
        match chain {
            1 =>{
                self.inner.lock().unwrap().utxo.blockchain.mine_block(txs, chain)
            }
            
            2 =>{
                self.inner.lock().unwrap().utxo1.blockchain.mine_block(txs, chain)
            }_ => panic!("Unknown chain index!")
        }
    }

    fn utxo_reindex(&self,chain: i32) -> Result<()> {
        match chain {
            1 =>{
                self.inner.lock().unwrap().utxo.reindex()
            }
            
            2 =>{
                self.inner.lock().unwrap().utxo1.reindex()
            }_ => panic!("Unknown chain index!")
        }
    }

    /* -----------------------------------------------------*/

    fn send_data(&self, addr: &str, data: &[u8],chain: i32) -> Result<()> {
        if addr == &self.node_address {
            return Ok(());
        }
        let mut stream = match TcpStream::connect(addr) {
            Ok(s) => s,
            Err(_) => {
                self.remove_node(addr);
                return Ok(());
            }
        };

        stream.write(data)?;

        println!("data send successfully");
        Ok(())
    }

    fn request_blocks(&self,chain: i32) -> Result<()> {
        for node in self.get_known_nodes() {
            self.send_get_blocks(&node,chain)?
        }
        Ok(())
    }

    fn send_block(&self, addr: &str, b: &Block, chain: i32) -> Result<()> {
        println!("send block data to: {} block hash: {}", addr, b.get_hash());
        let data = Blockmsg {
            addr_from: self.node_address.clone(),
            block: b.clone(),
            chain: chain.clone(),
        };
        let data = serialize(&(cmd_to_bytes("block"), data))?;
        self.send_data(addr, &data,chain)
    }

    fn send_addr(&self, addr: &str,chain: i32) -> Result<()> {
        println!("send address info to: {}", addr);
        let nodes = self.get_known_nodes();
        let data = serialize(&(cmd_to_bytes("addr"), nodes))?;
        self.send_data(addr, &data,chain)
    }

    fn send_inv(&self, addr: &str, kind: &str, items: Vec<String>,chain: i32) -> Result<()> {
        println!(
            "send inv message to: {} kind: {} data: {:?}",
            addr, kind, items
        );
        let data = Invmsg {
            addr_from: self.node_address.clone(),
            kind: kind.to_string(),
            items,
            chain: chain.clone(),
        };
        let data = serialize(&(cmd_to_bytes("inv"), data))?;
        self.send_data(addr, &data,chain)
    }

    fn send_get_blocks(&self, addr: &str,chain: i32) -> Result<()> {
        println!("send get blocks message to: {}", addr);
        let data = GetBlocksmsg {
            addr_from: self.node_address.clone(),
            chain: chain.clone(),
        };
        let data = serialize(&(cmd_to_bytes("getblocks"), data))?;
        self.send_data(addr, &data,chain)
    }

    fn send_get_data(&self, addr: &str, kind: &str, id: &str,chain: i32) -> Result<()> {
        println!(
            "send get data message to: {} kind: {} id: {}",
            addr, kind, id

        );
        let data = GetDatamsg {
            addr_from: self.node_address.clone(),
            kind: kind.to_string(),
            id: id.to_string(),
            chain: chain.clone(),

        };
        let data = serialize(&(cmd_to_bytes("getdata"), data))?;
        self.send_data(addr, &data,chain)
    }

    pub fn send_tx(&self, addr: &str, tx: &Transaction, chain: i32) -> Result<()> {
        println!("send tx to: {} txid: {}", addr, &tx.id);
        let data = Txmsg {
            addr_from: self.node_address.clone(),
            transaction: tx.clone(),
            chain: chain.clone(),
        };
        let data = serialize(&(cmd_to_bytes("tx"), data))?;
        self.send_data(addr, &data, chain)
    }

    fn send_version(&self, addr: &str,chain: i32) -> Result<()> {
        println!("send version info to: {}", addr);
        let data = Versionmsg {
            addr_from: self.node_address.clone(),
            best_height: self.get_best_height(chain)?,
            version: VERSION,
            chain: chain.clone(),
        };
        let data = serialize(&(cmd_to_bytes("version"), data))?;
        self.send_data(addr, &data,chain)
    }

    fn handle_version(&self, msg: Versionmsg,chain: i32) -> Result<()> {
        println!("receive version msg: {:#?}", msg);
        let my_best_height = self.get_best_height(msg.chain)?;
        if my_best_height < msg.best_height {
            self.send_get_blocks(&msg.addr_from,chain)?;
        } else if my_best_height > msg.best_height {
            self.send_version(&msg.addr_from,chain)?;
        }

        self.send_addr(&msg.addr_from,chain)?;

        if !self.node_is_known(&msg.addr_from) {
            self.add_nodes(&msg.addr_from);
        }
        Ok(())
    }

    fn handle_addr(&self, msg: Vec<String>, chain: i32) -> Result<()> {
        println!("receive address msg: {:#?}", msg);
        for node in msg {
            self.add_nodes(&node);
        }
        //self.request_blocks()?;
        Ok(())
    }

    fn handle_block(&self, msg: Blockmsg,chain: i32) -> Result<()> {
        println!(
            "receive block msg: {}, {}",
            msg.addr_from,
            msg.block.get_hash()
        );
        self.add_block(msg.block,msg.chain)?;

        let mut in_transit = self.get_in_transit();
        if in_transit.len() > 0 {
            let block_hash = &in_transit[0];
            self.send_get_data(&msg.addr_from, "block", block_hash,chain)?;
            in_transit.remove(0);
            self.replace_in_transit(in_transit);
        } else {
            self.utxo_reindex(msg.chain)?;
        }

        Ok(())
    }

    fn handle_inv(&self, msg: Invmsg,chain: i32) -> Result<()> {
        println!("receive inv msg: {:#?}", msg);
        if msg.kind == "block" {
            let block_hash = &msg.items[0];
            self.send_get_data(&msg.addr_from, "block", block_hash,chain)?;

            let mut new_in_transit = Vec::new();
            for b in &msg.items {
                if b != block_hash {
                    new_in_transit.push(b.clone());
                }
            }
            self.replace_in_transit(new_in_transit);
        } else if msg.kind == "tx" {
            let txid = &msg.items[0];
            match self.get_mempool_tx(txid) {
                Some(tx) => {
                    if tx.id.is_empty() {
                        self.send_get_data(&msg.addr_from, "tx", txid,chain)?
                    }
                }
                None => self.send_get_data(&msg.addr_from, "tx", txid,chain)?,
            }
        }
        Ok(())
    }

    fn handle_get_blocks(&self, msg: GetBlocksmsg,chain: i32) -> Result<()> {
        println!("receive get blocks msg: {:#?}", msg);
        let block_hashs = self.get_block_hashs(msg.chain);
        self.send_inv(&msg.addr_from, "block", block_hashs,chain)?;
        Ok(())
    }

    fn handle_get_data(&self, msg: GetDatamsg,chain: i32) -> Result<()> {
        println!("receive get data msg: {:#?}", msg);
        if msg.kind == "block" {
            let block = self.get_block(&msg.id)?;
            self.send_block(&msg.addr_from, &block,msg.chain)?;
        } else if msg.kind == "tx" {
            let tx = self.get_mempool_tx(&msg.id).unwrap();
            self.send_tx(&msg.addr_from, &tx,msg.chain)?;
        }
        Ok(())
    }

    fn handle_tx(&self, msg: Txmsg, chain: i32) -> Result<()> {
        println!("receive tx msg: {} {}, chain: {}", msg.addr_from, &msg.transaction.id, msg.chain);
        self.insert_mempool(msg.transaction.clone());

        let known_nodes = self.get_known_nodes();
        if self.node_address == KNOWN_NODE1 {
            for node in known_nodes {
                if node != self.node_address && node != msg.addr_from {
                    println!("Sending inv to other nodes");

                    self.send_inv(&node, "tx", vec![msg.transaction.id.clone()],msg.chain)?;
                }
            }
        } else {
            let mut mempool = self.get_mempool();
            println!("Current mempool: {:#?}", &mempool);
            if mempool.len() >= 1 && !self.mining_address.is_empty() {
                loop {
                    let mut txs = Vec::new();

                    for (_, tx) in &mempool {
                        if self.verify_tx(tx)? {
                            txs.push(tx.clone());
                        }
                    }

                    if txs.is_empty() {
                        return Ok(());
                    }

                    let cbtx =
                        Transaction::new_coinbase(self.mining_address.clone(), String::new())?;
                    txs.push(cbtx);

                    for tx in &txs {
                        mempool.remove(&tx.id);
                    }

                    let new_block = self.mine_block(txs,msg.chain)?;
                    self.utxo_reindex(msg.chain)?;

                    for node in self.get_known_nodes() {
                        if node != self.node_address {
                            self.send_inv(&node, "block", vec![new_block.get_hash()],msg.chain)?;
                        }
                    }

                    if mempool.len() == 0 {
                        break;
                    }
                }
                self.clear_mempool();
            }
        }

        Ok(())
    }

    fn handle_connection(&self, mut stream: TcpStream, chain: i32) -> Result<()> {
        let mut buffer = Vec::new();
        let count = stream.read_to_end(&mut buffer)?;
        println!("Accept request: length {}", count);

        let cmd = bytes_to_cmd(&buffer)?;

        match cmd {
            Message::Addr(data) => self.handle_addr(data,chain)?,
            Message::Block(data) => self.handle_block(data,chain)?,
            Message::Inv(data) => self.handle_inv(data,chain)?,
            Message::GetBlock(data) => self.handle_get_blocks(data,chain)?,
            Message::GetData(data) => self.handle_get_data(data,chain)?,
            Message::Tx(data) => self.handle_tx(data,chain)?,
            Message::Version(data) => self.handle_version(data,chain)?,
        }

        Ok(())
    }
}

fn cmd_to_bytes(cmd: &str) -> [u8; CMD_LEN] {
    let mut data = [0; CMD_LEN];
    for (i, d) in cmd.as_bytes().iter().enumerate() {
        data[i] = *d;
    }
    data
}

fn bytes_to_cmd(bytes: &[u8]) -> Result<Message> {
    let mut cmd = Vec::new();
    let cmd_bytes = &bytes[..CMD_LEN];
    let data = &bytes[CMD_LEN..];
    for b in cmd_bytes {
        if 0 as u8 != *b {
            cmd.push(*b);
        }
    }
    println!("cmd: {}", String::from_utf8(cmd.clone())?);

    if cmd == "addr".as_bytes() {
        let data: Vec<String> = deserialize(data)?;
        Ok(Message::Addr(data))
    } else if cmd == "block".as_bytes() {
        let data: Blockmsg = deserialize(data)?;
        Ok(Message::Block(data))
    } else if cmd == "inv".as_bytes() {
        let data: Invmsg = deserialize(data)?;
        Ok(Message::Inv(data))
    } else if cmd == "getblocks".as_bytes() {
        let data: GetBlocksmsg = deserialize(data)?;
        Ok(Message::GetBlock(data))
    } else if cmd == "getdata".as_bytes() {
        let data: GetDatamsg = deserialize(data)?;
        Ok(Message::GetData(data))
    } else if cmd == "tx".as_bytes() {
        let data: Txmsg = deserialize(data)?;
        Ok(Message::Tx(data))
    } else if cmd == "version".as_bytes() {
        let data: Versionmsg = deserialize(data)?;
        Ok(Message::Version(data))
    } else {
        Err(format_err!("Unknown command in the server"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::wallets::*;

    #[test]
    fn test_cmd() {

    }
}