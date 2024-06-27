// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::significant_drop_tightening, clippy::needless_pass_by_value)]

use agilent6612c::{device::Agilent6612c, params::ConnectionParameters};
use std::sync::Mutex;
use tauri::State;

type GlobalDeviceMutex = Mutex<Option<Agilent6612c>>;

macro_rules! stringify_error {
    ($e: expr) => {
        $e.map_err(|e| e.to_string())
    };
}

#[tauri::command(async)]
fn get_serial_ports() -> Result<Vec<String>, String> {
    let ports = serialport::available_ports().map_err(|e| e.to_string())?;

    let names = ports
        .into_iter()
        .map(|port| port.port_name)
        .collect::<Vec<String>>();

    Ok(names)
}

#[tauri::command]
fn connect(port: String, device: State<'_, GlobalDeviceMutex>) -> Result<(), String> {
    let connection = Agilent6612c::new(port, ConnectionParameters::default(), None)
        .map_err(|error| error.to_string())?;

    device.lock().unwrap().replace(connection);

    Ok(())
}

#[tauri::command]
fn disconnect(device: State<'_, GlobalDeviceMutex>) {
    device.lock().unwrap().take();
}

#[tauri::command(async)]
fn get_device_info(device: State<'_, GlobalDeviceMutex>) -> Result<(String, String), String> {
    let mut slot = device.lock().unwrap();
    let device = slot.as_mut().unwrap();

    let hw_name = stringify_error!(device.hwinfo())?;
    let fw_ver = stringify_error!(device.firmware_version())?;

    Ok((hw_name, fw_ver))
}

#[tauri::command(async)]
fn get_output_settings(device: State<'_, GlobalDeviceMutex>) -> Result<(f32, f32), String> {
    let mut slot = device.lock().unwrap();
    let device = slot.as_mut().unwrap();

    let voltage = stringify_error!(device.output_voltage())?;
    let current = stringify_error!(device.output_current())?;

    Ok((voltage, current))
}

#[tauri::command(async)]
fn get_output_state(device: State<'_, GlobalDeviceMutex>) -> Result<(f32, f32, bool), String> {
    let mut slot = device.lock().unwrap();
    let device = slot.as_mut().unwrap();

    let voltage = stringify_error!(device.measure_voltage())?;
    let current = stringify_error!(device.measure_current())?;
    let enabled = stringify_error!(device.output())?;

    Ok((voltage, current, enabled))
}

#[tauri::command]
fn set_output_state(device: State<'_, GlobalDeviceMutex>, enabled: bool) -> Result<(), String> {
    let mut slot = device.lock().unwrap();
    let device = slot.as_mut().unwrap();

    stringify_error!(device.set_output(enabled))
}

#[tauri::command(async)]
fn get_ocp_state(device: State<'_, GlobalDeviceMutex>) -> Result<bool, String> {
    let mut slot = device.lock().unwrap();
    let device = slot.as_mut().unwrap();

    stringify_error!(device.ocp())
}

#[tauri::command]
fn set_ocp_state(device: State<'_, GlobalDeviceMutex>, enabled: bool) -> Result<(), String> {
    let mut slot = device.lock().unwrap();
    let device = slot.as_mut().unwrap();

    stringify_error!(device.set_ocp(enabled))
}

#[tauri::command]
fn set_output_voltage(device: State<'_, GlobalDeviceMutex>, value: f32) -> Result<(), String> {
    let mut slot = device.lock().unwrap();
    let device = slot.as_mut().unwrap();

    stringify_error!(device.set_output_voltage(value))
}

#[tauri::command]
fn set_current_limit(device: State<'_, GlobalDeviceMutex>, value: f32) -> Result<(), String> {
    let mut slot = device.lock().unwrap();
    let device = slot.as_mut().unwrap();

    stringify_error!(device.set_output_current(value))
}

fn main() {
    let device = GlobalDeviceMutex::new(None);

    tauri::Builder::default()
        .manage(device)
        .invoke_handler(tauri::generate_handler![
            get_serial_ports,
            connect,
            disconnect,
            get_device_info,
            get_output_settings,
            get_output_state,
            set_output_state,
            get_ocp_state,
            set_ocp_state,
            set_output_voltage,
            set_current_limit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
