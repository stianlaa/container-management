use image::GrayImage;

fn main() {
    // A single image, to be read from mounted volume
    let tile = image::open("orange-cat.png").unwrap();
    let gray_tile = image::imageops::grayscale(&tile);
    let mut img = GrayImage::new(1920, 1080);

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", gray_tile.dimensions());

    // Tile grayed image out to desired dimensions
    image::imageops::tile(&mut img, &gray_tile);

    // Write the contents of this image to the Writer in PNG format.
    img.save("gray-tiled-cats.png").unwrap();
}
