# Redstone codebase
## Implementation of Redstone protocol
### To do
- Every block to include hedder egg chain 1 and prev hash from other chain if heder is chain 1 look chain 3
- Custom save 2 databses
- Wallet look both databases
- When sending txt send to rand chain
- When making new block node will in heder put on what blockchain is going on prev hash from that blockchain + prev hash from other chain egg 2nd.
- When validating chains open newest db from both chains and validate
- In block stuct insert hedder and prev hash other
- Added node saving to database by heder egg if heder 1 save to chain 1 database.
### done
- Simple p2p implemetation of blockchain
- Signing with wallet and sending txt to the blockchain
- Simple node