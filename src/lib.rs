//! # aesir-utils-rs
//!
//! Pure Rust port of FiveM safezone calculation, aspect ratio correction,
//! and UI anchor positioning engine (`manups4e/aesir_utils`).
//!
//! Provides mathematically precise screen safezone geometry, aspect ratio detection
//! (16:9, 16:10, 21:9 Ultrawide, 32:9 Super Ultrawide, 4:3, 5:4), compensation
//! scaling factors for distortion prevention, letterbox/pillarbox viewport fitting,
//! and 1080p virtual canvas anchor positioning.

pub mod anchor;
pub mod aspect;
pub mod safezone;

pub use anchor::{Anchor, AnchorPositioner, ScaleMode, VirtualCanvas};
pub use aspect::{AspectRatio, AspectRatioCategory, ViewportFit, ViewportFitType};
pub use safezone::{SafeRect, Safezone};
