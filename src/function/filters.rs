use photon_rs::{PhotonImage, filters::filter};

use super::ImageAction;

pub enum FilterType {
    Oceanic,
    Islands,
    Marine,
    Seagreen,
    Flagblue,
    Liquid,
    Diamante,
    Radio,
    Twenties,
    Rosetint,
    Mauve,
    Bluechrome,
    Vintage,
    Perfume,
    Serenity,
    Golden,
    Pastelpink,
    Cali,
    Dramatic,
    Firenze,
    Obsidian,
    Lofi
}
pub struct FilterAction(pub FilterType);

impl<'a> Into<&'a str> for &mut FilterAction {
    fn into(self) -> &'a str {
        match self.0 {
            FilterType::Oceanic => "oceanic",
            FilterType::Islands => "islands",
            FilterType::Marine => "marine",
            FilterType::Seagreen => "seagreen",
            FilterType::Flagblue => "flagblue",
            FilterType::Liquid => "liquid",
            FilterType::Diamante => "diamante",
            FilterType::Radio => "radio",
            FilterType::Twenties => "twenties",
            FilterType::Rosetint => "rosetint",
            FilterType::Mauve => "mauve",
            FilterType::Bluechrome => "bluechrome",
            FilterType::Vintage => "vintage",
            FilterType::Perfume => "perfume",
            FilterType::Serenity => "serenity",
            FilterType::Golden => "golden",
            FilterType::Pastelpink => "pastelpink",
            FilterType::Cali => "cali",
            FilterType::Dramatic => "dramatic",
            FilterType::Firenze => "firenze",
            FilterType::Obsidian => "obsidian",
            FilterType::Lofi => "lofi",
        }
    }
}

impl ImageAction for FilterAction {
    fn apply(&mut self, img: PhotonImage) -> PhotonImage {
        self.filter_apply(img)
    }
}

impl FilterAction {
    fn filter_apply(&mut self, img: PhotonImage) -> PhotonImage {
        let mut f_img = img;
        let sample_filter: &str = self.into();
        filter(&mut f_img, sample_filter);
        f_img
    }
}