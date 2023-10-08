use photon_rs::native::{open_image, save_image};



use image_filter::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("pic/IMG_0171.jpeg")?;

    // Increment the red channel by 40
    photon_rs::channels::alter_red_channel(&mut img, 40);

    // Write file to filesystem.
    save_image(img, "pic/IMG_0171_after.jpeg")?;

    Ok(())
}