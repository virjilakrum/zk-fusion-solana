# zk-fusion
**Financial Technologies (FinTech)
DeFi Protocols and Token Swap Platform**

```
// declare and export the program's entrypoint
entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // log a message to the blockchain
    msg!("say hi to fusion");

    // gracefully exit the program
    Ok(())
}
```

▸ Project Idea: Develop a platform within the decentralized finance (DeFi) ecosystem where users can exchange their assets at high speed and low cost, while providing enhanced privacy.

**Added Value:** With enhanced privacy and scalability, users can manage their assets more securely and efficiently. zk-Rollups offers the fastest token swap experience on the market thanks to Solana's high transaction speed, while maintaining the privacy of user transactions.

![zk-fusion](https://github.com/virjilakrum/zk-fusion-solana/assets/158029357/c534ce22-0a57-4821-bef9-e863f24a6699)
**High Performance and Low Latency**

▸ The zk-Rollups processor works in a few basic steps: it collects user transactions, organizes them in a Merkle tree, and then sends this group of transactions as a proof to the Solana blockchain. This process works as follows:

• Transaction Aggregation: The processor aggregates a large number of transactions from users. These transactions can be, for example, token transfers or smart contract interactions. The processor aggregates these transactions for processing within a certain timeframe or when the number of collected transactions crosses a certain threshold.

• Packaging into a Merkle Tree: The aggregated transactions are packaged into a Merkle tree. The Merkle tree is created by organizing transactions into a hash tree, which allows each transaction to be uniquely verified. This structure increases data integrity and security, while at the same time significantly reducing data size.

• Sending Proof: The processor generates the root hash of the Merkle tree and a zero-knowledge proof, which proves the correctness of the transactions. This proof is sent to the Solana blockchain, where it is verified by a smart contract. This verification process proves that all transactions sent by the processor are valid and correct, but without revealing the content of the transactions.

![diagram(35)](https://github.com/virjilakrum/zk-fusion-solana/assets/158029357/347ac599-9c3a-4cc1-945c-f1414f494f0d)



▸ The Rust programming language provides high performance, memory safety and concurrency support. Developing the zk-Rollups handler in Rust provides the following advantages:

• Memory Safety: Rust's ownership model prevents common memory management errors such as memory leaks and race conditions. This ensures reliable and error-free operation of the zk-Rollups handler.

• Concurrency: Rust's powerful concurrency features enable the handler to process multiple processes in parallel, resulting in low latencies and high processing capacity.

• Optimization: Rust's compiler optimizations ensure that the handler runs at maximum performance. This is especially critical when processing large amounts of transactions.


**Leveraging Solana's Parallel Processing Capabilities**

▸ Solana is a blockchain platform that offers high transaction speed and scalability. This is due, in part, to Solana's Proof of History (PoH) and parallel processing capabilities. The zk-Rollups processor takes advantage of these features of Solana in the following ways:

• Parallel Processing: Solana can process transactions in parallel, which enables fast verification of the proofs that the processor sends to the Solana blockchain. This leads to a higher processing capacity and lower latency across the system.

• High TPS (Transaction per Second): Solana's high TPS capacity allows the zk-Rollups processor to quickly send batch transactions to the blockchain. This provides scalability, especially for dApps with high user interaction.

## Architecture
![diagram(34)](https://github.com/virjilakrum/zk-fusion-solana/assets/158029357/60f35086-8a0e-405b-9939-800358d30285)

### Stack & Tools

• Rust: We use it to write Solana programs (smart contracts).

• Anchor: We use a framework that facilitates Rust-based smart contract development in Solana.

• zk-SNARKs Libraries: We use them to create zero-knowledge proofs (for example, bellman or halo2 for Rust, libsnark for C++).

• Solana CLI and Solana Web3.js: We use them to interact with the Solana network.

• Next or Vue: We use it for user interface (UI) development.

### Smart Contracts (Anchor-Rust)

• TokenSwap.rs: This contract allows users to send tokens, place buy and sell orders, and match orders.

• zk-Proof Integration: We added a zero-knowledge proof mechanism to transactions to ensure user privacy.

### Frontend Development

• User Interface Design: We designed a web interface where users can easily trade tokens, place trade orders and see swap transactions.

• Web3 Integration: Using the Solana Web3.js library, we connected the frontend to the Solana network and our smart contracts. 

### Testing and Deployment

• Unit Tests: We wrote unit tests for our Smartcontracts using the test tools provided by "Anchor".

• Integration Tests: We tested the integration between frontend and smart contracts.

• Deployment: We will deploy our smart contracts to Solana testnet or mainnet. We will also upload our frontend application to a distributed file system like Celestia or IPFS. On a web server for now.


### Environment and Dependencies:

• Install the Anchor framework using ```cargo install --git https://github.com/project-serum/anchor anchor-cli --locked.```
