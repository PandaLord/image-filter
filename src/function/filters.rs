// use photon_rs::{PhotonImage, filters::filter};

use photon_rs::{PhotonImage, filters::filter};

use super::TransformImpl;

pub struct Filter {
    pub filter: FilterType,
}

// use super::ImageAction;

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

// 辅助函数，photon_rs 相应的方法里需要字符串
impl Filter {
    pub fn to_str(&self) -> Option<&'static str> {
        match self.filter {
            FilterType::Oceanic => Some("oceanic"),
            FilterType::Islands => Some("islands"),
            FilterType::Marine => Some("marine"),
            FilterType::Seagreen => Some("seagreen"),
            FilterType::Flagblue => Some("flagblue"),
            FilterType::Liquid => Some("liquid"),
            FilterType::Diamante => Some("diamante"),
            FilterType::Radio => Some("radio"),
            FilterType::Twenties => Some("twenties"),
            FilterType::Rosetint => Some("rosetint"),
            FilterType::Mauve => Some("mauve"),
            FilterType::Bluechrome => Some("bluechrome"),
            FilterType::Vintage => Some("vintage"),
            FilterType::Perfume => Some("perfume"),
            FilterType::Serenity => Some("serenity"),
            FilterType::Golden => Some("golden"),
            FilterType::Pastelpink => Some("pastelpink"),
            FilterType::Cali => Some("cali"),
            FilterType::Dramatic => Some("dramatic"),
            FilterType::Firenze => Some("firenze"),
            FilterType::Obsidian => Some("obsidian"),
            FilterType::Lofi => Some("lofi"),
        }
    }
}


impl TransformImpl for Filter {
    fn transform(&self, img: &mut PhotonImage) {
        filter(img, self.to_str().unwrap_or("oceanic"));
    }
}
// pub struct FilterAction(pub FilterType);

// impl<'a> Into<&'a str> for &mut FilterAction {
//     fn into(self) -> &'a str {
//         match self.0 {
//             FilterType::Oceanic => "oceanic",
//             FilterType::Islands => "islands",
//             FilterType::Marine => "marine",
//             FilterType::Seagreen => "seagreen",
//             FilterType::Flagblue => "flagblue",
//             FilterType::Liquid => "liquid",
//             FilterType::Diamante => "diamante",
//             FilterType::Radio => "radio",
//             FilterType::Twenties => "twenties",
//             FilterType::Rosetint => "rosetint",
//             FilterType::Mauve => "mauve",
//             FilterType::Bluechrome => "bluechrome",
//             FilterType::Vintage => "vintage",
//             FilterType::Perfume => "perfume",
//             FilterType::Serenity => "serenity",
//             FilterType::Golden => "golden",
//             FilterType::Pastelpink => "pastelpink",
//             FilterType::Cali => "cali",
//             FilterType::Dramatic => "dramatic",
//             FilterType::Firenze => "firenze",
//             FilterType::Obsidian => "obsidian",
//             FilterType::Lofi => "lofi",
//         }
//     }
// }

// impl ImageAction for FilterAction {
//     fn apply(&mut self, img: PhotonImage) -> PhotonImage {
//         self.filter_apply(img)
//     }
// }

// impl FilterAction {
//     fn filter_apply(&mut self, img: PhotonImage) -> PhotonImage {
//         let mut f_img = img;
//         let sample_filter: &str = self.into();
//         filter(&mut f_img, sample_filter);
//         f_img
//     }
// }