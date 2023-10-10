use enum_dispatch::enum_dispatch;
use photon_rs::PhotonImage;

mod blending;
mod channel_manipulation;
mod colour_manipulation;
mod convolution;
mod correction;
pub mod filters;
mod mono_effect;
mod text_apply;
mod watermark;
pub mod transform;
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




#[cfg(test)]
mod test {
    use super::*;
    use crate::engine::{Engine, Photon};
    use photon_rs::native::save_image;
    use transform::*;
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
        handler.apply(actions.as_ref());
        assert!(save_image(handler.0, "pic/test_after.jpg").is_ok());
    }
}
