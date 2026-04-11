/// BlazePose ONNX inference wrapper — ort 2.x API.
///
/// Model: blazepose_full.onnx
///   Input:  [1, 256, 256, 3]  — RGB float32 NHWC, normalised to [0, 1]
///   Output: [1, 195]          — 33 landmarks × 5 values (x, y, z, visibility, presence)
///                               values are normalised to [0, 1] within the 256×256 patch
///
/// Obtaining the model:
///   Option A — convert TFLite to ONNX:
///     pip install tf2onnx
///     python -m tf2onnx.convert --tflite pose_landmark_full.tflite \
///         --output blazepose_full.onnx --opset 13
///   Option B — use the pre-converted model from PINTO model zoo.

use anyhow::{Context, Result};
use image::DynamicImage;
use ndarray::Array4;
use ort::{
    session::{builder::GraphOptimizationLevel, Session},
    value::Tensor,
};

pub const BLAZEPOSE_INPUT_SIZE: u32 = 256;
pub const N_LANDMARKS: usize = 33;

/// MediaPipe BlazePose landmark indices.
pub mod lm {
    pub const NOSE:           usize = 0;
    pub const LEFT_SHOULDER:  usize = 11;
    pub const RIGHT_SHOULDER: usize = 12;
    pub const LEFT_HIP:       usize = 23;
    pub const RIGHT_HIP:      usize = 24;
    pub const LEFT_ANKLE:     usize = 27;
    pub const RIGHT_ANKLE:    usize = 28;
}

#[derive(Debug, Clone, Copy)]
pub struct Landmark {
    /// Pixel coordinates in the ORIGINAL (unresized) image.
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
        let session = Session::builder()
            .map_err(|e| anyhow::anyhow!("Failed to create ONNX session builder: {e}"))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| anyhow::anyhow!("Failed to set optimization level: {e}"))?
            .with_intra_threads(4)
            .map_err(|e| anyhow::anyhow!("Failed to set intra-op threads: {e}"))?
            .commit_from_file(model_path)
            .map_err(|e| anyhow::anyhow!("Failed to load BlazePose model from '{model_path}': {e}"))?;

        Ok(Self { session })
    }

    /// Run pose estimation and return 33 landmarks in original-image pixel coords.
    pub fn estimate(&mut self, image: &DynamicImage) -> Result<Vec<Landmark>> {
        let (orig_w, orig_h) = (image.width(), image.height());

        // ── Preprocess: resize → float32 NHWC [1, 256, 256, 3], normalise [0,1] ──
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

        // ── Inference ─────────────────────────────────────────────────────────────
        let tensor = Tensor::<f32>::from_array(input)
            .context("Failed to create input tensor")?;

        let outputs = self
            .session
            .run(ort::inputs!["input_1" => tensor])
            .context("BlazePose inference failed")?;

        // ── Parse flat output [1, 195] = 33 landmarks × 5 values ─────────────────
        // tf2onnx exports the landmark output as "Identity" (first output node).
        let (_, flat) = outputs["Identity"]
            .try_extract_tensor::<f32>()
            .context("Failed to extract pose output tensor")?;

        // tf2onnx exports x/y as pixel coords in the 256×256 input patch (not [0,1]).
        // Normalise by input size, then scale to original image dimensions.
        let scale = BLAZEPOSE_INPUT_SIZE as f32;
        let landmarks: Vec<Landmark> = (0..N_LANDMARKS)
            .map(|i| {
                let base = i * 5;
                Landmark {
                    x:          (flat[base]     / scale) * orig_w as f32,
                    y:          (flat[base + 1] / scale) * orig_h as f32,
                    z:          flat[base + 2],
                    // visibility is a raw logit; positive = visible, negative = not.
                    // Keep as-is; validate_landmarks checks > 0.5 (any positive logit is fine).
                    visibility: flat[base + 3],
                }
            })
            .collect();

        Ok(landmarks)
    }
}
