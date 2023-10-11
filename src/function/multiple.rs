use photon_rs::{PhotonImage, Rgb, multiple::*};
use super::MultipleImpl;

pub struct Blend {
    pub mode: BlendMode
}

pub enum BlendMode {
    Overlay,
    Over,
    Atop,
    Xor,
    Multiply,
    Burn,
    Softlight,
    Hardlight,
    Difference,
    Lighten,
    Darken,
    Dodge,
    Plus,
    Exclusion
}

impl BlendMode {
    pub fn to_str(&self) -> &str {
        match self {
            BlendMode::Overlay => "overlay",
            BlendMode::Over => "over",
            BlendMode::Atop => "atop",
            BlendMode::Xor => "xor",
            BlendMode::Multiply => "multiply",
            BlendMode::Burn => "burn",
            BlendMode::Softlight => "softlight",
            BlendMode::Hardlight => "hardlight",
            BlendMode::Difference => "difference",
            BlendMode::Lighten => "lighten",
            BlendMode::Darken => "darken",
            BlendMode::Dodge => "dodge",
            BlendMode::Plus => "plus",
            BlendMode::Exclusion => "exclusion"
        }
    }
}

pub struct Fade {
    pub start_x: i32,
    pub end_x: i32,
    pub start_y: i32,
    pub end_y: i32
}

pub struct ApplyGradient;

pub struct CreateGradient {
    pub width: u32,
    pub height: u32
}

pub struct ReplaceBg {
    pub rgb: Rgb
}

pub struct Watermark {
    pub x: u32, 
    pub y: u32
}

impl MultipleImpl for Watermark {
    fn multiple(&self, img: &mut PhotonImage, img2: &PhotonImage) {
        watermark(img, img2, self.x, self.y);
    }
}

impl MultipleImpl for ReplaceBg {
    fn multiple(&self,img: &mut PhotonImage,img2: &PhotonImage) {
        replace_background(img, img2, &self.rgb);
    }
}

impl MultipleImpl for Fade {
    fn multiple(&self, img: &mut PhotonImage, img2: &PhotonImage) {
        fade(img, img2, self.start_x, self.end_x, self.start_y, self.end_y);
    }
}

impl MultipleImpl for CreateGradient {
    fn multiple(&self, img: &mut PhotonImage,img2: &PhotonImage) {
        *img = create_gradient(self.width, self.height)
    }
}

impl MultipleImpl for Blend {
    fn multiple(&self, img: &mut PhotonImage, img2: &PhotonImage) {
        blend(img, img2, self.mode.to_str());
    }
}

impl MultipleImpl for ApplyGradient {
    fn multiple(&self, img: &mut PhotonImage, img2: &PhotonImage) {
        apply_gradient(img);
    }
}