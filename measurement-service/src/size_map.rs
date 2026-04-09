/// Measurement → size label mapping (mirrors the Kotlin SizeMapper logic).
/// This is the Rust-side equivalent, used only if the service needs to
/// return sizes directly. Normally the JVM side does final size mapping.

use crate::measure::BodyMeasurements;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeLabel { XS, S, M, L, XL, XXL }

impl SizeLabel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::XS  => "XS",
            Self::S   => "S",
            Self::M   => "M",
            Self::L   => "L",
            Self::XL  => "XL",
            Self::XXL => "XXL",
        }
    }

    fn order(self) -> usize {
        match self {
            Self::XS  => 0,
            Self::S   => 1,
            Self::M   => 2,
            Self::L   => 3,
            Self::XL  => 4,
            Self::XXL => 5,
        }
    }

    fn all() -> &'static [SizeLabel] {
        &[Self::XS, Self::S, Self::M, Self::L, Self::XL, Self::XXL]
    }
}

struct Bounds {
    chest: Option<RangeInclusive<f32>>,
    waist: Option<RangeInclusive<f32>>,
    hip:   Option<RangeInclusive<f32>>,
}

fn women_tops() -> Vec<(SizeLabel, Bounds)> {
    vec![
        (SizeLabel::XS,  Bounds { chest: Some(76.0..=82.0),  waist: Some(58.0..=63.0), hip: None }),
        (SizeLabel::S,   Bounds { chest: Some(82.0..=88.0),  waist: Some(63.0..=68.0), hip: None }),
        (SizeLabel::M,   Bounds { chest: Some(88.0..=94.0),  waist: Some(68.0..=73.0), hip: None }),
        (SizeLabel::L,   Bounds { chest: Some(94.0..=100.0), waist: Some(73.0..=79.0), hip: None }),
        (SizeLabel::XL,  Bounds { chest: Some(100.0..=106.0),waist: Some(79.0..=85.0), hip: None }),
        (SizeLabel::XXL, Bounds { chest: Some(106.0..=114.0),waist: Some(85.0..=92.0), hip: None }),
    ]
}

pub fn recommend_tops(m: &BodyMeasurements) -> SizeLabel {
    let chart = women_tops();
    for (size, bounds) in &chart {
        let chest_ok = bounds.chest.as_ref().map_or(true, |r| r.contains(&m.chest_cm));
        let waist_ok = bounds.waist.as_ref().map_or(true, |r| r.contains(&m.waist_cm));
        if chest_ok && waist_ok {
            return *size;
        }
    }
    // Fallback: find closest chest
    chart.iter()
        .min_by(|(_, a), (_, b)| {
            let da = a.chest.as_ref().map_or(f32::MAX, |r| {
                (m.chest_cm - (r.start() + r.end()) / 2.0).abs()
            });
            let db = b.chest.as_ref().map_or(f32::MAX, |r| {
                (m.chest_cm - (r.start() + r.end()) / 2.0).abs()
            });
            da.partial_cmp(&db).unwrap()
        })
        .map(|(s, _)| *s)
        .unwrap_or(SizeLabel::M)
}
