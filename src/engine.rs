use photon_rs::PhotonImage;

use crate::prelude::Action;
use crate::function::ImageAction;

pub trait Engine {
    fn apply(&mut self, actions: &[Action]);
}



pub struct Photon(pub PhotonImage);

impl Engine for Photon {
    fn apply(&mut self, actions: &[Action]) {
        for action in actions.iter(){
            action.apply(&mut self.0);
        }
    }
}

impl Photon {
    pub fn new(img: PhotonImage) -> Self {
        Self(img)
    }
}



