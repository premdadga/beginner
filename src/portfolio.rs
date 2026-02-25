use crate::asset::Asset;

#[derive(Debug)]
pub struct Portfolio {
    assets: Vec<Asset>,
    name: String,
}

impl Portfolio {
    pub fn new(name: String) -> Self {
        Self {
            assets: Vec::new(),
            name: name,
        }
    }

    pub fn add_asset(&mut self, asset: Asset) {
        if let Some(existing) = self.assets.iter_mut().find(|a| a.symbol == asset.symbol) {
            let total_value = existing.market_value() + asset.market_value();
            existing.quantity += asset.quantity;
            existing.purchase_price = total_value / existing.quantity;
        } else {
            self.assets.push(asset);
        }
    }
    pub fn remove_asset(&mut self, symbol: &str) -> Option<Asset> {
        if let Some(pos) = self.assets.iter().position(|a| a.symbol == symbol) {
            Some(self.assets.remove(pos))
        } else {
            None
        }
    }

    pub fn total_value(&self) -> f64 {
        self.assets.iter().map(|asset| asset.market_value()).sum()
    }

    pub fn total_profit_loss(&self) -> f64 {
        self.assets.iter().map(|asset| asset.profit_loss()).sum()
    }
    pub fn get_asset(&self, symbol: &str) -> Option<&Asset> {
        self.assets.iter().find(|asset| asset.symbol == symbol)
    }
    pub fn get_asset_mut(&mut self, symbol: &str) -> Option<&mut Asset> {
        self.assets.iter_mut().find(|asset| asset.symbol == symbol)
    }

    pub fn list_assets(&self) -> &Vec<Asset> {
        &self.assets
    }
    pub fn update_all_prices(&mut self, price_updates: &[(String, f64)]) {
        for (symbol, new_price) in price_updates {
            if let Some(asset) = self.get_asset_mut(symbol) {
                asset.update_price(*new_price);
            }
        }
    }
}
