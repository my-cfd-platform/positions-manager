use chrono::{DateTime, Utc};
use trading_sdk::{
    get_close_price, ExecutionClosePositionReason, ExecutionPendingOrderType, PositionSide,
};

use crate::EngineBidAsk;

#[derive(Debug, Clone)]
pub struct EnginePosition {
    pub position_data: EngineBasePositionData,
    pub state: EnginePositionState,
}

impl EnginePosition {

    pub fn update_sl(&mut self, sl_in_profit: &Option<f64>, sl_in_asset_price: &Option<f64>){
        self.position_data.stop_loss_in_asset_price = sl_in_asset_price.clone();
        self.position_data.stop_loss_in_position_profit = sl_in_profit.clone();
    }

    pub fn update_tp(&mut self, tp_in_profit: &Option<f64>, tp_in_asset_price: &Option<f64>){
        self.position_data.take_profit_in_asset_price = tp_in_asset_price.clone();
        self.position_data.take_profit_in_position_profit = tp_in_profit.clone();
    }

    pub fn close_position(
        mut self,
        close_bid_ask: &EngineBidAsk,
        process_id: &str,
        close_reason: ExecutionClosePositionReason,
    ) -> Self {
        let EnginePositionState::Active(active_state) = self.state else{
            panic!("Can't close position. Position is not active");
        };

        let closed_state = ClosedPositionStates {
            active_state,
            close_price: get_close_price(close_bid_ask, &self.position_data.side),
            close_bid_ask: close_bid_ask.clone(),
            close_process_id: process_id.to_string(),
            close_reason,
        };

        self.state = EnginePositionState::Closed(closed_state);

        return self;
    }

    pub fn get_active_state(&self) -> &ActivePositionState {
        if let EnginePositionState::Active(active_state) = &self.state {
            return active_state;
        }

        panic!("Can't get active data. State is not active");
    }

    pub fn get_pending_state(&self) -> &PendingPositionState {
        if let EnginePositionState::Pending(pending_state) = &self.state {
            return pending_state;
        }

        panic!("Can't get pending data. State is not pending");
    }
}

#[derive(Debug, Clone)]
pub enum EnginePositionState {
    Pending(PendingPositionState),
    Active(ActivePositionState),
    Closed(ClosedPositionStates),
}

impl EnginePositionState {
    pub fn update_profit(&mut self, profit: f64) {
        if let EnginePositionState::Active(active_state) = self {
            active_state.profit = profit;
            return;
        }

        panic!("Can't update profit. Profit can be updated only for active positions");
    }
}

#[derive(Debug, Clone)]
pub struct EngineBasePositionData {
    pub id: String,
    pub account_id: String,
    pub asset_pair: String,
    pub side: PositionSide,
    pub invest_amount: f64,
    pub leverage: f64,
    pub stop_out_percent: f64,
    pub create_process_id: String,
    pub create_date: DateTime<Utc>,
    pub last_update_process_id: String,
    pub last_update_date: DateTime<Utc>,
    pub take_profit_in_position_profit: Option<f64>,
    pub take_profit_in_asset_price: Option<f64>,
    pub stop_loss_in_position_profit: Option<f64>,
    pub stop_loss_in_asset_price: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct PendingPositionState {
    pub desire_price: f64,
    pub pending_order_type: ExecutionPendingOrderType,
}

#[derive(Debug, Clone)]
pub struct ActivePositionState {
    pub open_price: f64,
    pub open_bid_ask: EngineBidAsk,
    pub open_process_id: String,
    pub open_date: DateTime<Utc>,
    pub profit: f64,
    pub pending_state: Option<PendingPositionState>,
}

#[derive(Debug, Clone)]
pub struct ClosedPositionStates {
    pub active_state: ActivePositionState,
    pub close_price: f64,
    pub close_bid_ask: EngineBidAsk,
    pub close_process_id: String,
    pub close_reason: ExecutionClosePositionReason,
}
