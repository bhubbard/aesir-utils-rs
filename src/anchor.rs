use crate::safezone::SafeRect;
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Standard screen/UI anchor positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    /// Commonly used in FiveM for circular/square minimap and vital stats (health, armor, hunger).
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Anchor {
    /// Returns the normalized anchor point `(u, v)` in `[0.0, 1.0]`.
    /// `(0.0, 0.0)` is top-left, `(1.0, 1.0)` is bottom-right.
    #[inline]
    pub const fn uv(&self) -> Vec2 {
        match self {
            Self::TopLeft => Vec2::new(0.0, 0.0),
            Self::TopCenter => Vec2::new(0.5, 0.0),
            Self::TopRight => Vec2::new(1.0, 0.0),
            Self::CenterLeft => Vec2::new(0.0, 0.5),
            Self::Center => Vec2::new(0.5, 0.5),
            Self::CenterRight => Vec2::new(1.0, 0.5),
            Self::BottomLeft => Vec2::new(0.0, 1.0),
            Self::BottomCenter => Vec2::new(0.5, 1.0),
            Self::BottomRight => Vec2::new(1.0, 1.0),
        }
    }

    /// Default pivot point matching the anchor.
    ///
    /// For instance, `BottomLeft` has pivot `(0.0, 1.0)` so that placing an element
    /// at the bottom-left anchor grounds its bottom-left corner at the anchor coordinate.
    #[inline]
    pub const fn default_pivot(&self) -> Vec2 {
        self.uv()
    }

    /// Returns natural directional signs for user offsets:
    /// - Left anchors: positive X shifts inward (right).
    /// - Right anchors: positive X shifts inward (left).
    /// - Top anchors: positive Y shifts inward (down).
    /// - Bottom anchors: positive Y shifts inward (up).
    #[inline]
    pub const fn inward_direction(&self) -> Vec2 {
        match self {
            Self::TopLeft => Vec2::new(1.0, 1.0),
            Self::TopCenter => Vec2::new(0.0, 1.0),
            Self::TopRight => Vec2::new(-1.0, 1.0),
            Self::CenterLeft => Vec2::new(1.0, 0.0),
            Self::Center => Vec2::new(0.0, 0.0),
            Self::CenterRight => Vec2::new(-1.0, 0.0),
            Self::BottomLeft => Vec2::new(1.0, -1.0),
            Self::BottomCenter => Vec2::new(0.0, -1.0),
            Self::BottomRight => Vec2::new(-1.0, -1.0),
        }
    }
}

/// Canvas scaling modes for virtual canvas resolution mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ScaleMode {
    /// Preserves aspect ratio by scaling according to the smaller dimension scale.
    #[default]
    UniformFit,
    /// Preserves aspect ratio by scaling according to the larger dimension scale.
    UniformFill,
    /// Scales strictly based on horizontal width ratio.
    MatchWidth,
    /// Scales strictly based on vertical height ratio.
    MatchHeight,
    /// Scales X and Y independently (stretches UI if aspect ratios differ).
    Stretch,
}

/// Virtual canvas baseline engine (default: 1920x1080 1080p).
///
/// Designed to author UI layouts once at 1080p and cleanly scale to 720p, 1440p, 4K, 8K,
/// or ultrawide displays with mathematical accuracy.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VirtualCanvas {
    pub reference_size: Vec2,
    pub scale_mode: ScaleMode,
}

impl Default for VirtualCanvas {
    fn default() -> Self {
        Self {
            reference_size: Vec2::new(1920.0, 1080.0),
            scale_mode: ScaleMode::UniformFit,
        }
    }
}

impl VirtualCanvas {
    /// Creates a virtual canvas with reference size (e.g. 1920x1080).
    pub const fn new(reference_width: f32, reference_height: f32) -> Self {
        Self {
            reference_size: Vec2::new(reference_width, reference_height),
            scale_mode: ScaleMode::UniformFit,
        }
    }

    /// Sets the scaling mode.
    pub fn with_mode(mut self, mode: ScaleMode) -> Self {
        self.scale_mode = mode;
        self
    }

    /// Computes the scale vector `(scale_x, scale_y)` from reference canvas to target resolution.
    pub fn scale_factors(&self, target_resolution: Vec2) -> Vec2 {
        let sx = if self.reference_size.x > 0.0 {
            target_resolution.x / self.reference_size.x
        } else {
            1.0
        };
        let sy = if self.reference_size.y > 0.0 {
            target_resolution.y / self.reference_size.y
        } else {
            1.0
        };

        match self.scale_mode {
            ScaleMode::UniformFit => {
                let s = sx.min(sy);
                Vec2::splat(s)
            }
            ScaleMode::UniformFill => {
                let s = sx.max(sy);
                Vec2::splat(s)
            }
            ScaleMode::MatchWidth => Vec2::splat(sx),
            ScaleMode::MatchHeight => Vec2::splat(sy),
            ScaleMode::Stretch => Vec2::new(sx, sy),
        }
    }

    /// Scales a 1080p reference size to the target resolution.
    #[inline]
    pub fn scale_size(&self, size_ref: Vec2, target_resolution: Vec2) -> Vec2 {
        let scales = self.scale_factors(target_resolution);
        Vec2::new(size_ref.x * scales.x, size_ref.y * scales.y)
    }

    /// Scales a 1080p scalar offset or padding.
    #[inline]
    pub fn scale_scalar(&self, scalar_ref: f32, target_resolution: Vec2) -> f32 {
        let scales = self.scale_factors(target_resolution);
        scalar_ref * scales.y
    }
}

/// Helper for calculating anchor positions relative to safezone rects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AnchorPositioner;

impl AnchorPositioner {
    /// Computes the anchor point on the safezone rectangle.
    pub fn anchor_point(anchor: Anchor, rect: &SafeRect) -> Vec2 {
        let uv = anchor.uv();
        Vec2::new(
            rect.min_x + uv.x * rect.width(),
            rect.min_y + uv.y * rect.height(),
        )
    }

    /// Computes the top-left coordinate `(x, y)` of a UI element positioned at an `anchor`
    /// with an optional pivot and screen offset.
    ///
    /// - If `pivot` is `None`, uses `anchor.default_pivot()`.
    /// - `offset` is added directly in screen pixel coordinates.
    pub fn position(
        anchor: Anchor,
        element_size: Vec2,
        offset: Vec2,
        pivot: Option<Vec2>,
        rect: &SafeRect,
    ) -> Vec2 {
        let origin = Self::anchor_point(anchor, rect);
        let p = pivot.unwrap_or_else(|| anchor.default_pivot());
        Vec2::new(
            origin.x - (p.x * element_size.x) + offset.x,
            origin.y - (p.y * element_size.y) + offset.y,
        )
    }

    /// Computes position using inward directional offset.
    ///
    /// For example, `inward_offset = Vec2::new(20.0, 20.0)` will inset the element 20px
    /// away from the edges regardless of which corner or side it is anchored to.
    pub fn position_inward(
        anchor: Anchor,
        element_size: Vec2,
        inward_offset: Vec2,
        pivot: Option<Vec2>,
        rect: &SafeRect,
    ) -> Vec2 {
        let dir = anchor.inward_direction();
        let screen_offset = Vec2::new(inward_offset.x * dir.x, inward_offset.y * dir.y);
        Self::position(anchor, element_size, screen_offset, pivot, rect)
    }

    /// Returns normalized UV `[0.0, 1.0]` coordinates on screen for an element's top-left corner.
    pub fn position_ndc(
        anchor: Anchor,
        element_size_pixels: Vec2,
        offset_pixels: Vec2,
        rect: &SafeRect,
        screen_resolution: Vec2,
    ) -> Vec2 {
        let pos = Self::position(anchor, element_size_pixels, offset_pixels, None, rect);
        Vec2::new(
            if screen_resolution.x > 0.0 {
                pos.x / screen_resolution.x
            } else {
                0.0
            },
            if screen_resolution.y > 0.0 {
                pos.y / screen_resolution.y
            } else {
                0.0
            },
        )
    }
}
