use std::sync::Arc;

use crate::{
    position_manager_grpc::PositionManagerClosePositionGrpcRequest, AppContext, EngineError,
    EnginePosition,
};

pub fn close_position(
    app: &Arc<AppContext>,
    request: PositionManagerClosePositionGrpcRequest,
) -> Result<Option<EnginePosition>, EngineError> {
    todo!()
}
