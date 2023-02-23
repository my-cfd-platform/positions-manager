use std::sync::Arc;

use trading_sdk::{ExecutionClosePositionReason, ExecutionPositionBase};

use crate::{AppContext, EngineError, EnginePosition};

pub async fn close_position(
    app: &Arc<AppContext>,
    _: &str,
    position_id: &str,
    close_position_reason: ExecutionClosePositionReason,
    process_id: &str,
) -> Result<EnginePosition, EngineError> {
    let Some(position_to_close) = app
        .active_positions_cache
        .get_position_by_id(&position_id)
        .await else{
        return Err(EngineError::PositionNotFound);
        };

    let Some(active_price) = app.active_prices_cache.get(position_to_close.get_asset_pair()).await else{
        return Err(EngineError::NoLiquidity);
    };

    let Some(removed_position) = app.active_positions_cache.remove_position(
        position_to_close.get_id(),
        position_to_close.get_asset_pair(),
    ).await else{
        return Err(EngineError::PositionNotFound);
    };

    return Ok(removed_position.close_position(
        active_price.as_ref(),
        process_id,
        close_position_reason,
    ));
}
