# Redstone codebase
## Implementation of Redstone protocol
### To do
- [ ] Every block to include hedder egg chain 1 and prev hash from other chain if header is chain 1 look chain 3
- [x]  Custom save 2 databses
- [ ]  Last header check
- [x]  Start node server on both chains
- [x]  Mine func for 1st and 2nd chain
- [ ]  Wallet look both databases
- [ ]  When making new block node will in header put on what blockchain is going on prev hash from that blockchain + prev hash from other chain egg 2nd.
- [ ]  When validating chains open newest db from both chains and validate
- [ ] In block stuct insert hedder and prev hash other
- [ ]  Added node saving to database by header egg if header 1 save to chain 1 database.
- [x] Simple p2p implemetation of blockchain
- [x] Signing with wallet and sending txt to the blockchain
- [x] Simple node
- [x] Block to include header and other hash (just added to block struct)
- [] ADVANCE: Implement everything in one function this will hapen at end insted of usinf eg mine_block_chain2() use mine_block(arg: chain) this will hapen at end as now we are testing features and implementing it for testing.
### struckture
