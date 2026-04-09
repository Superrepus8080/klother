/// BlazePose ONNX inference wrapper.
///
/// Model: blazepose_full.onnx
///   Input:  [1, 256, 256, 3]  — RGB, normalised to [0, 1]
///   Output: [1, 195]          — 33 landmarks × (x, y, z, visibility, presence)
///
/// Download the ONNX export from the MediaPipe model garden, or convert
/// the TFLite model with tf2onnx:
///   python -m tf2onnx.convert \
///       --tflite pose_landmark_full.tflite \
///       --output  blazepose_full.onnx

use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use ndarray::{s, Array4};
use ort::{inputs, Session, SessionBuilder};

pub const BLAZEPOSE_INPUT_SIZE: u32 = 256;

/// 33 body landmark indices (MediaPipe convention)
pub mod lm {
    pub const NOSE:           usize = 0;
    pub const LEFT_SHOULDER:  usize = 11;
    pub const RIGHT_SHOULDER: usize = 12;
    pub const LEFT_HIP:       usize = 23;
    pub const RIGHT_HIP:      usize = 24;
    pub const LEFT_KNEE:      usize = 25;
    pub const RIGHT_KNEE:     usize = 26;
    pub const LEFT_ANKLE:     usize = 27;
    pub const RIGHT_ANKLE:    usize = 28;
}

#[derive(Debug, Clone, Copy)]
pub struct Landmark {
    /// Normalised [0, 1] within the 256×256 input patch.
    pub x:          f32,
    pub y:          f32,
    pub z:          f32,
    pub visibility: f32,
}

pub struct PoseEstimator {
    session: Session,
}

impl PoseEstimator {
    pub fn load(model_path: &str) -> Result<Self> {
        let session = SessionBuilder::new()?
            .with_optimization_level(ort::GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_file(model_path)
            .context(format!("Failed to load BlazePose model from {model_path}"))?;

        Ok(Self { session })
    }

    /// Run pose estimation on a decoded image.
    /// Returns 33 landmarks in the original image's pixel coordinates.
    pub fn estimate(&self, image: &DynamicImage) -> Result<Vec<Landmark>> {
        let (orig_w, orig_h) = image.dimensions();

        // ── Preprocess: resize to 256×256, convert to f32 NHWC ──────────────
        let resized = image.resize_exact(
            BLAZEPOSE_INPUT_SIZE,
            BLAZEPOSE_INPUT_SIZE,
            image::imageops::FilterType::Lanczos3,
        );
        let rgb = resized.to_rgb8();

        let mut input = Array4::<f32>::zeros([1, 256, 256, 3]);
        for y in 0..256usize {
            for x in 0..256usize {
                let px = rgb.get_pixel(x as u32, y as u32);
                input[[0, y, x, 0]] = px[0] as f32 / 255.0;
                input[[0, y, x, 1]] = px[1] as f32 / 255.0;
                input[[0, y, x, 2]] = px[2] as f32 / 255.0;
            }
        }

        // ── Inference ─────────────────────────────────────────────────────────
        let outputs = self
            .session
            .run(inputs!["input" => input.view()]?)
            .context("BlazePose inference failed")?;

        // ── Parse output: shape [1, 195] = 33 landmarks × 5 values ───────────
        let raw = outputs["output_0"]
            .try_extract_tensor::<f32>()
            .context("Could not extract pose output tensor")?;

        let flat = raw.view();
        let landmarks: Vec<Landmark> = (0..33)
            .map(|i| {
                let base = i * 5;
                Landmark {
                    // Scale back from [0,1] normalised to original image pixels
                    x:          flat[[0, base    ]] * orig_w  as f32,
                    y:          flat[[0, base + 1]] * orig_h  as f32,
                    z:          flat[[0, base + 2]],
                    visibility: flat[[0, base + 3]],
                }
            })
            .collect();

        Ok(landmarks)
    }
}
