//! SigmaOS-bluetooth — Bluetooth stack model.
//!
//! Models Bluetooth adapters, device discovery, GATT services/characteristics,
//! and the overall Bluetooth device manager. Deterministic and std-only
//! (no actual hardware access).

pub mod bluetooth;

pub use bluetooth::{
    BluetoothDevice, BluetoothStack, BluetoothError, BluetoothResult,
    BluetoothAdapter, BluetoothDeviceManager, BLEDevice, BluetoothDeviceStatus,
    BluetoothDeviceType, BluetoothStatistics,
    GattCharacteristic, GattClient, GattDevice, GattService,
};
