use std::sync::Arc;

use trading_sdk::get_open_price;

use crate::{
    position_manager_grpc::PositionManagerOpenPositionGrpcRequest, ActivePositionState, AppContext,
    EngineBasePositionData, EngineError, EnginePosition, EnginePositionState,
};

pub async fn open_position(
    app: &Arc<AppContext>,
    request: PositionManagerOpenPositionGrpcRequest,
) -> Result<EnginePosition, EngineError> {
    let active_position = make_active_order_from_request(app, request).await?;
    app.active_positions_cache
        .add_position(active_position.clone())
        .await;
    return Ok(active_position);
}

pub async fn make_active_order_from_request(
    app: &Arc<AppContext>,
    request: PositionManagerOpenPositionGrpcRequest,
) -> Result<EnginePosition, EngineError> {
    let position_data: EngineBasePositionData = (&request).into();

    let Some(active_price) = app.active_prices_cache.get(&request.asset_pair).await else{
        return Err(EngineError::NoLiquidity);
    };

    let state = ActivePositionState {
        open_price: get_open_price(active_price.as_ref(), &position_data.side),
        open_bid_ask: active_price.as_ref().clone(),
        open_process_id: request.process_id,
        open_date: position_data.create_date,
        profit: 0.0,
        pending_state: None,
    };

    let state = EnginePositionState::Active(state);

    return Ok(EnginePosition {
        position_data,
        state,
    });
}
