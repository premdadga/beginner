mod asset;
mod portfolio;

use asset::Asset;
use portfolio::Portfolio;
use std::io::{self, Write};

fn main() {
    let mut portfolio = Portfolio::new("portfolio".to_string());

    loop {
        println!("\n=== Crypto Portfolio Tracker ===");
        println!("1. Add Asset");
        println!("2. Remove Asset");
        println!("3. Update Prices");
        println!("4. View Portfolio");
        println!("5. View Asset Details");
        println!("6. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => add_asset(&mut portfolio),
            "2" => remove_asset(&mut portfolio),
            "3" => update_prices(&mut portfolio),
            "4" => view_portfolio(&portfolio),
            "5" => view_asset_details(&portfolio),
            "6" => {
                println!("Thanks for using Portfolio Tracker!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn add_asset(portfolio: &mut Portfolio) {
    println!("\n--- Add New Asset ---");

    print!("Symbol (e.g., BTC, ETH): ");
    io::stdout().flush().unwrap();
    let mut symbol = String::new();
    io::stdin().read_line(&mut symbol).unwrap();
    let symbol = symbol.trim().to_uppercase();

    print!("Quantity: ");
    io::stdout().flush().unwrap();
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str).unwrap();
    let quantity: f64 = match quantity_str.trim().parse() {
        Ok(q) => q,
        Err(_) => {
            println!("Invalid quantity!");
            return;
        }
    };

    print!("Purchase price: $");
    io::stdout().flush().unwrap();
    let mut price_str = String::new();
    io::stdin().read_line(&mut price_str).unwrap();
    let price: f64 = match price_str.trim().parse() {
        Ok(p) => p,
        Err(_) => {
            println!("Invalid price!");
            return;
        }
    };

    let asset = Asset::new(symbol, quantity, price);
    portfolio.add_asset(asset);
    println!("Asset added successfully!");
}

fn view_portfolio(portfolio: &Portfolio) {
    println!("\n--- Portfolio Overview ---");

    let assets = portfolio.list_assets();
    if assets.is_empty() {
        println!("No assets in portfolio.");
        return;
    }

    println!(
        "{:<8} {:<12} {:<12} {:<12} {:<12}",
        "Symbol", "Quantity", "Avg Cost", "Current", "Value"
    );
    println!("{}", "-".repeat(60));

    for asset in assets {
        println!(
            "{:<8} {:<12.4} ${:<11.2} ${:<11.2} ${:<11.2}",
            asset.symbol,
            asset.quantity,
            asset.purchase_price,
            asset.current_price,
            asset.market_value()
        );
    }

    println!("{}", "-".repeat(60));
    println!("Total Portfolio Value: ${:.2}", portfolio.total_value());

    let total_pl = portfolio.total_profit_loss();
    let pl_indicator = if total_pl >= 0.0 { "+" } else { "" };
    println!("Total Profit/Loss: {}{:.2}", pl_indicator, total_pl);
}

// Additional helper functions...
fn remove_asset(portfolio: &mut Portfolio) {
    print!("Enter symbol to remove: ");
    io::stdout().flush().unwrap();
    let mut symbol = String::new();
    io::stdin().read_line(&mut symbol).unwrap();
    let symbol = symbol.trim().to_uppercase();

    match portfolio.remove_asset(&symbol) {
        Some(asset) => println!("Removed {} from portfolio", asset.symbol),
        None => println!("Asset {} not found", symbol),
    }
}

fn update_prices(portfolio: &mut Portfolio) {
    // Simulate price updates
    let price_updates = vec![
        ("BTC".to_string(), 45000.0),
        ("ETH".to_string(), 3200.0),
        ("SOL".to_string(), 120.0),
    ];

    portfolio.update_all_prices(&price_updates);
    println!("Prices updated!");
}

fn view_asset_details(portfolio: &Portfolio) {
    print!("Enter symbol: ");
    io::stdout().flush().unwrap();
    let mut symbol = String::new();
    io::stdin().read_line(&mut symbol).unwrap();
    let symbol = symbol.trim().to_uppercase();

    match portfolio.get_asset(&symbol) {
        Some(asset) => {
            println!("\n--- {} Details ---", asset.symbol);
            println!("Quantity: {:.4}", asset.quantity);
            println!("Average Cost: ${:.2}", asset.purchase_price);
            println!("Current Price: ${:.2}", asset.current_price);
            println!("Market Value: ${:.2}", asset.market_value());
            println!(
                "Profit/Loss: ${:.2} ({:.1}%)",
                asset.profit_loss(),
                asset.profit_loss_percentage()
            );
        }
        None => println!("Asset {} not found", symbol),
    }
}
