use image::GrayImage;

fn main() {
    loop {
        let mut img = GrayImage::new(1920, 1080);

        // A single image, to be read from mounted volume
        let tile = image::open("picture-dir/orange-cat.png").unwrap();
        let gray_tile = image::imageops::grayscale(&tile);
        println!("tile dimensions {:?}", gray_tile.dimensions());

        // Tile grayed image out to desired dimensions
        image::imageops::tile(&mut img, &gray_tile);

        // Write the contents of this image to the Writer in PNG format.
        img.save("result-picture-dir/gray-tiled-cats.png").unwrap();

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
