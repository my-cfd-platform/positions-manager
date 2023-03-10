use trading_sdk::{ExecutionClosePositionReason, PositionSide};

use crate::{
    generate_position_id, get_current_date,
    position_manager_grpc::{
        PositionManagerActivePositionGrpcModel, PositionManagerBidAsk,
        PositionManagerClosePositionReason, PositionManagerClosedPositionGrpcModel,
        PositionManagerOpenPositionGrpcRequest, PositionManagerOperationsCodes,
    },
    EngineBasePositionData, EngineBidAsk, EngineError, EnginePosition, EnginePositionState,
};

impl Into<EngineBasePositionData> for &PositionManagerOpenPositionGrpcRequest {
    fn into(self) -> EngineBasePositionData {
        let date = get_current_date();

        EngineBasePositionData {
            id: generate_position_id(),
            asset_pair: self.asset_pair.clone(),
            side: PositionSide::from(self.side),
            invest_amount: self.invest_amount,
            leverage: self.leverage,
            stop_out_percent: self.stop_out_percent,
            create_process_id: self.process_id.clone(),
            create_date: date.clone(),
            last_update_process_id: self.process_id.clone(),
            last_update_date: date,
            take_profit_in_position_profit: self.tp_in_profit,
            take_profit_in_asset_price: self.tp_in_asset_price,
            stop_loss_in_position_profit: self.sl_in_profit,
            stop_loss_in_asset_price: self.sl_in_asset_price,
            account_id: self.account_id.clone(),
        }
    }
}

impl Into<EngineBasePositionData> for PositionManagerOpenPositionGrpcRequest {
    fn into(self) -> EngineBasePositionData {
        (&self).into()
    }
}

impl Into<PositionManagerActivePositionGrpcModel> for EnginePosition {
    fn into(self) -> PositionManagerActivePositionGrpcModel {
        let data = self.position_data;

        let EnginePositionState::Active(active_state) = self.state else{
            panic!("Position is not active");
        };

        PositionManagerActivePositionGrpcModel {
            id: data.id,
            asset_pair: data.asset_pair,
            side: data.side as i32,
            invest_amount: data.invest_amount,
            leverage: data.leverage,
            stop_out_percent: data.stop_out_percent,
            create_process_id: data.create_process_id,
            create_date_unix_timestamp_milis: data.create_date.timestamp_millis() as u64,
            last_update_process_id: data.last_update_process_id,
            last_update_date: data.last_update_date.timestamp_millis() as u64,
            tp_in_profit: data.take_profit_in_position_profit,
            sl_in_profit: data.stop_loss_in_position_profit,
            tp_in_asset_price: data.take_profit_in_asset_price,
            sl_in_asset_price: data.stop_loss_in_asset_price,
            open_price: active_state.open_price,
            open_bid_ask: Some(active_state.open_bid_ask.into()),
            open_process_id: active_state.open_process_id,
            open_date: active_state.open_date.timestamp_millis() as u64,
            profit: active_state.profit,
        }
    }
}

impl Into<PositionManagerClosePositionReason> for ExecutionClosePositionReason {
    fn into(self) -> PositionManagerClosePositionReason {
        match self {
            ExecutionClosePositionReason::ClientCommand => {
                PositionManagerClosePositionReason::ClientCommand
            }
            ExecutionClosePositionReason::StopOut => PositionManagerClosePositionReason::StopOut,
            ExecutionClosePositionReason::TakeProfit => {
                PositionManagerClosePositionReason::TakeProfit
            }
            ExecutionClosePositionReason::StopLoss => PositionManagerClosePositionReason::StopLoss,
            ExecutionClosePositionReason::ForceClose => {
                PositionManagerClosePositionReason::ForceClose
            }
        }
    }
}

impl Into<PositionManagerClosedPositionGrpcModel> for EnginePosition {
    fn into(self) -> PositionManagerClosedPositionGrpcModel {
        let data = self.position_data;

        let EnginePositionState::Closed(closed_state) = self.state else{
            panic!("Position is not closed");
        };

        let close_position_reason: PositionManagerClosePositionReason =
            closed_state.close_reason.into();

        PositionManagerClosedPositionGrpcModel {
            id: data.id,
            asset_pair: data.asset_pair,
            side: data.side as i32,
            invest_amount: data.invest_amount,
            leverage: data.leverage,
            stop_out_percent: data.stop_out_percent,
            create_process_id: data.create_process_id,
            create_date_unix_timestamp_milis: data.create_date.timestamp_millis() as u64,
            last_update_process_id: data.last_update_process_id,
            last_update_date: data.last_update_date.timestamp_millis() as u64,
            tp_in_profit: data.take_profit_in_position_profit,
            sl_in_profit: data.stop_loss_in_position_profit,
            tp_in_asset_price: data.take_profit_in_asset_price,
            sl_in_asset_price: data.stop_loss_in_asset_price,
            open_price: closed_state.active_state.open_price,
            open_bid_ask: Some(closed_state.active_state.open_bid_ask.into()),
            open_process_id: closed_state.active_state.open_process_id,
            open_date: closed_state.active_state.open_date.timestamp_millis() as u64,
            profit: closed_state.active_state.profit,
            close_price: closed_state.close_price,
            close_bid_ask: Some(closed_state.close_bid_ask.into()),
            close_process_id: closed_state.close_process_id,
            close_reason: close_position_reason as i32,
        }
    }
}

impl Into<PositionManagerBidAsk> for EngineBidAsk {
    fn into(self) -> PositionManagerBidAsk {
        PositionManagerBidAsk {
            asset_pair: self.asset_pair.clone(),
            bid: self.bid,
            ask: self.ask,
            date_time_unix_timestamp_milis: self.datetime.timestamp_millis() as u64,
        }
    }
}

impl Into<PositionManagerOperationsCodes> for EngineError {
    fn into(self) -> PositionManagerOperationsCodes {
        match self {
            EngineError::NoLiquidity => PositionManagerOperationsCodes::NoLiquidity,
            EngineError::PositionNotFound => PositionManagerOperationsCodes::PositionNotFound,
        }
    }
}
