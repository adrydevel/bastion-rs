#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub quorum: usize,
    pub max_kelly: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config { rpc_url: "https://rpc.robinhood-chain.xyz".into(), quorum: 3, max_kelly: 0.25 }
    }
}
