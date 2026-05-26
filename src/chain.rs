use crate::config::Config;

/// Thin placeholder for the Robinhood Chain adapter (reads via RPC).
pub struct Chain { pub cfg: Config }

impl Chain {
    pub fn new(cfg: Config) -> Self { Chain { cfg } }
    pub fn rpc(&self) -> &str { &self.cfg.rpc_url }
}
