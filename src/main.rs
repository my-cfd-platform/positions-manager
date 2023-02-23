use std::sync::Arc;

use positions_manager::{start_grpc_server, AppContext};

#[tokio::main]
async fn main() {
    let app = AppContext::new();

    let app = Arc::new(app);
    start_grpc_server(app.clone(), 8888).await;

    app.app_states.wait_until_shutdown().await;
}
