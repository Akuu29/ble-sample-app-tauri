use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time;

use crate::bluetooth::BleHelper;

/// スレッドの作成
pub async fn create_thread() -> Arc<Mutex<BleHelper>> {
    // デフォルトで可変
    let ble_helper = Arc::new(Mutex::new(BleHelper { peripherals: None }));

    let thread_state = ble_helper.clone();
    tokio::spawn(async move {
        loop {
            {
                // デバイスとのKeep Alive
                // 5分間おきに行う？
                println!("thread");
            }

            // 5秒wait
            time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    thread_state
}
