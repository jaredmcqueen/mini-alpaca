pub enum Endpoint {
    StocksProductionIex,
    StocksProductionSip,
    StocksSandboxIex,
    StocksSandboxSip,
    Crypto,
    News,
}

impl Endpoint {
    pub fn to_url(&self) -> String {
        match self {
            Endpoint::Crypto => "wss://stream.data.alpaca.markets/v1beta3/crypto/us".to_string(),
            Endpoint::News => "wss://stream.data.alpaca.markets/v1beta1/news".to_string(),
            Endpoint::StocksProductionIex => "wss://stream.data.alpaca.markets/v2/iex".to_string(),
            Endpoint::StocksProductionSip => "wss://stream.data.alpaca.markets/v2/sip".to_string(),
            Endpoint::StocksSandboxIex => {
                "wss://stream.data.sandbox.alpaca.markets/v2/iex".to_string()
            }
            Endpoint::StocksSandboxSip => {
                "wss://stream.data.sandbox.alpaca.markets/v2/sip".to_string()
            }
        }
    }
}
