use trading_sdk::{
    ActiveExecutionPosition, ExecutionPendingOrderType, ExecutionPositionBase,
    PendingExecutionOrder, PositionSide,
};

use crate::EnginePosition;

impl ExecutionPositionBase for EnginePosition {
    fn get_id(&self) -> &str {
        &self.position_data.id
    }

    fn get_asset_pair(&self) -> &str {
        &self.position_data.asset_pair
    }

    fn get_side(&self) -> &PositionSide {
        &self.position_data.side
    }

    fn get_volume(&self) -> f64 {
        return self.position_data.leverage * self.position_data.invest_amount;
    }

    fn get_invest_amount(&self) -> f64 {
        self.position_data.invest_amount
    }

    fn get_so_percent(&self) -> f64 {
        self.position_data.stop_out_percent
    }
}

impl ActiveExecutionPosition for EnginePosition {
    fn get_profit(&self) -> f64 {
        let state = self.get_active_state();
        state.profit
    }

    fn get_open_price(&self) -> f64 {
        let state = self.get_active_state();
        state.open_price
    }

    fn get_next_charge_setlement_fee_date(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        None
    }

    fn get_last_charge_setlement_fee_date(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        None
    }

    fn get_charge_setlement_fee_period_in_seconds(&self) -> Option<chrono::Duration> {
        None
    }

    fn get_take_profit_in_order_profit(&self) -> Option<f64> {
        self.position_data.take_profit_in_position_profit
    }

    fn get_take_profit_in_asset_price(&self) -> Option<f64> {
        self.position_data.take_profit_in_asset_price
    }

    fn get_stop_loss_in_order_profit(&self) -> Option<f64> {
        self.position_data.stop_loss_in_position_profit
    }

    fn get_stop_loss_in_asset_price(&self) -> Option<f64> {
        self.position_data.stop_loss_in_asset_price
    }

    fn update_profit(&mut self, profit: f64) {
        self.state.update_profit(profit);
    }
}

impl PendingExecutionOrder for EnginePosition {
    fn get_desired_price(&self) -> f64 {
        let state = self.get_pending_state();
        state.desire_price
    }

    fn get_pending_order_type(&self) -> &ExecutionPendingOrderType {
        let state = self.get_pending_state();
        &state.pending_order_type
    }
}
