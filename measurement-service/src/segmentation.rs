/// U2-Net background segmentation ONNX wrapper — ort 2.x API.
///
/// Model: u2net.onnx  (from rembg project)
///   Input:  [1, 3, 320, 320]  — RGB float32 NCHW, ImageNet-normalised
///   Output: [1, 1, 320, 320]  — saliency mask, float32 [0, 1]
///
/// Download:
///   curl -L -o models/u2net.onnx \
///     https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net.onnx

use anyhow::{Context, Result};
use image::{DynamicImage, GrayImage, Luma};
use ndarray::Array4;
use ort::{
    session::{builder::GraphOptimizationLevel, Session},
    value::Tensor,
};

const IMAGENET_MEAN: [f32; 3] = [0.485, 0.456, 0.406];
const IMAGENET_STD:  [f32; 3] = [0.229, 0.224, 0.225];
const U2NET_SIZE:    u32      = 320;

pub struct Segmentor {
    session: Session,
}

impl Segmentor {
    pub fn load(model_path: &str) -> Result<Self> {
        let session = Session::builder()
            .map_err(|e| anyhow::anyhow!("Failed to create ONNX session builder: {e}"))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| anyhow::anyhow!("Failed to set optimization level: {e}"))?
            .with_intra_threads(2)
            .map_err(|e| anyhow::anyhow!("Failed to set intra-op threads: {e}"))?
            .commit_from_file(model_path)
            .map_err(|e| anyhow::anyhow!("Failed to load U2-Net model from '{model_path}': {e}"))?;

        Ok(Self { session })
    }

    /// Produce a binary body-silhouette mask at the original image resolution.
    /// Returns a GrayImage where 255 = body, 0 = background.
    pub fn segment(&mut self, image: &DynamicImage) -> Result<GrayImage> {
        let (orig_w, orig_h) = (image.width(), image.height());

        // ── Preprocess: resize → NCHW float32 with ImageNet normalisation ────────
        let resized = image.resize_exact(
            U2NET_SIZE,
            U2NET_SIZE,
            image::imageops::FilterType::Lanczos3,
        );
        let rgb = resized.to_rgb8();

        let mut input = Array4::<f32>::zeros([1, 3, U2NET_SIZE as usize, U2NET_SIZE as usize]);
        for y in 0..U2NET_SIZE as usize {
            for x in 0..U2NET_SIZE as usize {
                let px = rgb.get_pixel(x as u32, y as u32);
                for c in 0..3usize {
                    input[[0, c, y, x]] =
                        (px[c] as f32 / 255.0 - IMAGENET_MEAN[c]) / IMAGENET_STD[c];
                }
            }
        }

        // ── Inference ─────────────────────────────────────────────────────────────
        let tensor = Tensor::<f32>::from_array(input)
            .context("Failed to create segmentation input tensor")?;

        let outputs = self
            .session
            .run(ort::inputs!["input.1" => tensor])
            .context("U2-Net inference failed")?;

        // U2-Net final output node — first output is the full-res saliency map.
        // "1959" for the rembg v0.0.0 u2net.onnx model.
        let (_, flat) = outputs["1959"]
            .try_extract_tensor::<f32>()
            .context("Failed to extract segmentation output tensor")?;

        // flat is [1, 1, 320, 320] in row-major order → flat[y * 320 + x]
        let stride = U2NET_SIZE as usize;

        // ── Binary mask at 320×320 ────────────────────────────────────────────────
        let mut mask_320 = GrayImage::new(U2NET_SIZE, U2NET_SIZE);
        for y in 0..stride {
            for x in 0..stride {
                let val = flat[y * stride + x];
                mask_320.put_pixel(x as u32, y as u32, Luma([if val > 0.5 { 255u8 } else { 0u8 }]));
            }
        }

        // ── Upscale to original image dimensions ──────────────────────────────────
        let mask_orig = DynamicImage::ImageLuma8(mask_320)
            .resize_exact(orig_w, orig_h, image::imageops::FilterType::Nearest)
            .into_luma8();

        Ok(mask_orig)
    }

    /// Width in pixels of the body silhouette at a given vertical fraction of the image.
    /// `y_fraction`: 0.0 = top, 1.0 = bottom.
    pub fn width_at_fraction(mask: &GrayImage, y_fraction: f32) -> f32 {
        let (w, h) = mask.dimensions();
        let row = ((y_fraction * h as f32) as u32).clamp(0, h - 1);

        let mut left  = w;
        let mut right = 0u32;
        let mut found = false;

        for x in 0..w {
            if mask.get_pixel(x, row)[0] > 127 {
                if x < left  { left  = x; }
                if x > right { right = x; }
                found = true;
            }
        }

        if found { (right - left) as f32 } else { 0.0 }
    }
}
