# Bloor Trading

This repository will contain the code for the required components of a high frequency trading system built with Rust. 

For now, I'm solely focusing on the crypto market with hopes of potentially expanding in the future.

## Components

### Market Data Processor

- Purpose: To consume and process real-time market data from crypto exchanges.

- Sub-components: Data ingestion mechanisms, data normalization (to standardize data across different exchanges), and efficient storage for quick access and analysis.

### Trading Strategy Module

- Purpose: To define and execute trading strategies based on processed market data.

- Sub-components: Strategy logic (implementing algorithms for strategies like momentum, arbitrage, etc.), backtesting framework (to test strategies on historical data), and a strategy optimizer (for parameter tuning).

### Order Execution System

- Purpose: To execute trades on crypto exchanges with minimal delay.

- Sub-components: Order management (tracking order states and updates), trade execution logic (to handle order placement, modification, and cancellation), and slippage minimization (to reduce the cost impact of executing large orders).

### Risk Management System

- Purpose: To monitor and manage the risks associated with high-frequency trading.

- Sub-components: Position sizing (determining how much to trade), risk limits (setting maximum loss thresholds), and real-time monitoring (for unusual activity or deviations from expected performance).

### Infrastructure and Connectivity

- Purpose: To ensure high-speed, reliable connectivity to crypto exchanges and robust system operation.

- Sub-components: Low-latency networking, direct market access (DMA) where possible, and redundancy (to handle system failures gracefully).

### Performance Monitoring and Reporting

- Purpose: To track the system’s performance and health in real-time.

- Sub-components: Performance metrics (latency, slippage, win/loss ratios, etc.), alerting mechanisms, and logging.

## Architecture

- Core Trading Components (such as the Market Data Processor, Trading Strategy Module, and Order Execution System) could be tightly integrated to minimize latency. These components are highly latency-sensitive and benefit from being in the same process or on the same machine to ensure the fastest possible execution.

- Supporting Services (like Risk Management, Performance Monitoring, and Reporting) could be designed as separate microservices. These components are less sensitive to microsecond-level delays and can benefit from the scalability and flexibility of microservices architecture.