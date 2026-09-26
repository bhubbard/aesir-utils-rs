use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Standard aspect ratio classifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AspectRatioCategory {
    /// 16:9 widescreen standard (e.g. 1920x1080, 2560x1440, 3840x2160) (~1.778)
    Standard16_9,
    /// 16:10 monitor standard (e.g. 1920x1200, 2560x1600, Steam Deck 1280x800) (1.6)
    Standard16_10,
    /// 21:9 Ultrawide (e.g. 2560x1080, 3440x1440) (~2.333 - 2.389)
    Ultrawide21_9,
    /// 32:9 Super Ultrawide / Dual-wide (e.g. 5120x1440) (~3.556)
    SuperUltrawide32_9,
    /// 4:3 classic / CRT ratio (e.g. 1024x768, 800x600) (~1.333)
    Classic4_3,
    /// 5:4 classic monitor ratio (e.g. 1280x1024) (1.25)
    Classic5_4,
    /// Other non-standard ratio
    Custom,
}

impl AspectRatioCategory {
    /// Human-readable label for the aspect ratio category.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Standard16_9 => "16:9 Standard",
            Self::Standard16_10 => "16:10",
            Self::Ultrawide21_9 => "21:9 Ultrawide",
            Self::SuperUltrawide32_9 => "32:9 Super Ultrawide",
            Self::Classic4_3 => "4:3 Classic",
            Self::Classic5_4 => "5:4 Classic",
            Self::Custom => "Custom",
        }
    }

    /// Whether this aspect ratio is classified as ultrawide or super-ultrawide.
    pub fn is_ultrawide(&self) -> bool {
        matches!(self, Self::Ultrawide21_9 | Self::SuperUltrawide32_9)
    }
}

/// Viewport letterbox or pillarbox fitting style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewportFitType {
    /// Exact match with target aspect ratio (no bars).
    Exact,
    /// Screen is wider than target ratio; black vertical bars on left & right.
    Pillarbox,
    /// Screen is taller than target ratio; black horizontal bars on top & bottom.
    Letterbox,
}

/// Result of fitting a target aspect ratio into an arbitrary screen resolution.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ViewportFit {
    /// Type of bar padding required.
    pub fit_type: ViewportFitType,
    /// Left offset of active viewport rect.
    pub x: f32,
    /// Top offset of active viewport rect.
    pub y: f32,
    /// Width of active viewport rect.
    pub width: f32,
    /// Height of active viewport rect.
    pub height: f32,
    /// Uniform scale factor from reference design dimensions.
    pub scale: f32,
    /// Total horizontal bar padding (left + right).
    pub total_bar_x: f32,
    /// Total vertical bar padding (top + bottom).
    pub total_bar_y: f32,
}

impl ViewportFit {
    /// Top-left position of the fitted viewport.
    #[inline]
    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// Dimensions (width, height) of the fitted viewport.
    #[inline]
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    /// Half bar width on one side (pillarbox left or right margin).
    #[inline]
    pub fn bar_left(&self) -> f32 {
        self.x
    }

    /// Half bar height on one side (letterbox top or bottom margin).
    #[inline]
    pub fn bar_top(&self) -> f32 {
        self.y
    }
}

/// Aspect ratio calculations, monitor classification, and scaling engine.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AspectRatio {
    ratio: f32,
    category: AspectRatioCategory,
}

impl AspectRatio {
    /// Standard 16:9 ratio constant (16.0 / 9.0 = 1.7777778)
    pub const RATIO_16_9: f32 = 16.0 / 9.0;
    /// Standard 16:10 ratio constant (16.0 / 10.0 = 1.6)
    pub const RATIO_16_10: f32 = 1.6;
    /// Standard 21:9 ultrawide ratio constant (~2.37)
    pub const RATIO_21_9: f32 = 21.0 / 9.0;
    /// Standard 32:9 super ultrawide ratio constant (~3.555)
    pub const RATIO_32_9: f32 = 32.0 / 9.0;
    /// Standard 4:3 classic ratio constant (4.0 / 3.0 = 1.3333334)
    pub const RATIO_4_3: f32 = 4.0 / 3.0;
    /// Standard 5:4 classic ratio constant (5.0 / 4.0 = 1.25)
    pub const RATIO_5_4: f32 = 1.25;

    /// Computes aspect ratio from width and height with default tolerance (0.04).
    pub fn from_resolution(width: f32, height: f32) -> Self {
        let ratio = if height > 0.0 { width / height } else { 1.0 };
        let category = Self::classify(ratio, 0.04);
        Self { ratio, category }
    }

    /// Creates an `AspectRatio` from a `Vec2` screen resolution.
    #[inline]
    pub fn from_vec2(resolution: Vec2) -> Self {
        Self::from_resolution(resolution.x, resolution.y)
    }

    /// Returns the raw numerical aspect ratio $\alpha = W / H$.
    #[inline]
    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    /// Returns the classified category.
    #[inline]
    pub fn category(&self) -> AspectRatioCategory {
        self.category
    }

    /// Classifies an aspect ratio float with a specified tolerance threshold.
    pub fn classify(ratio: f32, tolerance: f32) -> AspectRatioCategory {
        if (ratio - Self::RATIO_16_9).abs() <= tolerance {
            AspectRatioCategory::Standard16_9
        } else if (ratio - Self::RATIO_16_10).abs() <= tolerance {
            AspectRatioCategory::Standard16_10
        } else if (ratio - Self::RATIO_21_9).abs() <= 0.08 || (ratio - 2.37).abs() <= 0.08 {
            AspectRatioCategory::Ultrawide21_9
        } else if (ratio - Self::RATIO_32_9).abs() <= 0.1 {
            AspectRatioCategory::SuperUltrawide32_9
        } else if (ratio - Self::RATIO_4_3).abs() <= tolerance {
            AspectRatioCategory::Classic4_3
        } else if (ratio - Self::RATIO_5_4).abs() <= tolerance {
            AspectRatioCategory::Classic5_4
        } else {
            AspectRatioCategory::Custom
        }
    }

    /// Returns true if the screen is wider than standard 16:9 widescreen.
    #[inline]
    pub fn is_wider_than_16_9(&self) -> bool {
        self.ratio > Self::RATIO_16_9 + 0.02
    }

    /// Returns true if classified as 21:9 or 32:9 ultrawide.
    #[inline]
    pub fn is_ultrawide(&self) -> bool {
        self.category.is_ultrawide()
    }

    /// Computes the horizontal aspect compensation factor relative to a baseline aspect ratio
    /// (by default 16:9).
    ///
    /// In FiveM and GTA V UI engines, when rendering at non-16:9 resolutions (especially 21:9 and 32:9),
    /// circular radar minimaps and reticles stretch horizontally if rendered in raw NDC coordinates.
    ///
    /// The compensation factor $C_x$ scales the horizontal width of UI primitives by:
    /// $$C_x = \frac{\alpha_{\text{ref}}}{\alpha_{\text{current}}}$$
    ///
    /// For a circular minimap of radius $R$:
    /// `pixel_width = R * 2.0 * C_x`
    pub fn compensation_factor(&self, reference_aspect: f32) -> f32 {
        if self.ratio > 0.0 {
            reference_aspect / self.ratio
        } else {
            1.0
        }
    }

    /// Computes horizontal compensation factor using 16:9 reference.
    #[inline]
    pub fn compensation_factor_16_9(&self) -> f32 {
        self.compensation_factor(Self::RATIO_16_9)
    }

    /// Computes compensating scale vector `(Cx, 1.0)` to apply to UI elements.
    #[inline]
    pub fn compensation_scale(&self, reference_aspect: f32) -> Vec2 {
        Vec2::new(self.compensation_factor(reference_aspect), 1.0)
    }

    /// Computes Letterbox or Pillarbox viewport fitting.
    ///
    /// Given a screen resolution $(W, H)$ and a target design aspect ratio $\alpha_{\text{target}}$:
    /// - If screen is wider than target ($\alpha > \alpha_{\text{target}}$): Pillarbox (vertical side bars).
    /// - If screen is taller than target ($\alpha < \alpha_{\text{target}}$): Letterbox (horizontal top/bottom bars).
    /// - If screen matches target: Exact.
    pub fn compute_viewport_fit(
        screen_resolution: Vec2,
        target_aspect: f32,
        reference_resolution: Option<Vec2>,
    ) -> ViewportFit {
        let sw = screen_resolution.x;
        let sh = screen_resolution.y;
        let screen_aspect = if sh > 0.0 { sw / sh } else { target_aspect };

        let eps = 1e-4;
        let (fit_type, vw, vh, ox, oy) = if (screen_aspect - target_aspect).abs() < eps {
            (ViewportFitType::Exact, sw, sh, 0.0, 0.0)
        } else if screen_aspect > target_aspect {
            // Screen is wider -> Pillarbox (bars on left and right)
            let vw = sh * target_aspect;
            let ox = (sw - vw) * 0.5;
            (ViewportFitType::Pillarbox, vw, sh, ox, 0.0)
        } else {
            // Screen is narrower/taller -> Letterbox (bars on top and bottom)
            let vh = sw / target_aspect;
            let oy = (sh - vh) * 0.5;
            (ViewportFitType::Letterbox, sw, vh, 0.0, oy)
        };

        let scale = if let Some(ref_res) = reference_resolution {
            (vw / ref_res.x).min(vh / ref_res.y)
        } else {
            1.0
        };

        ViewportFit {
            fit_type,
            x: ox,
            y: oy,
            width: vw,
            height: vh,
            scale,
            total_bar_x: (sw - vw).max(0.0),
            total_bar_y: (sh - vh).max(0.0),
        }
    }
}
