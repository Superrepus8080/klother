/// U2-Net background segmentation ONNX wrapper.
///
/// Model: u2net.onnx  (from rembg project)
///   Input:  [1, 3, 320, 320]  — RGB, normalised to ImageNet mean/std
///   Output: [1, 1, 320, 320]  — binary saliency mask [0, 1]
///
/// Download: https://github.com/danielgatis/rembg/releases
/// (models/u2net.onnx)

use anyhow::{Context, Result};
use image::{DynamicImage, GrayImage, Luma};
use ndarray::Array4;
use ort::{inputs, Session, SessionBuilder};

const IMAGENET_MEAN: [f32; 3] = [0.485, 0.456, 0.406];
const IMAGENET_STD:  [f32; 3] = [0.229, 0.224, 0.225];
const U2NET_SIZE:    u32      = 320;

pub struct Segmentor {
    session: Session,
}

impl Segmentor {
    pub fn load(model_path: &str) -> Result<Self> {
        let session = SessionBuilder::new()?
            .with_optimization_level(ort::GraphOptimizationLevel::Level3)?
            .with_intra_threads(2)?
            .commit_from_file(model_path)
            .context(format!("Failed to load U2-Net model from {model_path}"))?;

        Ok(Self { session })
    }

    /// Produce a binary body silhouette mask at the original image resolution.
    /// Returns a GrayImage where 255 = body, 0 = background.
    pub fn segment(&self, image: &DynamicImage) -> Result<GrayImage> {
        let (orig_w, orig_h) = (image.width(), image.height());

        // ── Preprocess: resize → float32 NCHW with ImageNet normalisation ────
        let resized = image.resize_exact(U2NET_SIZE, U2NET_SIZE, image::imageops::FilterType::Lanczos3);
        let rgb     = resized.to_rgb8();

        let mut input = Array4::<f32>::zeros([1, 3, U2NET_SIZE as usize, U2NET_SIZE as usize]);
        for y in 0..U2NET_SIZE as usize {
            for x in 0..U2NET_SIZE as usize {
                let px = rgb.get_pixel(x as u32, y as u32);
                for c in 0..3 {
                    input[[0, c, y, x]] =
                        (px[c] as f32 / 255.0 - IMAGENET_MEAN[c]) / IMAGENET_STD[c];
                }
            }
        }

        // ── Inference ─────────────────────────────────────────────────────────
        let outputs = self
            .session
            .run(inputs!["input.1" => input.view()]?)
            .context("U2-Net inference failed")?;

        let raw = outputs["1556"]  // U2-Net final output node name
            .try_extract_tensor::<f32>()
            .context("Could not extract segmentation output")?;
        let view = raw.view();

        // ── Build mask at original resolution ─────────────────────────────────
        // First create mask at 320×320
        let mut mask_320 = GrayImage::new(U2NET_SIZE, U2NET_SIZE);
        for y in 0..U2NET_SIZE as usize {
            for x in 0..U2NET_SIZE as usize {
                let val = view[[0, 0, y, x]];
                mask_320.put_pixel(x as u32, y as u32, Luma([if val > 0.5 { 255 } else { 0 }]));
            }
        }

        // Resize mask back to original image dimensions
        let mask_dyn = DynamicImage::ImageLuma8(mask_320);
        let mask_orig = mask_dyn
            .resize_exact(orig_w, orig_h, image::imageops::FilterType::Nearest)
            .into_luma8();

        Ok(mask_orig)
    }

    /// Get width in pixels of the silhouette at a given vertical fraction.
    ///
    /// `y_fraction` = 0.0 means top of the image, 1.0 = bottom.
    pub fn width_at_fraction(mask: &GrayImage, y_fraction: f32) -> f32 {
        let (w, h) = mask.dimensions();
        let row_idx = ((y_fraction * h as f32) as u32).clamp(0, h - 1);

        let mut left  = w;
        let mut right = 0u32;
        let mut found = false;

        for x in 0..w {
            if mask.get_pixel(x, row_idx)[0] > 127 {
                left  = left.min(x);
                right = right.max(x);
                found = true;
            }
        }

        if found { (right - left) as f32 } else { 0.0 }
    }
}
