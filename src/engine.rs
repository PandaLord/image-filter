use photon_rs::PhotonImage;
use crate::{function::transform::*, prelude::{TransformAction, TransformImpl, MultipleImpl, MultipleAction, MonoAction, MonoImpl}};
pub trait Engine {
    fn transform(&mut self, actions: &[TransformAction]);
    fn multiple(&mut self, img2: &PhotonImage, actions: &[MultipleAction]);
    fn mono(&mut self, actions: &[MonoAction]);
}

pub struct Photon(pub PhotonImage);

impl Engine for Photon {
    fn transform(&mut self, actions: &[TransformAction]) {
        for action in actions.iter() {
            action.transform(&mut self.0);
        }
    }

    fn multiple(&mut self, img2: &PhotonImage, actions: &[MultipleAction]) {
        for action in actions.iter() {
            action.multiple(&mut self.0, img2);
        }
    }
    
    fn mono(&mut self, actions: &[MonoAction]) {
        for action in actions.iter() {
            action.mono(&mut self.0);
        }
    }

}

impl Photon {
    pub fn new(img: PhotonImage) -> Self {
        Self(img)
    }
}
