#!/usr/bin/env bash
# Download ONNX models required by the measurement service.
# Run this once before starting the service.
set -euo pipefail

MODELS_DIR="${1:-../models}"
mkdir -p "$MODELS_DIR"

echo "Downloading BlazePose Full ONNX model..."
# MediaPipe BlazePose full landmark model — ONNX export
# Original TFLite: https://storage.googleapis.com/mediapipe-models/pose_landmarker/pose_landmarker_full/float16/latest/pose_landmarker_full.task
# Convert with: python -m tf2onnx.convert --tflite pose_landmark_full.tflite --output blazepose_full.onnx
# Pre-converted version (community):
curl -L -o "$MODELS_DIR/blazepose_full.onnx" \
  "https://github.com/PINTO0309/PINTO_model_zoo/raw/main/053_BlazePose/20_3D_Multi-Person_PoseNet_with_Tracking/model/movenet_multipose_lightning.onnx" \
  || echo "⚠  Auto-download failed — see README for manual steps"

echo "Downloading U2-Net segmentation model..."
# U2-Net: used by rembg for background removal
curl -L -o "$MODELS_DIR/u2net.onnx" \
  "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net.onnx" \
  || echo "⚠  Auto-download failed — download manually from rembg releases"

echo ""
echo "Models saved to $MODELS_DIR/"
echo ""
echo "Directory contents:"
ls -lh "$MODELS_DIR/"
