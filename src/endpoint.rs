pub enum Source {
    Iex,
    Sip,
}

pub enum Endpoint {
    Stocks(Source),
    StocksSandbox(Source),
    Crypto,
    News,
}

impl Endpoint {
    pub fn to_url(&self) -> String {
        match self {
            Endpoint::Stocks(source) => match source {
                Source::Iex => "wss://stream.data.alpaca.markets/v2/iex".to_string(),
                Source::Sip => "wss://stream.data.alpaca.markets/v2/sip".to_string(),
            },
            Endpoint::StocksSandbox(source) => match source {
                Source::Iex => "wss://stream.data.sandbox.alpaca.markets/v2/iex".to_string(),
                Source::Sip => "wss://stream.data.sandbox.alpaca.markets/v2/sip".to_string(),
            },
            Endpoint::Crypto => "wss://stream.data.alpaca.markets/v1beta3/crypto/us".to_string(),
            Endpoint::News => "wss://stream.data.alpaca.markets/v1beta1/news".to_string(),
        }
    }
}
