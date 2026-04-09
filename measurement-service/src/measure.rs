/// Core measurement extraction pipeline.
///
/// Given front + side images and user height, returns body measurements in cm.
///
/// Pipeline:
///   1. Decode JPEG/PNG images
///   2. Run BlazePose on front image → 33 landmarks
///   3. Compute pixels-per-cm from ankle-to-nose distance + user height
///   4. Run U2-Net on both images → body silhouette masks
///   5. Sample mask widths at chest / waist / hip levels
///   6. Estimate circumferences via Ramanujan's ellipse formula
///   7. Return BodyMeasurements

use anyhow::{bail, Result};
use image::DynamicImage;

use crate::pose::{lm, Landmark, PoseEstimator};
use crate::segmentation::Segmentor;

/// All measurements are circumferences or lengths in centimetres.
#[derive(Debug)]
pub struct BodyMeasurements {
    pub height_cm:         f32,
    pub chest_cm:          f32,
    pub waist_cm:          f32,
    pub hip_cm:            f32,
    pub shoulder_width_cm: f32,
    pub inseam_cm:         f32,
}

pub fn extract(
    pose:       &mut PoseEstimator,
    segmentor:  &mut Segmentor,
    front_bytes: &[u8],
    side_bytes:  &[u8],
    height_cm:   f32,
) -> Result<BodyMeasurements> {

    // ── 1. Decode images ──────────────────────────────────────────────────────
    let front_img = image::load_from_memory(front_bytes)?;
    let side_img  = image::load_from_memory(side_bytes)?;

    let front_h = front_img.height() as f32;

    // ── 2. Pose estimation (front image) ──────────────────────────────────────
    let landmarks = pose.estimate(&front_img)?;
    validate_landmarks(&landmarks)?;

    // ── 3. Pixel-to-cm scale factor ───────────────────────────────────────────
    // Use ankle midpoint → nose pixel distance.
    // Subtract ~7 cm for average nose-to-crown offset.
    let effective_height = height_cm - 7.0;

    let ankle_y = (landmarks[lm::LEFT_ANKLE].y + landmarks[lm::RIGHT_ANKLE].y) / 2.0;
    let nose_y  = landmarks[lm::NOSE].y;
    let body_px = (ankle_y - nose_y).abs();

    if body_px < 50.0 {
        bail!("Could not detect full body — make sure you're standing with your whole body visible");
    }

    let ppcm = body_px / effective_height;  // pixels per centimetre

    // ── 4. Silhouette masks ───────────────────────────────────────────────────
    let front_mask = segmentor.segment(&front_img)?;
    let side_mask  = segmentor.segment(&side_img)?;

    // ── 5. Landmark-derived Y fractions for measurement levels ────────────────
    let shoulder_y = (landmarks[lm::LEFT_SHOULDER].y + landmarks[lm::RIGHT_SHOULDER].y) / 2.0;
    let hip_y      = (landmarks[lm::LEFT_HIP].y      + landmarks[lm::RIGHT_HIP].y)      / 2.0;

    // Fractions within the image height:
    //   chest  ≈ 10 px below shoulder midpoint
    //   waist  ≈ midpoint between shoulder and hip
    //   hip    ≈ hip landmark + small offset downward
    let chest_frac = ((shoulder_y + 10.0) / front_h).clamp(0.0, 1.0);
    let waist_frac = ((shoulder_y + hip_y) / 2.0 / front_h).clamp(0.0, 1.0);
    let hip_frac   = ((hip_y + 15.0)       / front_h).clamp(0.0, 1.0);

    // ── 6. Half-widths from front mask (a) and side mask (b) ─────────────────
    let (chest_a, chest_b) = half_widths(&front_mask, &side_mask, chest_frac, ppcm);
    let (waist_a, waist_b) = half_widths(&front_mask, &side_mask, waist_frac, ppcm);
    let (hip_a,   hip_b)   = half_widths(&front_mask, &side_mask, hip_frac,   ppcm);

    // ── 7. Ellipse circumferences (Ramanujan's approximation) ─────────────────
    let chest_cm = ramanujan(chest_a, chest_b);
    let waist_cm = ramanujan(waist_a, waist_b);
    let hip_cm   = ramanujan(hip_a,   hip_b);

    // ── 8. Shoulder width (direct landmark distance) ──────────────────────────
    let l_sh = landmarks[lm::LEFT_SHOULDER];
    let r_sh = landmarks[lm::RIGHT_SHOULDER];
    let shoulder_px = euclid(l_sh.x, l_sh.y, r_sh.x, r_sh.y);
    let shoulder_width_cm = shoulder_px / ppcm;

    // ── 9. Inseam (hip to ankle along y-axis) ─────────────────────────────────
    let inseam_px = (landmarks[lm::LEFT_ANKLE].y - landmarks[lm::LEFT_HIP].y).abs();
    let inseam_cm = inseam_px / ppcm;

    Ok(BodyMeasurements {
        height_cm,
        chest_cm:          round1(chest_cm),
        waist_cm:          round1(waist_cm),
        hip_cm:            round1(hip_cm),
        shoulder_width_cm: round1(shoulder_width_cm),
        inseam_cm:         round1(inseam_cm),
    })
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Ramanujan's approximation for ellipse perimeter.
/// a = semi-major axis (half of front-view width in cm)
/// b = semi-minor axis (half of side-view depth in cm)
fn ramanujan(a: f32, b: f32) -> f32 {
    let h = ((a - b).powi(2)) / ((a + b).powi(2) + 1e-9);
    std::f32::consts::PI * (a + b) * (1.0 + (3.0 * h) / (10.0 + (4.0 - 3.0 * h).sqrt()))
}

fn euclid(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn round1(v: f32) -> f32 {
    (v * 10.0).round() / 10.0
}

/// Returns (a_cm, b_cm) = half front-width, half side-depth in cm.
fn half_widths(
    front_mask: &image::GrayImage,
    side_mask:  &image::GrayImage,
    y_frac:     f32,
    ppcm:       f32,
) -> (f32, f32) {
    let front_px = Segmentor::width_at_fraction(front_mask, y_frac);
    let side_px  = Segmentor::width_at_fraction(side_mask,  y_frac);
    let a = (front_px / ppcm) / 2.0;
    let b = (side_px  / ppcm) / 2.0;
    (a.max(0.1), b.max(0.1))   // prevent degenerate ellipse
}

/// Sanity check: ensure key landmarks are visible enough.
fn validate_landmarks(lms: &[Landmark]) -> Result<()> {
    let key_indices = [
        lm::NOSE, lm::LEFT_SHOULDER, lm::RIGHT_SHOULDER,
        lm::LEFT_HIP, lm::RIGHT_HIP, lm::LEFT_ANKLE, lm::RIGHT_ANKLE,
    ];
    for &idx in &key_indices {
        if lms[idx].visibility < 0.5 {
            bail!(
                "Body landmark {} not visible (visibility={:.2}) — ensure your full body is in frame",
                idx, lms[idx].visibility
            );
        }
    }
    Ok(())
}
