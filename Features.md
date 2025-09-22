'''# Solana Arbitrage Trading Bot

This project implements a sophisticated arbitrage trading bot on the Solana blockchain that maintains a 1:1 peg between two tokens (Token A and Token B) by exploiting price differences across three liquidity pools:

*   Token A / SOL
*   Token B / SOL
*   Token A / Token B (Meteora)

The bot leverages Jito bundles for atomic transaction execution, providing strong protection against MEV (Maximal Extractable Value) attacks such as front-running and sandwich attacks.

## Features

*   **Real-time Pool Monitoring**: Continuously tracks token reserves and prices in the A/SOL, B/SOL, and A/B pools using both REST APIs and real-time gRPC streams.
*   **Triangular Arbitrage Execution**: Automatically detects and executes triangular arbitrage opportunities to restore the 1:1 peg between Token A and Token B.
*   **Jito Bundle Integration**: Submits all trades as atomic bundles to the Jito block engine, ensuring all-or-nothing execution and MEV protection.
*   **Profitability Engine**: Calculates net profitability of each trade, factoring in swap fees, slippage, Solana transaction costs, and Jito tips.
*   **Advanced Monitoring & Safety**: Includes a circuit breaker to halt trading during abnormal market conditions, comprehensive logging, and Prometheus metrics for performance monitoring.
*   **Configuration Flexibility**: All critical parameters can be configured through environment variables or a dedicated configuration file.

## System Architecture

The bot is built with a modular, multi-threaded architecture in Rust for maximum performance and reliability. It consists of three main components:

1.  **Pool Monitor**: Subscribes to real-time data streams (Yellowstone gRPC) and polls REST APIs (Meteora) to maintain an up-to-date view of the target liquidity pools.
2.  **Arbitrage Engine**: The core logic of the bot, which continuously analyzes pool data to identify and validate profitable arbitrage opportunities.
3.  **Transaction Manager**: Constructs and submits the three-legged arbitrage trades as a single atomic Jito bundle, handles transaction signing, and monitors execution status.

For a more detailed overview of the architecture, please see the [Architecture Documentation](docs/architecture.md).

## Getting Started

### Prerequisites

*   Rust and Cargo (latest stable version)
*   Docker and Docker Compose (for containerized deployment)
*   A Solana wallet keypair with sufficient SOL, Token A, and Token B for trading.
*   An RPC provider subscription (e.g., Helius, QuickNode) with the Yellowstone gRPC add-on enabled.

### Installation

1.  **Clone the repository:**

    ```bash
    git clone <repository-url>
    cd solana-arbitrage-bot
    ```

2.  **Install Rust dependencies:**

    ```bash
    cargo build --release
    ```

### Configuration

1.  **Create an environment file:**

    Copy the `.env.example` file to `.env`:

    ```bash
    cp .env.example .env
    ```

2.  **Edit the `.env` file** with your specific configuration:

    *   `TOKEN_A_MINT`: The mint address of Token A.
    *   `TOKEN_B_MINT`: The mint address of Token B.
    *   `A_SOL_POOL_ADDRESS`: The address of the Token A / SOL liquidity pool.
    *   `B_SOL_POOL_ADDRESS`: The address of the Token B / SOL liquidity pool.
    *   `A_B_POOL_ADDRESS`: The address of the Token A / Token B Meteora pool.
    *   `WALLET_KEYPAIR_PATH`: The absolute path to your Solana wallet keypair file.
    *   `SOLANA_RPC_URL`: Your Solana RPC provider URL.
    *   `YELLOWSTONE_GRPC_ENDPOINT`: Your Yellowstone gRPC endpoint.
    *   `YELLOWSTONE_GRPC_TOKEN`: Your Yellowstone gRPC authentication token.

### Running the Bot

#### Local Execution

To run the bot directly on your local machine:

```bash
cargo run --release
```

#### Docker Deployment

To build and run the bot in a Docker container:

```bash
docker-compose up --build
```

## Monitoring

The bot exposes Prometheus metrics for monitoring key performance indicators, including:

*   Total trades executed
*   Successful and failed trades
*   Total profit and fees paid
*   Circuit breaker state
*   Trade execution latency

For detailed instructions on setting up monitoring and alerting, please see the [Monitoring Documentation](docs/monitoring.md).

## Disclaimer

This software is provided for educational purposes only. Trading cryptocurrencies involves significant risk, and you are solely responsible for any financial losses. The authors and contributors are not liable for any damages or losses arising from the use of this software.
'''
