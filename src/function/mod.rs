use enum_dispatch::enum_dispatch;
use photon_rs::PhotonImage;

mod blending;
mod channel_manipulation;
mod colour_manipulation;
mod convolution;
mod correction;
pub mod filters;
pub mod mono_effect;
mod text_apply;
mod watermark;
pub mod transform;
pub mod multiple;
use transform::*;

use filters::Filter;


#[enum_dispatch]
pub trait TransformImpl {
    fn transform(&self, img: &mut PhotonImage);
}

#[enum_dispatch(TransformImpl)]
pub enum TransformAction {
    Resize(Resize),
    Resample(Resample),
    Crop(Crop),
    Fliph(Fliph),
    Flipv(Filpv),
    Padding(Padding),
    Rotate(Rotate),
    Compress(Compress),
    Filter(Filter)
}


#[enum_dispatch]
pub trait MultipleImpl {
    fn multiple(&self, img: &mut PhotonImage, img2: &PhotonImage);
}


#[enum_dispatch(MultipleImpl)]
pub enum MultipleAction {
    Blend(multiple::Blend),
    Fade(multiple::Fade),
    ApplyGradient(multiple::ApplyGradient),
    CreateGradient(multiple::CreateGradient),
    ReplaceBg(multiple::ReplaceBg),
    Watermark(multiple::Watermark)
}

#[enum_dispatch]
pub trait MonoImpl {
    fn mono(&self, img: &mut PhotonImage);
}



#[enum_dispatch(MonoImpl)]
pub enum MonoAction {
    Desaturate(mono_effect::Desaturate),
    Decompose(mono_effect::Decompose),
    Grayscale(mono_effect::Grayscale),
    Monochrome(mono_effect::Monochrome),
    Sepia(mono_effect::Sepia),
    Threshold(mono_effect::Threshold)
}




#[cfg(test)]
mod test {
    use super::*;
    use crate::engine::{Engine, Photon};
    use photon_rs::{native::save_image, Rgb};
    #[test]
    fn test_transform() {
        let img = photon_rs::native::open_image("pic/test.jpg").unwrap();
        let mut handler = Photon::new(img);

        let actions = vec![
            TransformAction::Resize(Resize {
                width: 100,
                height: 100,
                filter: photon_rs::transform::SamplingFilter::Nearest,
            }),
            TransformAction::Fliph(Fliph),
            TransformAction::Flipv(Filpv),
            TransformAction::Rotate(Rotate { angle: 90 }),
        ];
        handler.transform(actions.as_ref());
        assert!(save_image(handler.0, "pic/test_after.jpg").is_ok());
    }

    #[test]
    fn multiple_test() {
        let img = photon_rs::native::open_image("pic/test_after.jpg").unwrap();
        let img2 = photon_rs::native::open_image("pic/test.jpg").unwrap();
        let mut handler = Photon::new(img);
        let actions = vec![
            MultipleAction::Blend(multiple::Blend {
                mode: multiple::BlendMode::Darken,
            }),
            MultipleAction::Fade(multiple::Fade {
                start_x: 0,
                end_x: 100,
                start_y: 0,
                end_y: 100,
            }),
            MultipleAction::ApplyGradient(multiple::ApplyGradient),
            MultipleAction::CreateGradient(multiple::CreateGradient {
                width: 100,
                height: 100,
            }),
            MultipleAction::ReplaceBg(multiple::ReplaceBg {
                rgb: Rgb::new(255,255,255)
            }),
            MultipleAction::Watermark(multiple::Watermark { x: 0, y: 0 }),
        ];
        handler.multiple(&img2, actions.as_ref());
        assert!(save_image(handler.0, "pic/test_multiple.jpg").is_ok());
    }

    #[test]
    fn mono_test() {
        let img = photon_rs::native::open_image("pic/test.jpg").unwrap();
        let mut handler = Photon::new(img);
        let actions = vec![
            MonoAction::Desaturate(mono_effect::Desaturate),
            MonoAction::Decompose(mono_effect::Decompose {
                d_type: mono_effect::DecomposeType::Max,
            }),
            MonoAction::Grayscale(mono_effect::Grayscale {
                channel: mono_effect::GrayscaleChannel::Blue,
            }),
            MonoAction::Monochrome(mono_effect::Monochrome {
                r_offset: 0,
                g_offset: 0,
                b_offset: 0,
            }),
            MonoAction::Sepia(mono_effect::Sepia),
            MonoAction::Threshold(mono_effect::Threshold { threshold: 100 }),
        ];
        handler.mono(actions.as_ref());
        assert!(save_image(handler.0, "pic/test_mono.jpg").is_ok());
    }
}
