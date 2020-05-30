use tokio::net::TcpListener;
use mysql_async::Pool;
use crate::common::BillConfig;
use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 接受TCP连接
pub(super) async fn accept_connection(
    listener: &mut TcpListener,
    db_pool: &Pool,
    server_config: &BillConfig,
    tx: Sender<u8>,
) {
    let stopped_flag = Arc::new(RwLock::new(false));
    loop {
        let (socket, addr) = match listener.accept().await {
            Ok(value) => value,
            Err(err) => {
                eprintln!("accept client error: {}", err);
                continue;
            }
        };
        super::on_client_connected::on_client_connected(
            socket,
            addr,
            db_pool,
            &server_config,
            &tx,
            stopped_flag.clone(),
        );
    }
}