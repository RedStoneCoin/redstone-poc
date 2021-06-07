use crate::server::*;
fn new(){
    println!("Redstone RPC");
    let bc = Blockchain::new()?;
    let bc1 = Blockchain::new2()?;
    let port = 3001;
    let utxo_set = UTXOSet { blockchain: bc };
    let utxo_set1 = UTXOSet { blockchain: bc1 };    
    let utxo = utxo_set;
    let utxo1 = utxo_set1;
    let server = Server::new(port, "",utxo,utxo1)?;
    println!("Started RPC");

    server.start_server(1)?;


}