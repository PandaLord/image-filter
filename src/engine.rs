use photon_rs::PhotonImage;
use crate::{function::transform::*, prelude::{TransformAction, TransformImpl}};
pub trait Engine {
    fn apply(&mut self, actions: &[TransformAction]);
}

pub struct Photon(pub PhotonImage);

impl Engine for Photon {
    fn apply(&mut self, actions: &[TransformAction]) {
        for action in actions.iter() {
            action.transform(&mut self.0);
        }
    }
}

impl Photon {
    pub fn new(img: PhotonImage) -> Self {
        Self(img)
    }
}
