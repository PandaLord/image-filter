// docs: https://docs.rs/photon-rs/latest/photon_rs/transform/index.html
use photon_rs::{Rgba, transform::*, PhotonImage};

use super::ImageAction;
pub enum TransformType {
    Resize {width: u32, height: u32, filter: SamplingFilter},
    Resample {dst_width: usize, dst_height: usize},
    Crop {x1: u32, x2: u32, y1: u32, y2: u32},
    Fliph,
    Flipv,
    Padding(Vec<PaddingAttr>),
    Rotate { angle : i32 },
    Compress(u8),

}

enum PaddingArch {
    Top,
    Bottom,
    Left,
    Right
}

pub struct PaddingAttr {
    padding: u32,
    padding_rgba: Rgba,
    padding_anch: PaddingArch
}

pub struct TransformAction(pub TransformType);

impl TransformAction {
    pub fn transform_apply(&self, img: PhotonImage) -> PhotonImage {
        let mut r_img = img;
        match &self.0 {
            
            TransformType::Resize {width, height, filter} => {
                let filter = match filter {
                    SamplingFilter::Nearest => SamplingFilter::Nearest,
                    SamplingFilter::Triangle => SamplingFilter::Triangle,
                    SamplingFilter::CatmullRom => SamplingFilter::CatmullRom,
                    SamplingFilter::Gaussian => SamplingFilter::Gaussian,
                    SamplingFilter::Lanczos3 => SamplingFilter::Lanczos3,
                };
                r_img = resize(&r_img, *width, *height, filter);
            },
            TransformType::Resample {dst_width, dst_height} => {
                r_img = resample(&r_img, *dst_width, *dst_height);
            },
            TransformType::Crop {x1, x2, y1, y2} => {
                r_img = crop(&mut r_img, *x1, *x2, *y1, *y2);
            },
            TransformType::Fliph => {
                fliph(&mut r_img);
            },
            TransformType::Flipv => {
                flipv(&mut r_img);
            },
            TransformType::Padding(padding_attrs) => {
                todo!()
            },
            TransformType::Rotate {angle} => {
                r_img = rotate(&r_img, *angle);
            },

            TransformType::Compress(quality) => {
                r_img = compress(&r_img, *quality);
            }
        }
        r_img
    }
}


impl ImageAction for TransformAction {
    fn apply(&mut self, img: PhotonImage) -> PhotonImage {
        self.transform_apply(img)
    }
}

