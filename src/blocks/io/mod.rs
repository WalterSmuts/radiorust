//! External sources and sinks
//!
//! Blocks in this module will stop working when dropped.
//!
//! The [`audio`] and [`rf`] modules contain blocks that allow accessing
//! hardware audio or radio interfaces.
//! The [`raw`] module allows reading or writing I/Q data as bytes (e.g.
//! from/to files).

pub mod audio;
pub mod raw;
pub mod rf;
