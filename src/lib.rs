//! Rust Board Support Crate (BSC) for the MCCI Catena 4610
//!
//! More infos about the MCCI Catena 4610:
//! https://store.mcci.com/collections/lorawan-iot-and-the-things-network/products/mcci-catena-4610-integrated-node-for-lorawan-technology
//!
//! This Board Support Crate is in its very early stages. Documentation is
//! sparse and features are basic. Pull requests welcome!

#![no_std]

pub type DebugUsart = hal::serial::USART1;

pub use cmwx1zzabz::hal;

pub use cmwx1zzabz::initialize_radio_irq;
pub use cmwx1zzabz::LongFiBindings;
pub use cmwx1zzabz::RadioIRQ;
pub use cmwx1zzabz::TcxoEn;
