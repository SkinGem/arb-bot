# Deployment Guide

This guide provides step-by-step instructions for deploying the Solana arbitrage trading bot in production environments.

## Prerequisites

Before deploying the bot, ensure you have the following:

*   A VPS or dedicated server with at least 4GB RAM and 2 CPU cores
*   Docker and Docker Compose installed
*   A Solana wallet with sufficient SOL, Token A, and Token B for trading
*   An RPC provider subscription (Helius, QuickNode, etc.) with Yellowstone gRPC enabled
*   Access to Jito Block Engine for bundle submission

## Deployment Options

### Option 1: Docker Deployment (Recommended)

This is the easiest and most reliable deployment method.

#### Step 1: Clone and Configure

```bash
git clone <repository-url>
cd solana-arbitrage-bot
cp .env.example .env
```

#### Step 2: Configure Environment Variables

Edit the `.env` file with your specific configuration:

```bash
nano .env
```

Fill in all required values:
- Token mint addresses
- Pool addresses
- Wallet keypair path
- RPC URLs and credentials
- Trading parameters

#### Step 3: Prepare Wallet

Create a wallet directory and place your keypair file:

```bash
mkdir wallet
cp /path/to/your/keypair.json wallet/
chmod 600 wallet/keypair.json
```

Update the `WALLET_KEYPAIR_PATH` in your `.env` file:
```
WALLET_KEYPAIR_PATH=/app/wallet/keypair.json
```

#### Step 4: Deploy with Docker Compose

```bash
docker-compose up -d
```

This will start:
- The arbitrage bot
- Prometheus for metrics collection
- Grafana for monitoring dashboards

#### Step 5: Verify Deployment

Check that all services are running:

```bash
docker-compose ps
```

Access the monitoring dashboard at `http://your-server:3000` (admin/admin123).

### Option 2: Native Deployment

For advanced users who prefer to run the bot directly on the host system.

#### Step 1: Install Dependencies

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install system dependencies
sudo apt-get update
sudo apt-get install -y build-essential libssl-dev pkg-config
```

#### Step 2: Build the Application

```bash
git clone <repository-url>
cd solana-arbitrage-bot
cargo build --release
```

#### Step 3: Configure Environment

```bash
cp .env.example .env
nano .env  # Configure your settings
```

#### Step 4: Run the Bot

```bash
./target/release/arbitrage-bot
```

## Configuration Parameters

### Required Parameters

| Parameter | Description | Example |
|-----------|-------------|---------|
| `TOKEN_A_MINT` | Token A mint address | `So11111111111111111111111111111111111111112` |
| `TOKEN_B_MINT` | Token B mint address | `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v` |
| `A_SOL_POOL_ADDRESS` | Token A/SOL pool address | `58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2` |
| `B_SOL_POOL_ADDRESS` | Token B/SOL pool address | `7XawhbbxtsRcQA8KTkHT9f9nc6d69UwqCDh6U5EEbEmX` |
| `A_B_POOL_ADDRESS` | Token A/B Meteora pool address | `2QdhepnKRTLjjSqPL1PtKNwqrUkoLee5Gqs8bvZhRdMv` |
| `WALLET_KEYPAIR_PATH` | Path to wallet keypair file | `/app/wallet/keypair.json` |
| `SOLANA_RPC_URL` | Solana RPC endpoint | `https://api.mainnet-beta.solana.com` |

### Optional Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `MIN_PROFIT_THRESHOLD` | `0.005` | Minimum profit percentage (0.5%) |
| `MAX_TRADE_AMOUNT_SOL` | `1000000000` | Maximum trade size in lamports (1 SOL) |
| `SLIPPAGE_TOLERANCE` | `0.01` | Slippage tolerance (1%) |
| `JITO_TIP_LAMPORTS` | `10000` | Jito tip amount |
| `ARBITRAGE_CHECK_INTERVAL_MS` | `1000` | Check interval in milliseconds |

## Security Considerations

### Wallet Security

*   Store your keypair file securely with restricted permissions (`chmod 600`)
*   Use a dedicated wallet for the bot with only necessary funds
*   Regularly monitor wallet balance and transaction history
*   Consider using a hardware wallet for key generation

### Network Security

*   Deploy on a VPS with proper firewall configuration
*   Only expose necessary ports (9090 for metrics, 3000 for Grafana)
*   Use strong passwords for monitoring dashboards
*   Enable SSH key authentication and disable password login

### Operational Security

*   Monitor bot logs regularly for suspicious activity
*   Set up alerts for circuit breaker triggers and failed trades
*   Implement proper backup procedures for configuration and logs
*   Keep the bot software updated with security patches

## Monitoring and Maintenance

### Health Checks

The bot includes built-in health checks that monitor:
- RPC connection status
- Pool data freshness
- Wallet balance
- Circuit breaker state

### Metrics and Alerting

Key metrics to monitor:
- Trade success/failure rates
- Profit and loss tracking
- Execution latency
- Circuit breaker triggers

### Log Management

Logs are stored in the `logs/` directory and include:
- Trade execution details
- Error messages and warnings
- Performance metrics
- Circuit breaker events

### Backup Procedures

Regularly backup:
- Configuration files (`.env`)
- Wallet keypair files
- Trading logs and metrics
- Grafana dashboards and alerts

## Troubleshooting

### Common Issues

**Bot not starting:**
- Check environment variable configuration
- Verify wallet keypair file permissions
- Ensure RPC endpoint is accessible

**No trades executing:**
- Verify pool addresses are correct
- Check minimum profit threshold settings
- Confirm wallet has sufficient balance
- Review circuit breaker status

**High failure rates:**
- Increase slippage tolerance
- Reduce trade size
- Check RPC endpoint performance
- Verify Jito tip amounts

### Support and Maintenance

For ongoing support:
1. Monitor the bot's performance metrics
2. Review logs for errors or warnings
3. Keep the software updated
4. Adjust parameters based on market conditions

## Performance Optimization

### Server Specifications

Recommended minimum specifications:
- **CPU:** 2 cores, 2.4GHz+
- **RAM:** 4GB
- **Storage:** 20GB SSD
- **Network:** Low-latency connection to Solana validators

### Network Optimization

*   Deploy close to Solana validator nodes (US East Coast recommended)
*   Use a high-performance RPC provider with Yellowstone gRPC
*   Configure appropriate timeout and retry settings
*   Monitor network latency to RPC endpoints

### Trading Parameters

*   Start with conservative profit thresholds and trade sizes
*   Gradually optimize based on market conditions and performance
*   Monitor gas costs and adjust Jito tips accordingly
*   Use circuit breaker settings to limit losses during volatile periods
