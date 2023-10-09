mod blending;
mod channel_manipulation;
mod colour_manipulation;
mod convolution;
mod correction;
mod filters;
mod mono_effect;
mod text_apply;
mod watermark;

use anyhow::{Context, Result};
use photon_rs::native::save_image;
use photon_rs::PhotonImage;
use enum_dispatch::enum_dispatch;


mod transform {
    use photon_rs::transform::*;
    use photon_rs::PhotonImage;
    use super::ImageAction;

    pub struct Resize {
        pub width: u32,
        pub height: u32,
        pub filter: SamplingFilter,
    }

    pub struct Resample {
        pub dst_width: usize, 
        pub dst_height: usize
    }

    pub struct Crop {
        pub x1: u32,
        pub x2: u32,
        pub y1: u32,
        pub y2: u32,
    }
    
    pub struct Fliph;

    pub struct Filpv;

    pub struct Padding; 

    pub struct Rotate {
        pub angle: i32,
    }
    pub struct Compress {
        pub quality: u8,
    }

    impl ImageAction for Resize {
        fn apply(&self, img: &mut PhotonImage) {
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
    impl ImageAction for Resample {
        fn apply(&self, img: &mut PhotonImage) {
            *img = resample(img, self.dst_width, self.dst_height);
        }
    }
    impl ImageAction for Crop {
        fn apply(&self, img: &mut PhotonImage) {
            crop(img, self.x1, self.x2, self.y1,self.y2);
        }
    }

    impl ImageAction for Fliph {
        fn apply(&self, img: &mut PhotonImage) {
            fliph(img);
        }
    }

    impl ImageAction for Filpv {
        fn apply(&self, img: &mut PhotonImage) {
            flipv(img);
        }
    }

    impl ImageAction for Rotate {
        fn apply(&self, img: &mut PhotonImage) {
            *img = rotate(img, self.angle);
        }
    }

    impl ImageAction for Compress {
        fn apply(&self, img: &mut PhotonImage) {
            *img = compress(img, self.quality);
        }
    }
    impl ImageAction for Padding {
        fn apply(&self, img: &mut PhotonImage) {
            todo!()
        }
    }
}

#[enum_dispatch]
pub trait ImageAction {
    fn apply(&self, img: &mut PhotonImage);
}

#[enum_dispatch(ImageAction)]
pub enum Action {
    Resize(self::transform::Resize),
    Resample(self::transform::Resample),
    Crop(self::transform::Crop),
    Fliph(self::transform::Fliph),
    Flipv(self::transform::Filpv),
    Padding(self::transform::Padding),
    Rotate(self::transform::Rotate),
    Compress(self::transform::Compress),
}


#[cfg(test)]
mod test {
    use photon_rs::transform::SamplingFilter;

    use crate::engine::{Photon, Engine};

    use super::*;


    #[test]
    fn test_transform() {
        let img = photon_rs::native::open_image("pic/test.jpg").unwrap();
        let mut handler = Photon::new(img);

        let actions = vec![
            Action::Resize(self::transform::Resize {
                width: 100,
                height: 100,
                filter: photon_rs::transform::SamplingFilter::Nearest,
            }),
            Action::Fliph(self::transform::Fliph),
            Action::Flipv(self::transform::Filpv),
            Action::Rotate(self::transform::Rotate { angle: 90 }),
        ];
        handler.apply(actions.as_ref());
        assert!(save_image(handler.0, "pic/test_after.jpg").is_ok());
    }
}
