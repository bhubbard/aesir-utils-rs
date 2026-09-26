use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Safe area bounds represented by top-left (min) and bottom-right (max) coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SafeRect {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl SafeRect {
    /// Creates a new `SafeRect` with the given coordinates.
    #[inline]
    pub const fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    /// Top-left point of the safe area.
    #[inline]
    pub fn min(&self) -> Vec2 {
        Vec2::new(self.min_x, self.min_y)
    }

    /// Bottom-right point of the safe area.
    #[inline]
    pub fn max(&self) -> Vec2 {
        Vec2::new(self.max_x, self.max_y)
    }

    /// Total width of the safe area.
    #[inline]
    pub fn width(&self) -> f32 {
        (self.max_x - self.min_x).max(0.0)
    }

    /// Total height of the safe area.
    #[inline]
    pub fn height(&self) -> f32 {
        (self.max_y - self.min_y).max(0.0)
    }

    /// Dimensions (width, height) of the safe area.
    #[inline]
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }

    /// Center point of the safe area.
    #[inline]
    pub fn center(&self) -> Vec2 {
        Vec2::new(
            (self.min_x + self.max_x) * 0.5,
            (self.min_y + self.max_y) * 0.5,
        )
    }

    /// Checks if a given point is inside the safe area.
    #[inline]
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.min_x
            && point.x <= self.max_x
            && point.y >= self.min_y
            && point.y <= self.max_y
    }

    /// Clamps a point within the safe area.
    #[inline]
    pub fn clamp_point(&self, point: Vec2) -> Vec2 {
        Vec2::new(
            point.x.clamp(self.min_x, self.max_x),
            point.y.clamp(self.min_y, self.max_y),
        )
    }

    /// Clamps a rectangular UI element `[position, position + size]` within the safe area.
    #[inline]
    pub fn clamp_element(&self, position: Vec2, size: Vec2) -> Vec2 {
        let max_pos_x = (self.max_x - size.x).max(self.min_x);
        let max_pos_y = (self.max_y - size.y).max(self.min_y);
        Vec2::new(
            position.x.clamp(self.min_x, max_pos_x),
            position.y.clamp(self.min_y, max_pos_y),
        )
    }
}

/// FiveM and console-standard safezone scalar and calculation engine.
///
/// In FiveM and GTA V, the safezone setting ranges between 0.8 (most inset) and 1.0 (full edge-to-edge).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Safezone {
    scalar: f32,
}

impl Default for Safezone {
    /// Default safezone is 1.0 (full screen, zero margin).
    fn default() -> Self {
        Self { scalar: 1.0 }
    }
}

impl Safezone {
    /// Minimum allowed safezone scalar (80% safe area).
    pub const MIN_SCALAR: f32 = 0.8;
    /// Maximum allowed safezone scalar (100% full screen).
    pub const MAX_SCALAR: f32 = 1.0;
    /// Standard console broadcast safezone (90% safe area).
    pub const BROADCAST_SCALAR: f32 = 0.9;

    /// Creates a new `Safezone` with the scalar clamped between `MIN_SCALAR` (0.8) and `MAX_SCALAR` (1.0).
    pub fn new(scalar: f32) -> Self {
        Self {
            scalar: scalar.clamp(Self::MIN_SCALAR, Self::MAX_SCALAR),
        }
    }

    /// Creates a full edge-to-edge safezone (scalar = 1.0).
    #[inline]
    pub fn full() -> Self {
        Self { scalar: 1.0 }
    }

    /// Creates a standard broadcast 90% safezone (scalar = 0.9).
    #[inline]
    pub fn broadcast() -> Self {
        Self {
            scalar: Self::BROADCAST_SCALAR,
        }
    }

    /// Returns the active scalar value in `[0.8, 1.0]`.
    #[inline]
    pub fn scalar(&self) -> f32 {
        self.scalar
    }

    /// Sets the scalar value clamped to `[0.8, 1.0]`.
    pub fn set_scalar(&mut self, scalar: f32) {
        self.scalar = scalar.clamp(Self::MIN_SCALAR, Self::MAX_SCALAR);
    }

    /// Computes the horizontal and vertical margins for a given screen resolution:
    ///
    /// $$M_x = \frac{W \cdot (1 - S)}{2}$$
    /// $$M_y = \frac{H \cdot (1 - S)}{2}$$
    #[inline]
    pub fn compute_margins(&self, resolution: Vec2) -> Vec2 {
        let factor = (1.0 - self.scalar) * 0.5;
        Vec2::new(resolution.x * factor, resolution.y * factor)
    }

    /// Computes the pixel safe rectangle for a given screen resolution:
    /// `[Mx, My, W - Mx, H - My]`.
    pub fn compute_rect(&self, resolution: Vec2) -> SafeRect {
        let margins = self.compute_margins(resolution);
        SafeRect::new(
            margins.x,
            margins.y,
            (resolution.x - margins.x).max(margins.x),
            (resolution.y - margins.y).max(margins.y),
        )
    }

    /// Converts normalized screen coordinates (UV `[0.0, 1.0]`) to absolute pixel coordinates.
    #[inline]
    pub fn ndc_to_pixel(ndc: Vec2, resolution: Vec2) -> Vec2 {
        Vec2::new(ndc.x * resolution.x, ndc.y * resolution.y)
    }

    /// Converts pixel coordinates to normalized screen coordinates (UV `[0.0, 1.0]`).
    #[inline]
    pub fn pixel_to_ndc(pixel: Vec2, resolution: Vec2) -> Vec2 {
        Vec2::new(
            if resolution.x > 0.0 {
                pixel.x / resolution.x
            } else {
                0.0
            },
            if resolution.y > 0.0 {
                pixel.y / resolution.y
            } else {
                0.0
            },
        )
    }

    /// Converts normalized safezone coordinates (`[0.0, 1.0]` mapping within the safe rect)
    /// to screen pixel coordinates.
    pub fn safe_ndc_to_pixel(&self, safe_ndc: Vec2, resolution: Vec2) -> Vec2 {
        let rect = self.compute_rect(resolution);
        Vec2::new(
            rect.min_x + safe_ndc.x * rect.width(),
            rect.min_y + safe_ndc.y * rect.height(),
        )
    }

    /// Converts screen pixel coordinates to normalized safezone coordinates
    /// (`[0.0, 1.0]` mapping inside the safe area).
    pub fn pixel_to_safe_ndc(&self, pixel: Vec2, resolution: Vec2) -> Vec2 {
        let rect = self.compute_rect(resolution);
        let w = rect.width();
        let h = rect.height();
        Vec2::new(
            if w > 0.0 {
                (pixel.x - rect.min_x) / w
            } else {
                0.0
            },
            if h > 0.0 {
                (pixel.y - rect.min_y) / h
            } else {
                0.0
            },
        )
    }
}
