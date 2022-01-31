use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

use image::imageops::{resize, FilterType};
use image::io::Reader as ImageReader;

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static/").join(file);
    if path.is_dir() {
        return None;
    }
    NamedFile::open(path).await.ok()
}

#[get("/images/<file..>?<width>&<height>")]
async fn images(file: PathBuf, width: u32, height: u32) -> Option<NamedFile> {
    let temp_image_name = format!(
        "{}({}x{}).{}",
        &file.file_stem().unwrap().to_str().unwrap(),
        width.to_string(),
        height.to_string(),
        &file.extension().unwrap().to_str().unwrap()
    );

    let path = Path::new("static/images/temp").join(&temp_image_name);

    if path.exists() {
        NamedFile::open(Path::new(&path)).await.ok()
    } else {
        let img = ImageReader::open(Path::new("static/images").join(&file)).unwrap();

        let img = img.decode().unwrap();

        match resize(&img, width, height, FilterType::Gaussian).save(&path) {
            Ok(_) => NamedFile::open(Path::new(&path)).await.ok(),
            Err(_) => None,
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Public route ignited", |rocket| async {
        rocket.mount("/public", routes![files, images])
    })
}
