use aesir_utils_rs::{Anchor, AnchorPositioner, Safezone, ScaleMode, VirtualCanvas};
use glam::Vec2;

#[test]
fn test_anchor_positions_bottom_left_minimap() {
    let sz = Safezone::broadcast(); // 0.9 scalar
    let screen = Vec2::new(1920.0, 1080.0);
    let rect = sz.compute_rect(screen);

    // Inset ~96px X, ~54px Y
    assert!((rect.min_x - 96.0).abs() < 1e-3);
    assert!((rect.max_y - (1080.0 - 54.0)).abs() < 1e-3); // 1026.0

    let minimap_size = Vec2::new(280.0, 200.0);
    // At BottomLeft with default pivot (0, 1) and 0 offset:
    let pos = AnchorPositioner::position(Anchor::BottomLeft, minimap_size, Vec2::ZERO, None, &rect);
    assert!((pos.x - 96.0).abs() < 1e-3);
    assert!((pos.y - 826.0).abs() < 1e-3);

    // Using inward offset of 20px horizontally and 20px vertically
    let pos_inward = AnchorPositioner::position_inward(
        Anchor::BottomLeft,
        minimap_size,
        Vec2::new(20.0, 20.0),
        None,
        &rect,
    );
    assert!((pos_inward.x - (96.0 + 20.0)).abs() < 1e-3);
    assert!((pos_inward.y - (826.0 - 20.0)).abs() < 1e-3);
}

#[test]
fn test_anchor_positions_top_right() {
    let sz = Safezone::broadcast();
    let screen = Vec2::new(1920.0, 1080.0);
    let rect = sz.compute_rect(screen);

    let hud_size = Vec2::new(150.0, 60.0);
    // TopRight pivot is (1, 0)
    let pos = AnchorPositioner::position(Anchor::TopRight, hud_size, Vec2::ZERO, None, &rect);
    assert!((pos.x - (rect.max_x - 150.0)).abs() < 1e-3);
    assert!((pos.y - rect.min_y).abs() < 1e-3);

    let pos_inward = AnchorPositioner::position_inward(
        Anchor::TopRight,
        hud_size,
        Vec2::new(15.0, 15.0),
        None,
        &rect,
    );
    assert!((pos_inward.x - ((rect.max_x - 150.0) - 15.0)).abs() < 1e-3);
    assert!((pos_inward.y - (rect.min_y + 15.0)).abs() < 1e-3);
}

#[test]
fn test_virtual_canvas_1080p_scaling() {
    let canvas = VirtualCanvas::default(); // 1920x1080 baseline

    // 4K UHD (3840x2160) is exact 2x scale
    let res_4k = Vec2::new(3840.0, 2160.0);
    let size_1080p = Vec2::new(200.0, 100.0);
    let scaled_4k = canvas.scale_size(size_1080p, res_4k);
    assert_eq!(scaled_4k, Vec2::new(400.0, 200.0));

    // 720p (1280x720) is 2/3 scale
    let res_720p = Vec2::new(1280.0, 720.0);
    let scaled_720p = canvas.scale_size(size_1080p, res_720p);
    assert!((scaled_720p.x - 200.0 * (1280.0 / 1920.0)).abs() < 1e-4);
    assert!((scaled_720p.y - 100.0 * (720.0 / 1080.0)).abs() < 1e-4);

    // Ultrawide 3440x1440 with UniformFit
    // 3440 / 1920 = 1.791666, 1440 / 1080 = 1.333333 -> fit scale is 1.333333
    let res_uw = Vec2::new(3440.0, 1440.0);
    let scaled_uw = canvas.scale_size(size_1080p, res_uw);
    assert!((scaled_uw.x - 200.0 * (4.0 / 3.0)).abs() < 1e-3);
    assert!((scaled_uw.y - 100.0 * (4.0 / 3.0)).abs() < 1e-3);

    // Stretch scale mode
    let canvas_stretch = VirtualCanvas::default().with_mode(ScaleMode::Stretch);
    let scaled_stretch = canvas_stretch.scale_size(size_1080p, res_uw);
    assert!((scaled_stretch.x - 200.0 * (3440.0 / 1920.0)).abs() < 1e-3);
    assert!((scaled_stretch.y - 100.0 * (1440.0 / 1080.0)).abs() < 1e-3);
}
