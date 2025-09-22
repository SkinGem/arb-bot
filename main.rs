mod arbitrage_engine;
mod pool_monitor;
mod transaction_manager;
mod config;
mod monitoring;

use anyhow::Result;
use log::{info, error, warn};
use std::env;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

use arbitrage_engine::{ArbitrageEngine, ArbitrageConfig};
use pool_monitor::{PoolMonitor, PoolMonitorBuilder, PoolUpdateEvent};
use transaction_manager::{TransactionManager, TransactionManagerConfig};
use config::BotConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting Solana Arbitrage Bot");

    // Load configuration
    let config = BotConfig::load_from_env()?;
    info!("Configuration loaded successfully");

    // Create communication channels
    let (pool_update_tx, mut pool_update_rx) = mpsc::unbounded_channel::<PoolUpdateEvent>();
    let (trade_tx, mut trade_rx) = mpsc::unbounded_channel();

    // Initialize arbitrage engine
    let mut arbitrage_engine = ArbitrageEngine::new(
        config.arbitrage.clone(),
        config.token_a_mint.clone(),
        config.token_b_mint.clone(),
    );

    // Initialize pool monitor
    let pool_addresses = vec![
        config.a_sol_pool_address.clone(),
        config.b_sol_pool_address.clone(),
        config.a_b_pool_address.clone(),
    ];

    let pool_monitor = PoolMonitorBuilder::new()
        .with_config(config.pool_monitor.clone())
        .with_pool_addresses(pool_addresses)
        .build(pool_update_tx);

    // Initialize transaction manager
    let transaction_manager = TransactionManager::new(
        config.transaction_manager.clone(),
        config.wallet_keypair_path.clone(),
    ).await?;

    // Start pool monitoring task
    let pool_monitor_task = tokio::spawn(async move {
        if let Err(e) = pool_monitor.start_monitoring().await {
            error!("Pool monitor failed: {}", e);
        }
    });

    // Start arbitrage detection task
    let arbitrage_task = tokio::spawn(async move {
        let mut arbitrage_interval = interval(Duration::from_millis(config.arbitrage_check_interval_ms));
        
        loop {
            tokio::select! {
                // Process pool updates
                Some(pool_update) = pool_update_rx.recv() => {
                    arbitrage_engine.update_pool_state(pool_update.pool_state);
                    
                    // Check for arbitrage opportunities after each pool update
                    match arbitrage_engine.check_arbitrage_opportunities() {
                        Ok(opportunities) => {
                            for opportunity in opportunities {
                                info!("Found arbitrage opportunity: {:.4}% profit, {} lamports net", 
                                      opportunity.profit_percentage, opportunity.net_profit_lamports);
                                
                                if let Err(e) = trade_tx.send(opportunity) {
                                    error!("Failed to send trade opportunity: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Error checking arbitrage opportunities: {}", e);
                        }
                    }
                }
                
                // Periodic arbitrage check (fallback)
                _ = arbitrage_interval.tick() => {
                    match arbitrage_engine.check_arbitrage_opportunities() {
                        Ok(opportunities) => {
                            if !opportunities.is_empty() {
                                info!("Periodic check found {} opportunities", opportunities.len());
                                for opportunity in opportunities {
                                    if let Err(e) = trade_tx.send(opportunity) {
                                        error!("Failed to send trade opportunity: {}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Error in periodic arbitrage check: {}", e);
                        }
                    }
                }
            }
        }
    });

    // Start transaction execution task
    let transaction_task = tokio::spawn(async move {
        while let Some(opportunity) = trade_rx.recv().await {
            info!("Executing arbitrage trade with {:.4}% profit", opportunity.profit_percentage);
            
            match transaction_manager.execute_arbitrage_trade(opportunity).await {
                Ok(signature) => {
                    info!("Arbitrage trade executed successfully: {}", signature);
                }
                Err(e) => {
                    error!("Failed to execute arbitrage trade: {}", e);
                }
            }
        }
    });

    // Wait for all tasks to complete (or one to fail)
    tokio::select! {
        result = pool_monitor_task => {
            error!("Pool monitor task completed: {:?}", result);
        }
        result = arbitrage_task => {
            error!("Arbitrage task completed: {:?}", result);
        }
        result = transaction_task => {
            error!("Transaction task completed: {:?}", result);
        }
    }

    Ok(())
}

/// Health check and monitoring
async fn start_health_monitor() -> Result<()> {
    let mut interval = interval(Duration::from_secs(60));
    
    loop {
        interval.tick().await;
        
        // TODO: Implement health checks
        // - Check RPC connection
        // - Check pool data freshness
        // - Check wallet balance
        // - Check system resources
        
        info!("Health check passed");
    }
}

/// Graceful shutdown handler
async fn setup_shutdown_handler() -> Result<()> {
    tokio::signal::ctrl_c().await?;
    info!("Received shutdown signal, gracefully shutting down...");
    
    // TODO: Implement graceful shutdown
    // - Stop accepting new trades
    // - Wait for pending transactions
    // - Save state if needed
    // - Close connections
    
    Ok(())
}
