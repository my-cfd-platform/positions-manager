use std::pin::Pin;

use tonic::codegen::futures_core::Stream;
use trading_sdk::ExecutionClosePositionReason;

use crate::{
    close_position, open_position,
    position_manager_grpc::{
        position_manager_grpc_service_server::PositionManagerGrpcService,
        PositionManagerActivePositionGrpcModel, PositionManagerClosePositionGrpcRequest,
        PositionManagerClosePositionGrpcResponse, PositionManagerGetActivePositionsGrpcRequest,
        PositionManagerOpenPositionGrpcRequest, PositionManagerOpenPositionGrpcResponse,
        PositionManagerOperationsCodes, PositionManagerUpdateSlTpGrpcRequest,
        PositionManagerUpdateSlTpGrpcResponse,
    },
    EnginePosition, GrpcService,
};

#[tonic::async_trait]
impl PositionManagerGrpcService for GrpcService {
    type GetAccountActivePositionsStream = Pin<
        Box<
            dyn Stream<Item = Result<PositionManagerActivePositionGrpcModel, tonic::Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

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
    ) -> Result<tonic::Response<PositionManagerClosePositionGrpcResponse>, tonic::Status> {
        let request = request.into_inner();

        let closed_position = close_position(
            &self.app,
            &request.account_id,
            &request.position_id,
            ExecutionClosePositionReason::ClientCommand,
            &request.process_id,
        )
        .await;

        let response = match closed_position {
            Ok(position) => PositionManagerClosePositionGrpcResponse {
                position: Some(position.into()),
                status: PositionManagerOperationsCodes::Ok as i32,
            },
            Err(error) => {
                let grpc_status: PositionManagerOperationsCodes = error.into();
                PositionManagerClosePositionGrpcResponse {
                    position: None,
                    status: grpc_status as i32,
                }
            }
        };

        return Ok(tonic::Response::new(response));
    }

    async fn get_account_active_positions(
        &self,
        request: tonic::Request<PositionManagerGetActivePositionsGrpcRequest>,
    ) -> Result<tonic::Response<Self::GetAccountActivePositionsStream>, tonic::Status> {
        let request = request.into_inner();
        let account_positions = self
            .app
            .active_positions_cache
            .get_account_active_positions(&request.account_id)
            .await;

        let response: Vec<PositionManagerActivePositionGrpcModel> = account_positions
            .iter()
            .map(|position| {
                let grpc_position: PositionManagerActivePositionGrpcModel = position.clone().into();
                grpc_position
            })
            .collect();

        return my_grpc_extensions::grpc_server::send_vec_to_stream(response, |x| x).await;
    }

    async fn update_sl_tp(
        &self,
        request: tonic::Request<PositionManagerUpdateSlTpGrpcRequest>,
    ) -> Result<tonic::Response<PositionManagerUpdateSlTpGrpcResponse>, tonic::Status> {
        let request = request.into_inner();
        let updated_position = self
            .app
            .active_positions_cache
            .update_position_by_id(&request.position_id, |pos: Option<&mut EnginePosition>| {
                let Some(position_to_update) = pos else{
                    return None;
                };

                position_to_update.update_sl(&request.sl_in_profit, &request.sl_in_asset_price);
                position_to_update.update_tp(&request.tp_in_profit, &request.tp_in_asset_price);

                return Some(position_to_update.clone());
            })
            .await;

        let response = match updated_position {
            Some(position) => PositionManagerUpdateSlTpGrpcResponse {
                position: Some(position.into()),
                status: PositionManagerOperationsCodes::Ok as i32,
            },
            None => PositionManagerUpdateSlTpGrpcResponse {
                position: None,
                status: PositionManagerOperationsCodes::PositionNotFound as i32,
            },
        };
        return Ok(tonic::Response::new(response));
    }
}
