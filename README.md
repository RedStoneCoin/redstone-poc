# Redstone codebase
## Implementation of Redstone protocol
This is the offical implemention of the redstone protocol. It is written in rust. Protocol is subject to frequent change and as such no documention exists (however it is in the works) It is currently not ready for usage.
### To do
- [x] Every block to include hedder egg chain 1 and prev hash from other chain if header is chain 1 look chain 2
- [x]  Custom save 2 databses
- [x]  Header Input
- [x]  Start node server on both chains
- [ ]  Syncing network by node
- [ ]  Only synces nodes can make blocks, so if someone does not sync it cant send newest block that prev hash is behiend network
- [x]  Mine func for 1st and 2nd chain
- [x]  Wallet look both databases
- [x]  When making new block node will in header put on what blockchain is going on prev hash from that blockchain + prev hash from other chain egg 2nd.
- [x]  When validating chains open newest db from both chains and validate
- [x] In block stuct insert hedder and prev hash other
- [x]  Added node saving to database by header egg if header 1 save to chain 1 database.
- [ ] 2 chain validation (if necesery)
- [ ] block reward (if necesery)
- [ ] P2P: In every function add if chain 2 to use utxo1 insted of utxo 2 so add utxo handler
- [x] Simple p2p implemetation of blockchain
- [x] Signing with wallet and sending txt to the blockchain
- [x] Simple node
- [x] Balance = chain 1 addr balance + chain 2 addr balance
- [x] Block to include header and other hash (just added to block struct)
- [x] ADVANCE: Implement everything in one function this will hapen at end insted of usinf eg mine_block_chain2() use mine_block(arg: chain) this will hapen at end as now we are testing features and implementing it for testing.
- [ ] ADVANCE: Adding encrypted p2p system (this will be added at the end)
