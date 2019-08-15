//! Rust Board Support Crate (BSC) for the MCCI Catena 4610
//!
//! More infos about the MCCI Catena 4610:
//! https://store.mcci.com/collections/lorawan-iot-and-the-things-network/products/mcci-catena-4610-integrated-node-for-lorawan-technology
//!
//! This Board Support Crate is in its very early stages. Documentation is
//! sparse and features are basic. Pull requests welcome!


#![no_std]


pub use stm32l0xx_hal as hal;
