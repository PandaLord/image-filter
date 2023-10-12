use photon_rs::{monochrome::*, PhotonImage};

use super::MonoImpl;

pub struct Monochrome {
    r_offset: u32,
    g_offset: u32,
    b_offset: u32,
}

pub struct Sepia;

pub struct Decompose {
    d_type: DecomposeType,
}

pub enum DecomposeType {
    Max,
    Min,
}

pub struct Desaturate;

pub struct Grayscale {
    channel: GrayscaleChannel,
}

pub enum GrayscaleChannel {
    Blue,
    Green,
    Red,
    Average,
    HumanCorrect,
    Shades(u8),
}

pub struct Threshold {
    threshold: u32,
}

impl MonoImpl for Monochrome {
    fn mono(&self, img: &mut PhotonImage) {
        monochrome(img, self.r_offset, self.g_offset, self.b_offset)
    }
}

impl MonoImpl for Grayscale {
    fn mono(&self, img: &mut PhotonImage) {
        match self.channel {
            GrayscaleChannel::Blue => b_grayscale(img),
            GrayscaleChannel::Green => g_grayscale(img),
            GrayscaleChannel::Red => r_grayscale(img),
            GrayscaleChannel::Average => grayscale(img),
            GrayscaleChannel::HumanCorrect => grayscale_human_corrected(img),
            GrayscaleChannel::Shades(shades) => grayscale_shades(img, shades),
        }
    }
}

impl MonoImpl for Sepia {
    fn mono(&self, img: &mut PhotonImage) {
        sepia(img)
    }
}

impl MonoImpl for Desaturate {
    fn mono(&self, img: &mut PhotonImage) {
        desaturate(img)
    }
}

impl MonoImpl for Decompose {
    fn mono(&self, img: &mut PhotonImage) {
        match self.d_type {
            DecomposeType::Max => decompose_max(img),
            DecomposeType::Min => decompose_min(img),
        }
    }
}

impl MonoImpl for Threshold {
    fn mono(&self, img: &mut PhotonImage) {
        threshold(img, self.threshold)
    }
}
