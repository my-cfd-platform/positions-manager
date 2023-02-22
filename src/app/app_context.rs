use std::sync::Arc;

use trading_sdk::{ActivePositionsCache, ActivePricesCache, PendingOrdersCache};

use crate::{EnginePosition, EngineBidAsk};

pub struct AppContext {
    pub active_positions_cache: Arc<ActivePositionsCache<EnginePosition>>,
    pub pending_orders_cache: Arc<PendingOrdersCache<EnginePosition>>,
    pub active_prices_cache: Arc<ActivePricesCache<EngineBidAsk>>
}


 