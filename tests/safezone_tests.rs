use aesir_utils_rs::{SafeRect, Safezone};
use glam::Vec2;

#[test]
fn test_safezone_full_resolution() {
    let sz = Safezone::full();
    assert_eq!(sz.scalar(), 1.0);

    let res = Vec2::new(1920.0, 1080.0);
    let margins = sz.compute_margins(res);
    assert_eq!(margins, Vec2::ZERO);

    let rect = sz.compute_rect(res);
    assert_eq!(rect.min_x, 0.0);
    assert_eq!(rect.min_y, 0.0);
    assert_eq!(rect.max_x, 1920.0);
    assert_eq!(rect.max_y, 1080.0);
    assert_eq!(rect.width(), 1920.0);
    assert_eq!(rect.height(), 1080.0);
}

#[test]
fn test_safezone_broadcast_0_9() {
    let sz = Safezone::broadcast();
    assert_eq!(sz.scalar(), 0.9);

    let res = Vec2::new(1920.0, 1080.0);
    let margins = sz.compute_margins(res);
    // (1 - 0.9) / 2 = 0.05
    // 1920 * 0.05 = 96
    // 1080 * 0.05 = 54
    assert!((margins.x - 96.0).abs() < 1e-4);
    assert!((margins.y - 54.0).abs() < 1e-4);

    let rect = sz.compute_rect(res);
    assert!((rect.min_x - 96.0).abs() < 1e-3);
    assert!((rect.min_y - 54.0).abs() < 1e-3);
    assert!((rect.max_x - (1920.0 - 96.0)).abs() < 1e-3);
    assert!((rect.max_y - (1080.0 - 54.0)).abs() < 1e-3);
    assert!((rect.width() - 1728.0).abs() < 1e-3);
    assert!((rect.height() - 972.0).abs() < 1e-3);
}

#[test]
fn test_safezone_clamping() {
    let sz_low = Safezone::new(0.5);
    assert_eq!(sz_low.scalar(), 0.8);

    let sz_high = Safezone::new(1.5);
    assert_eq!(sz_high.scalar(), 1.0);
}

#[test]
fn test_ndc_safe_conversions() {
    let sz = Safezone::broadcast();
    let res = Vec2::new(1920.0, 1080.0);

    // safe NDC (0, 0) should map to safezone top-left (96, 54)
    let p_tl = sz.safe_ndc_to_pixel(Vec2::ZERO, res);
    assert!((p_tl.x - 96.0).abs() < 1e-4);
    assert!((p_tl.y - 54.0).abs() < 1e-4);

    // safe NDC (1, 1) should map to safezone bottom-right
    let p_br = sz.safe_ndc_to_pixel(Vec2::ONE, res);
    assert!((p_br.x - (1920.0 - 96.0)).abs() < 1e-4);
    assert!((p_br.y - (1080.0 - 54.0)).abs() < 1e-4);

    // roundtrip test
    let safe_ndc = Vec2::new(0.35, 0.75);
    let px = sz.safe_ndc_to_pixel(safe_ndc, res);
    let back_ndc = sz.pixel_to_safe_ndc(px, res);
    assert!((back_ndc.x - safe_ndc.x).abs() < 1e-4);
    assert!((back_ndc.y - safe_ndc.y).abs() < 1e-4);
}

#[test]
fn test_rect_contains_and_clamp() {
    let rect = SafeRect::new(100.0, 50.0, 500.0, 400.0);
    assert!(rect.contains(Vec2::new(200.0, 150.0)));
    assert!(!rect.contains(Vec2::new(50.0, 150.0)));

    let clamped = rect.clamp_point(Vec2::new(10.0, 600.0));
    assert_eq!(clamped, Vec2::new(100.0, 400.0));

    let el_pos = rect.clamp_element(Vec2::new(450.0, 350.0), Vec2::new(100.0, 100.0));
    assert_eq!(el_pos, Vec2::new(400.0, 300.0));
}
