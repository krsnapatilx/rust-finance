# Architecture Overview

## Data Flow
1. **Ingestion**: WebSocket connection to Solana RPC (logsSubscribe).
2. **Parser**: Decodes raw logs into structured `SwapEvent`s.
3. **Feature**: Computes indicators and metrics (EMA, RSI, buy/sell volume) in real-time.
4. **Strategy**: Decision engine (Simple Rules or RL Model) produces an `Action`.
5. **Risk**: Validates action against account limits, drawdown, and confidence thresholds.
6. **Executor**: Translates action into instructions, signs, and sends transaction.
Outlining Core Components

I'm now zeroing in on the architecture. It seems necessary to define the initial skeleton and interactions for the "health-check / fastest-node selector" in crates/relay and the "web-dashboard scaffold" in crates/web-dashboard. This involves outlining core data structures and potential API endpoints.

Implementing Persistence & Ingestion

I'm now focusing on implementing the SQLite persistence writer within crates/persistence and robust ingestion with reconnection & replay logic in crates/ingestion. The idea is to create the necessary files and update the Cargo.toml files accordingly. The plan is to proceed component-by-component, which includes updating the workspace dependencies and adding required code.

Defining Workspace Integration

I'm now clarifying the workspace management. It seems creating a new directory and Cargo.toml isn't enough; I must update the root Cargo.toml to recognize the new components. I'm focusing on defining crates/persistence and crates/ingestion and how they'll be integrated into the workspace. I'll need to create the files and update the Cargo.toml files accordingly. Specifically, I'm working on the persistence crate, starting with its file creation, and ingestion improvements next.

Analyzed
c:\Users\ashut\OneDrive\Documents\rust trading terminal\rl-trading-bot\crates
Analyzed
Cargo.toml#L1-28
Generating

## Concurrency Model
- **Ingestion**: Async (Tokio).
- **Processing**: Synchronous hot-path threads communicating via `crossbeam-channel`.
- **Execution**: Async (Tokio) to handle network latency.

## Performance Tuning
- `target-cpu=native` for SIMD optimizations.
- `LTO = true` for inter-crate optimizations.
- `panic = abort` to minimize binary size and overhead.
- Bounded channels to prevent backpressure issues and OOM.
