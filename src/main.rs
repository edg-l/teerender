//#![warn(clippy::nursery)]
//#![warn(clippy::pedantic)]

use std::net::SocketAddr;
use std::io::Cursor;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView, ImageBuffer, open, RgbaImage, Pixel, SubImage, DynamicImage, Rgba};

use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}

// blob:https://ddnet.org/704f4903-7d99-440b-9826-208a9bb00cf6

async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    /*
    let app = Router::new().route("/:skin", get(render));

    let addr = SocketAddr::from((
        [127, 0, 0, 1],
        std::env::var("TEERENDER_LISTEN_PORT")
            .map(|x| x.parse().unwrap())
            .unwrap_or(3000),
    ));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    */

    let x = color_code_base(7985705, false);
    dbg!(x);

    // 256, 128
    let img = ImageReader::open("skins/brownbear.png")?.decode()?;

    let mut def = RgbaImage::new(96, 64);

    let filter = FilterType::Triangle;

    let back_feet_shadow = img.crop_imm(192, 64, 64, 32).resize(64, 30, filter);
    blend_region(&mut def.sub_image(8, 32, 64, 30), &back_feet_shadow);

    let body_shadow = img.crop_imm(96, 0, 96, 96).resize(64, 64, filter);
    blend_region(&mut def.sub_image(14, 0, 64, 64), &body_shadow);

    let front_feet_shadow = img.crop_imm(192, 64, 64, 32).resize(64, 30, filter);
    blend_region(&mut def.sub_image(24, 32, 64, 30), &front_feet_shadow);

    let back_feet = img.crop_imm(192, 32, 64, 32).resize(64, 30, filter);
    blend_region(&mut def.sub_image(8, 32, 64, 30), &back_feet);

    let body = img.crop_imm(0, 0, 96, 96).resize(64, 64, filter);
    blend_region(&mut def.sub_image(14, 0, 64, 64), &body);

    let front_feet = img.crop_imm(192, 32, 64, 32).resize(64, 30, filter);
    blend_region(&mut def.sub_image(24, 32, 64, 30), &front_feet);

    let left_eye = img.crop_imm(64, 96, 32, 32).resize(26, 26, filter);
    blend_region(&mut def.sub_image(36, 17, 26, 26), &left_eye);

    let right_eye = left_eye.fliph();
    blend_region(&mut def.sub_image(36 + 9, 17, 26, 26), &right_eye);

    def.save("result.png")?;

    Ok(())
}

fn blend_region(region: &mut SubImage<&mut ImageBuffer<Rgba<u8>, Vec<u8>>>, from: &DynamicImage) {
    for (x, y, pixel) in from.pixels() {
        let mut p = region.get_pixel(x, y);
        p.blend(&pixel);
        region.put_pixel(x, y, p);
    }
}

#[derive(Debug, Deserialize)]
struct RenderParams {
    body_color: Option<u32>,
    feet_color: Option<u32>
}

// basic handler that responds with a static string
async fn render(Path(skin): Path<String>, Query(params): Query<RenderParams>) -> &'static str {
    todo!()
}

#[derive(Debug, Clone, Copy)]
struct HSLAColor {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub a: f32,
}

#[allow(clippy::cast_precision_loss)]
fn color_code_base(col: u32, alpha: bool) -> HSLAColor {
    let a = if alpha {
        ((col >> 24) & 0xFF) as f32 / 255.0f32
    } else {
        1.0f32
    };
    let x = ((col >> 16) & 0xFF) as f32 / 255.0f32;
    let y = ((col >> 8) & 0xFF) as f32 / 255.0f32;
    let z = (col & 0xFF) as f32 / 255.0f32;

    HSLAColor { x, y, z, a }
}

/*

player_color_body 7985705
player_color_feet 9568511
*/
