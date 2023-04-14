// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use std::time::Duration;
use tokio::time;

use bluetooth::BleHelper;
use manage_thread::create_thread;

mod bluetooth;
mod manage_thread;

/// アドバタイズしているデバイスの一覧を取得する
#[tauri::command]
async fn get_devices() -> Result<Vec<String>, String> {
    let manager = Manager::new().await.unwrap();

    // アダプタの作成
    let result_get_adapters = manager.adapters().await;
    let adapters = match result_get_adapters {
        Ok(adapters) => adapters,
        Err(_) => return Err("Can not get adapters".to_string()),
    };
    let central = adapters.into_iter().nth(0).unwrap();

    // アドバタイズしているデバイスのスキャン
    let start_scan_result = central.start_scan(ScanFilter::default()).await;
    match start_scan_result {
        Ok(_) => (),
        Err(_) => return Err("Failed to start scan".to_string()),
    }
    // 5秒間wait
    time::sleep(Duration::from_secs(5)).await;

    // 取得したデバイスの名称
    let device_name = get_device_name(central).await;

    dbg!(&device_name);
    Ok(device_name)
}

/// デバイスの名称を取得
async fn get_device_name(central: Adapter) -> Vec<String> {
    let mut device_name = vec![];
    for p in central.peripherals().await.unwrap() {
        let properties = p.properties().await.unwrap();
        if let Some(data) = properties {
            if let Some(name) = data.local_name {
                device_name.push(name);
            } else {
                continue;
            }
        } else {
            continue;
        }
    }

    device_name
}

#[tokio::main]
async fn main() {
    // use tauri::async_runtime::block_on;

    // スレッド作成
    let task = create_thread().await;

    tauri::Builder::default()
        .manage(task)
        .invoke_handler(tauri::generate_handler![get_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
