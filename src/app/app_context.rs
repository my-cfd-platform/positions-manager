use std::sync::Arc;

use rust_extensions::AppStates;
use trading_sdk::{ActivePositionsCache, ActivePricesCache, PendingOrdersCache};

use crate::{EngineBidAsk, EnginePosition};

pub struct AppContext {
    pub active_positions_cache: Arc<ActivePositionsCache<EnginePosition>>,
    pub pending_orders_cache: Arc<PendingOrdersCache<EnginePosition>>,
    pub active_prices_cache: Arc<ActivePricesCache<EngineBidAsk>>,
    pub app_states: Arc<AppStates>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            active_positions_cache: Arc::new(ActivePositionsCache::new()),
            pending_orders_cache: Arc::new(PendingOrdersCache::new()),
            active_prices_cache: Arc::new(ActivePricesCache::new()),
            app_states: Arc::new(AppStates::create_initialized()),
        }
    }
}
