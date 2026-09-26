# aesir-utils-rs

[![GitHub Pages](https://img.shields.io/badge/docs-GitHub%20Pages-blue.svg)](https://bhubbard.github.io/aesir-utils-rs/)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Rust: 2024 Edition](https://img.shields.io/badge/Rust-2024%20Edition-black?logo=rust)](https://www.rust-lang.org)

> Pure Rust port of FiveM safezone calculation, aspect ratio distortion correction, and UI anchor positioning engine (`manups4e/aesir_utils`).

Interactive Simulator Demo: **[bhubbard.github.io/aesir-utils-rs](https://bhubbard.github.io/aesir-utils-rs/)**

---

## Features

- 🎯 **Mathematical Safezone Engine**:
  - Safezone scalar $S \in [0.8, 1.0]$.
  - Margins: $M_x = \frac{W \cdot (1 - S)}{2}$, $M_y = \frac{H \cdot (1 - S)}{2}$.
  - Safe area rectangle: $[M_x, M_y, W - M_x, H - M_y]$.
  - Two-way conversions between screen pixels, raw NDC $[0, 1]$, and safezone-relative NDC.
- 📐 **Aspect Ratio Classification & Correction**:
  - Precision detection: `16:9`, `16:10`, `21:9 Ultrawide`, `32:9 Super Ultrawide`, `4:3 Classic`, `5:4 Classic`.
  - Aspect compensation factor $C_x = \frac{\alpha_{\text{ref}}}{\alpha_{\text{current}}}$ prevents circular HUDs (minimaps, weapon wheels, crosshairs) from stretching into ovals on ultrawide monitors.
  - Complete Letterbox and Pillarbox viewport fitting.
- ⚓ **9-Way Anchor Positioning**:
  - `TopLeft`, `TopCenter`, `TopRight`, `CenterLeft`, `Center`, `CenterRight`, `BottomLeft`, `BottomCenter`, `BottomRight`.
  - Automatic inward padding and alignment pivot support.
- 🖥️ **Virtual 1080p Reference Canvas**:
  - Design once at 1920×1080, scale cleanly to 720p, 1440p, 4K, 8K, or arbitrary display sizes.
  - Scale modes: `UniformFit`, `UniformFill`, `MatchWidth`, `MatchHeight`, `Stretch`.

---

## Mathematical Foundations

### Safezone Perimeter

Given resolution $(W, H)$ and safezone scalar $S \in [0.8, 1.0]$:

$$M_x = \frac{W \cdot (1 - S)}{2}, \quad M_y = \frac{H \cdot (1 - S)}{2}$$

$$\text{SafeRect} = [M_x, \, M_y, \, W - M_x, \, H - M_y]$$

### Aspect Ratio Minimap Distortion Prevention

On ultrawide monitors (21:9 or 32:9), UI elements rendered with naive NDC UV coordinates stretch horizontally. `aesir-utils-rs` computes the compensation factor:

$$C_x = \frac{\alpha_{\text{16:9}}}{\alpha_{\text{monitor}}} = \frac{16 / 9}{W / H}$$

Multiply the horizontal size of circular HUD elements by $C_x$ to preserve strict 1:1 circle geometry across all monitor form factors.

---

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
aesir-utils-rs = { git = "https://github.com/bhubbard/aesir-utils-rs" }
glam = "0.29"
```

### Positioning a Bottom-Left Minimap

```rust
use aesir_utils_rs::{Anchor, AnchorPositioner, Safezone, VirtualCanvas};
use glam::Vec2;

fn main() {
    // 1440p monitor with 90% broadcast safezone
    let screen = Vec2::new(2560.0, 1440.0);
    let safezone = Safezone::new(0.90);
    let safe_rect = safezone.compute_rect(screen);

    // Baseline 1080p minimap: 280x200 px
    let canvas = VirtualCanvas::default();
    let minimap_size = canvas.scale_size(Vec2::new(280.0, 200.0), screen);

    // Anchor at Bottom-Left with 20px inward margin
    let minimap_pos = AnchorPositioner::position_inward(
        Anchor::BottomLeft,
        minimap_size,
        canvas.scale_size(Vec2::new(20.0, 20.0), screen),
        None,
        &safe_rect,
    );

    println!("Minimap screen position: {:?}", minimap_pos);
}
```

### Aspect Ratio Detection & Ultrawide Minimap Scaling

```rust
use aesir_utils_rs::AspectRatio;
use glam::Vec2;

let ultrawide_res = Vec2::new(3440.0, 1440.0);
let aspect = AspectRatio::from_vec2(ultrawide_res);

println!("Aspect ratio: {:.2}:1", aspect.ratio());
println!("Category: {:?}", aspect.category()); // AspectRatioCategory::Ultrawide21_9

let comp_factor = aspect.compensation_factor_16_9(); // ~0.744
let circular_radius = 80.0;
let element_width = circular_radius * 2.0 * comp_factor;
let element_height = circular_radius * 2.0;
```

---

## Interactive Simulator

Explore the live web simulator at [bhubbard.github.io/aesir-utils-rs](https://bhubbard.github.io/aesir-utils-rs/):
- **Live Safezone Slider**: Dynamic preview of margin calculation and bounds.
- **Resolution Switcher**: 1080p, 1440p, 4K, 21:9 Ultrawide, 32:9 Super Ultrawide, 4:3 CRT.
- **Distortion Comparison**: Toggle between uncompensated oval stretch and compensated true circle radar geometry.
- **Viewport Fit**: Live visualization of letterbox and pillarbox rendering.

---

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.
