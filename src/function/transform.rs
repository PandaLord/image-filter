use photon_rs::transform::*;
use photon_rs::PhotonImage;
use photon_rs::Rgba;

use super::TransformImpl;

pub struct Resize {
    pub width: u32,
    pub height: u32,
    pub filter: SamplingFilter,
}

pub struct Resample {
    pub dst_width: usize,
    pub dst_height: usize,
}

pub struct Crop {
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub y2: u32,
}

pub struct Fliph;

pub struct Filpv;

pub struct Padding {
    pub padding: u32,
    pub p_type: PaddingSpec,
    pub p_rgba: Rgba,
}

pub enum PaddingSpec {
    Top,
    Bottom,
    Left,
    Right,
    All,
}
pub struct Rotate {
    pub angle: i32,
}
pub struct Compress {
    pub quality: u8,
}

impl TransformImpl for Resize {
    fn transform(&self, img: &mut PhotonImage) {
        let filter = match self.filter {
            SamplingFilter::Nearest => SamplingFilter::Nearest,
            SamplingFilter::Triangle => SamplingFilter::Triangle,
            SamplingFilter::CatmullRom => SamplingFilter::CatmullRom,
            SamplingFilter::Gaussian => SamplingFilter::Gaussian,
            SamplingFilter::Lanczos3 => SamplingFilter::Lanczos3,
        };
        *img = resize(img, self.width, self.height, filter);
    }
}
impl TransformImpl for Resample {
    fn transform(&self, img: &mut PhotonImage) {
        *img = resample(img, self.dst_width, self.dst_height);
    }
}
impl TransformImpl for Crop {
    fn transform(&self, img: &mut PhotonImage) {
        crop(img, self.x1, self.x2, self.y1, self.y2);
    }
}

impl TransformImpl for Fliph {
    fn transform(&self, img: &mut PhotonImage) {
        fliph(img);
    }
}

impl TransformImpl for Filpv {
    fn transform(&self, img: &mut PhotonImage) {
        flipv(img);
    }
}

impl TransformImpl for Rotate {
    fn transform(&self, img: &mut PhotonImage) {
        *img = rotate(img, self.angle);
    }
}

impl TransformImpl for Compress {
    fn transform(&self, img: &mut PhotonImage) {
        *img = compress(img, self.quality);
    }
}
impl TransformImpl for Padding {
    fn transform(&self, img: &mut PhotonImage) {
        let p_rgba = Rgba::new(
            self.p_rgba.get_red(),
            self.p_rgba.get_green(),
            self.p_rgba.get_blue(),
            self.p_rgba.get_alpha(),
        );
        *img = match self.p_type {
            PaddingSpec::Top => padding_top(img, self.padding, p_rgba),
            PaddingSpec::Bottom => padding_bottom(img, self.padding, p_rgba),
            PaddingSpec::Left => padding_left(img, self.padding, p_rgba),
            PaddingSpec::Right => padding_right(img, self.padding, p_rgba),
            PaddingSpec::All => padding_uniform(img, self.padding, p_rgba),
        };
    }
}


