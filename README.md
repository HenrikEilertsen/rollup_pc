# Rollup PoC (Rust)

This is a school project proof of concept for a Layer-2 scaling solution for blockchain systems.

The goal of this project is to explore how rollups can improve blockchain scalability by processing transactions off-chain and only submitting compressed results to the main chain.

Instead of executing every transaction directly on-chain, transactions are collected, processed in batches, and then committed in a simplified form.

---

## CLI Interface

The program provides a simple command-line interface:

- **make tx**  
  Creates and enqueues a new transaction into the current batch.

- **fraud proof**  
  Verifies the correctness of a submitted batch and checks for invalid transactions.

- **print batch**  
  Displays the current batch of transactions that would be submitted on-chain.

---

## Notes


This is not a production system. It is purely an educational implementation to demonstrate the core ideas behind Layer-2 rollups.
