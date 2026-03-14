# Implementation Plan - RL Trading Bot Fixes

This plan outlines the steps to resolve critical connectivity, performance, and reliability issues in the trading bot.

## Phase 1: Core Pipeline & Connectivity
- [ ] **Daemon Integration**: 
    - Connect `db_tx` to save trades.
    - Wire `web_tx` and `event_bus` to broadcast real-market events.
    - Fix blocking channel calls in async tasks.
- [ ] **Parser Repair**:
    - Update `ParserService` to recognize replayed block formats.
    - Improve error logging for visibility.
- [ ] **Node Selector**:
    - Implement parallel latency measurements.
    - Ensure `Executor` and `Ingestion` actually use the healthiest node.

## Phase 2: Execution Performance
- [ ] **Priority Fees**: Add Compute Budget instructions to `Executor` to ensure transactions land during congestion.
- [ ] **Async Hygiene**: Standardize on `tokio::sync` channels for async tasks to prevent worker starvation.

## Phase 3: Observability
- [ ] **TUI/Web Updates**: Ensure real-time position and latency updates are flowing through the `EventBus`.
