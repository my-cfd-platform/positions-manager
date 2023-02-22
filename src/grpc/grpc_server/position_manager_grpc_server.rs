use std::sync::Arc;

use crate::{
    open_position,
    position_manager_grpc::{
        position_manager_grpc_service_server::PositionManagerGrpcService,
        PositionManagerClosePositionGrpcRequest, PositionManagerClosedPositionGrpcModel,
        PositionManagerOpenPositionGrpcRequest, PositionManagerOpenPositionGrpcResponse,
        PositionManagerOperationsCodes,
    },
    AppContext,
};

#[derive(Clone)]
pub struct GrpcService {
    pub app: Arc<AppContext>,
}

impl GrpcService {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[tonic::async_trait]
impl PositionManagerGrpcService for GrpcService {
    async fn open_position(
        &self,
        request: tonic::Request<PositionManagerOpenPositionGrpcRequest>,
    ) -> Result<tonic::Response<PositionManagerOpenPositionGrpcResponse>, tonic::Status> {
        let request = request.into_inner();
        let open_position_result = open_position(&self.app, request).await;

        let response = match open_position_result {
            Ok(position) => PositionManagerOpenPositionGrpcResponse {
                positon: Some(position.into()),
                status: PositionManagerOperationsCodes::Ok as i32,
            },
            Err(error) => {
                let grpc_status: PositionManagerOperationsCodes = error.into();
                PositionManagerOpenPositionGrpcResponse {
                    positon: None,
                    status: grpc_status as i32,
                }
            }
        };

        return Ok(tonic::Response::new(response));
    }
    async fn close_position(
        &self,
        request: tonic::Request<PositionManagerClosePositionGrpcRequest>,
    ) -> Result<tonic::Response<PositionManagerClosedPositionGrpcModel>, tonic::Status> {
        todo!()
    }
}
