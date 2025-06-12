mod args;
mod tracing_setup;

use std::rc::Rc;

use ::tracing::info;
use args::Args;
use bevy_math::IRect;
use bevy_math::IVec2;
use clap::Parser;
use image::RgbImage;
use winc::prelude::*;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::try_parse()?;
    tracing_setup::setup_tracing(&args.global, std::io::stdout)?;
    info!("Ahoy!");

    let monitors = get_all_monitors()?;
    let capture_rect = IRect::from_corners(
        IVec2 {
            x: args.x,
            y: args.y,
        },
        IVec2 {
            x: args.x + args.width,
            y: args.y + args.height,
        },
    );
    let mut texture = RgbImage::new(args.width as u32, args.height as u32);
    for monitor in monitors {
        let monitor_rect = IRect::from_corners(
            IVec2 {
                x: monitor.info.rect.left,
                y: monitor.info.rect.top,
            },
            IVec2 {
                x: monitor.info.rect.right,
                y: monitor.info.rect.bottom,
            },
        );
        let monitor_capture_rect = monitor_rect.intersect(capture_rect);
        if monitor_capture_rect.is_empty() {
            continue;
        }
        let offset_x = monitor_capture_rect.min.x - capture_rect.min.x;
        let offset_y = monitor_capture_rect.min.y - capture_rect.min.y;
        let monitor_name = monitor.info.name.clone();
        let monitor_capture_rect_dbg = monitor_capture_rect;
        let capturer = get_monitor_capturer(
            Rc::new(monitor),
            RECT {
                left: monitor_capture_rect.min.x,
                top: monitor_capture_rect.min.y,
                right: monitor_capture_rect.max.x,
                bottom: monitor_capture_rect.max.y,
            },
        );
        let capture: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = capturer.capture(&mut Metrics::None)?;
        for y in 0..capture.height() {
            for x in 0..capture.width() {
                let pixel = capture.get_pixel(x, y);
                let image_x = (x as i32 + offset_x) as u32;
                let image_y = (y as i32 + offset_y) as u32;
                if image_x < texture.width() && image_y < texture.height() {
                    let rgb_pixel = image::Rgb([pixel[0], pixel[1], pixel[2]]);
                    texture.put_pixel(image_x, image_y, rgb_pixel);
                }
            }
        }
        info!(
            "Captured monitor {} at rect {:?} to texture at offset ({}, {})",
            monitor_name, monitor_capture_rect_dbg, offset_x, offset_y
        );
    }
    // Optionally, save the texture to a file for debugging
    texture.save(&args.out)?;
    Ok(())
}
