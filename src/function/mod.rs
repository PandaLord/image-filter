mod transform;
mod correction;
mod convolution;
mod channel_manipulation;
mod mono_effect;
mod colour_manipulation;
mod filters;
mod text_apply;
mod watermark;
mod blending;



use photon_rs::PhotonImage;
use photon_rs::native::save_image;
use anyhow::{Result, Context};


pub trait ImageAction {
    fn apply(&mut self, img: PhotonImage) -> PhotonImage;
}

pub struct ImageHandler {
    pub image: photon_rs::PhotonImage,
    pub actions: Vec<Box<dyn ImageAction>>,
}

impl ImageHandler {
    pub fn new(image: photon_rs::PhotonImage) -> Self {
        Self {
            image,
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: Box<dyn ImageAction>) {
        self.actions.push(action);
    }

    pub fn apply(&mut self) {
        let mut img = self.image.clone();
        for action in &mut self.actions {
            img = action.apply(img);
        }
        self.image = img;
    }
    pub fn output(self, path: &str) -> Result<()>{
        save_image(self.image, path).context("save image error")
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use super::filters::{FilterAction, FilterType};
    use super::transform::{TransformType, TransformAction};

    #[test]
    fn test_transform() {
        let img = photon_rs::native::open_image("pic/IMG_0171.jpeg").unwrap();
        let mut handler = ImageHandler::new(img);
        // handler.add_action(Box::new(TransformAction(TransformType::Resize {width: 100, height: 100, filter: SamplingFilter::Nearest})));
        handler.add_action(Box::new(TransformAction(TransformType::Fliph)));
        handler.add_action(Box::new(TransformAction(TransformType::Flipv)));
        handler.add_action(Box::new(TransformAction(TransformType::Rotate {angle: 90})));
        handler.apply();
        assert!(handler.output("pic/IMG_0171_after.jpeg").is_ok());
    }

    #[test]
    fn test_multiple_actions() {
        let img = photon_rs::native::open_image("pic/IMG_0171.jpeg").unwrap();
        let mut handler = ImageHandler::new(img);
        handler.add_action(Box::new(TransformAction(TransformType::Fliph)));
        handler.add_action(Box::new(FilterAction(FilterType::Oceanic)));
        handler.apply();
        assert!(handler.output("pic/IMG_0171_after.jpeg").is_ok());
    }

}