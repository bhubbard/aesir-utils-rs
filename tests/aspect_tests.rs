use aesir_utils_rs::{AspectRatio, AspectRatioCategory, ViewportFitType};
use glam::Vec2;

#[test]
fn test_aspect_ratio_classification() {
    let standard = AspectRatio::from_resolution(1920.0, 1080.0);
    assert_eq!(standard.category(), AspectRatioCategory::Standard16_9);
    assert!(!standard.is_ultrawide());

    let wqhd = AspectRatio::from_resolution(2560.0, 1440.0);
    assert_eq!(wqhd.category(), AspectRatioCategory::Standard16_9);

    let sixteen_ten = AspectRatio::from_resolution(1920.0, 1200.0);
    assert_eq!(sixteen_ten.category(), AspectRatioCategory::Standard16_10);

    let ultrawide = AspectRatio::from_resolution(3440.0, 1440.0);
    assert_eq!(ultrawide.category(), AspectRatioCategory::Ultrawide21_9);
    assert!(ultrawide.is_ultrawide());

    let super_ultrawide = AspectRatio::from_resolution(5120.0, 1440.0);
    assert_eq!(
        super_ultrawide.category(),
        AspectRatioCategory::SuperUltrawide32_9
    );
    assert!(super_ultrawide.is_ultrawide());

    let classic_4_3 = AspectRatio::from_resolution(1024.0, 768.0);
    assert_eq!(classic_4_3.category(), AspectRatioCategory::Classic4_3);

    let classic_5_4 = AspectRatio::from_resolution(1280.0, 1024.0);
    assert_eq!(classic_5_4.category(), AspectRatioCategory::Classic5_4);
}

#[test]
fn test_aspect_compensation_factor() {
    let standard = AspectRatio::from_resolution(1920.0, 1080.0);
    // At 16:9, compensation relative to 16:9 is 1.0
    assert!((standard.compensation_factor_16_9() - 1.0).abs() < 1e-4);

    let uw = AspectRatio::from_resolution(3440.0, 1440.0);
    let comp = uw.compensation_factor_16_9();
    // 3440/1440 = 2.38888. 1.77777 / 2.38888 ~= 0.744
    assert!(comp < 1.0);
    assert!((comp - (16.0 / 9.0) / (3440.0 / 1440.0)).abs() < 1e-4);
}

#[test]
fn test_viewport_letterbox_and_pillarbox() {
    // 16:9 target inside a 21:9 screen (3440x1440) -> Pillarbox (bars on sides)
    let fit_pillar = AspectRatio::compute_viewport_fit(
        Vec2::new(3440.0, 1440.0),
        16.0 / 9.0,
        Some(Vec2::new(1920.0, 1080.0)),
    );
    assert_eq!(fit_pillar.fit_type, ViewportFitType::Pillarbox);
    assert_eq!(fit_pillar.height, 1440.0);
    assert!((fit_pillar.width - (1440.0 * 16.0 / 9.0)).abs() < 1e-3);
    assert!(fit_pillar.x > 0.0);
    assert_eq!(fit_pillar.y, 0.0);

    // 16:9 target inside a 4:3 screen (1024x768) -> Letterbox (bars top/bottom)
    let fit_letter = AspectRatio::compute_viewport_fit(
        Vec2::new(1024.0, 768.0),
        16.0 / 9.0,
        Some(Vec2::new(1920.0, 1080.0)),
    );
    assert_eq!(fit_letter.fit_type, ViewportFitType::Letterbox);
    assert_eq!(fit_letter.width, 1024.0);
    assert!((fit_letter.height - (1024.0 / (16.0 / 9.0))).abs() < 1e-3);
    assert_eq!(fit_letter.x, 0.0);
    assert!(fit_letter.y > 0.0);

    // Exact match
    let fit_exact = AspectRatio::compute_viewport_fit(
        Vec2::new(1920.0, 1080.0),
        16.0 / 9.0,
        Some(Vec2::new(1920.0, 1080.0)),
    );
    assert_eq!(fit_exact.fit_type, ViewportFitType::Exact);
    assert_eq!(fit_exact.x, 0.0);
    assert_eq!(fit_exact.y, 0.0);
}
