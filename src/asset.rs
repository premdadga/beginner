#[derive(Clone, Debug)]
pub struct Asset {
    pub symbol: String,
    pub quantity: f64,
    pub purchase_price: f64,
    pub current_price: f64,
    pub purchase_date: String,
}

impl Asset {
    pub fn new(symbol: String, quantity: f64, puchase_price: f64) -> Self {
        Self {
            symbol: symbol,
            quantity: quantity,
            purchase_price: puchase_price,
            current_price: puchase_price,
            purchase_date: "2026-02-23".to_string(),
        }
    }

    pub fn market_value(&self) -> f64 {
        self.quantity * self.current_price
    }

    pub fn profit_loss(&self) -> f64 {
        self.quantity * (self.current_price - self.purchase_price)
    }

    pub fn profit_loss_percentage(&self) -> f64 {
        ((self.current_price - self.purchase_price) / (self.current_price)) / 100.0
    }

    pub fn update_price(&mut self, new_price: f64) {
        self.current_price = new_price;
    }
}
