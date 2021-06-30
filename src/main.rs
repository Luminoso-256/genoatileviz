use std::path::Path;
use std::string;

use image::GenericImageView;
use raylib::prelude::*;

/*
 0: industry/airport/military/otherwise considered bad for gameplay
 1: major road/rails (looks like road)
 2: road
 3: minor road
 4: cycle path
 5: (never seen in my data)
 6: sand
 7: docks
 8: path
 9: water
10: playground
11: farmland
12: grass
13: trees
14: building
15: unclassified (looks like grass)
*/

fn rgb_to_mce(r: i32, g: i32, b: i32) -> i32 {
    let mut hex: i32 = r / 16;
    hex = (hex << 8) + g / 16;
    hex = (hex << 8) + b / 16;
    hex % 17
}

struct Pixel {
    //MCE Color Code
    color: i32,
    x: i32,
    y: i32,
}

const COLOR_0: Color = Color::new((0.3 * 255.0) as u8, (0.5 * 255.0) as u8, (0.1 * 255.0) as u8, 255);
const COLOR_1: Color = Color::new((0.123 * 255.0) as u8, (0.125 * 255.0) as u8, (0.125 * 255.0) as u8, 255);
const COLOR_2: Color = Color::new((0.123 * 255.0) as u8, (0.125 * 255.0) as u8, (0.125 * 255.0) as u8, 255);
const COLOR_3: Color = Color::new((0.2 * 255.0) as u8, (0.184 * 255.0) as u8, (0.188 * 255.0) as u8, 255);
const COLOR_4: Color = Color::new((0.7 * 255.0) as u8, (0.6 * 255.0) as u8, (0.2 * 255.0) as u8, 255);
const COLOR_5: Color = Color::new((0.4 * 255.0) as u8, (0.4 * 255.0) as u8, (0.4 * 255.0) as u8, 255);
const COLOR_6: Color = Color::new((0.949 * 255.0) as u8, (0.8 * 255.0) as u8, (0.568 * 255.0) as u8, 255);
const COLOR_7: Color = Color::new((0.6 * 255.0) as u8, (0.4 * 255.0) as u8, (0.25 * 255.0) as u8, 255);
const COLOR_8: Color = Color::new((0.313 * 255.0) as u8, (0.3 * 255.0) as u8, (0.309 * 255.0) as u8, 255);
const COLOR_9: Color = Color::new((0.1 * 255.0) as u8, (0.4 * 255.0) as u8, (0.6 * 255.0) as u8, 255);
const COLOR_10: Color = Color::new((0.325 * 255.0) as u8, (0.6 * 255.0) as u8, (0.1 * 255.0) as u8, 255);
const COLOR_11: Color = Color::new((0.3 * 255.0) as u8, (0.5 * 255.0) as u8, (0.1 * 255.0) as u8, 255);
const COLOR_12: Color = Color::new((0.2 * 255.0) as u8, (0.5 * 255.0) as u8, (0.1 * 255.0) as u8, 255);
const COLOR_13: Color = Color::new((0.2 * 255.0) as u8, (0.4 * 255.0) as u8, (0.1 * 255.0) as u8, 255);
const COLOR_14: Color = Color::new((0.325 * 255.0) as u8, (0.5 * 255.0) as u8, (0.1 * 255.0) as u8, 255);
const COLOR_15: Color = Color::new((0.25 * 255.0) as u8, (0.5 * 255.0) as u8, (0.1 * 255.0) as u8, 255);


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("GenoaTileViz")
        .build();
    let mut renderable_pixels: Vec<Pixel> = Vec::new();
    let mut tile_img = image::open(&Path::new("seattle.png")).unwrap();
    for pixel in tile_img.pixels() {
        renderable_pixels.push(Pixel {
            color: rgb_to_mce(pixel.2[0] as i32, pixel.2[1] as i32, pixel.2[2] as i32),
            x: pixel.0 as i32,
            y: pixel.1 as i32,
        });
    }
    let mut camera = Camera3D::perspective(
        Vector3::new(4.0, 2.0, 4.0),
        Vector3::new(0.0, 1.8, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );
    rl.set_camera_mode(&camera, CameraMode::CAMERA_FREE);
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.update_camera(&mut camera);

        d.clear_background(Color::WHITE);
        let mut d3 = d.begin_mode3D(&camera);
        for pixel in &renderable_pixels {
            match pixel.color {
                /* WARNING: TERRIBLE CODE AHEAD! */
                0 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.2, 0.1,
                                  COLOR_0),

                1 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.2, 0.1,
                                  COLOR_1),

                2 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.2, 0.1,
                                  COLOR_2),

                3 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.2, 0.1,
                                  COLOR_3),

                4 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.2, 0.1,
                                  COLOR_4),

                5 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.4, 0.1,
                                  COLOR_5),

                6 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.2, 0.1,
                                  COLOR_6),

                7 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.2, 0.1,
                                  COLOR_7),

                8 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.2, 0.1,
                                  COLOR_8),

                9 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.1, 0.1,
                                  COLOR_9),

                10 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.4, 0.1,
                                   COLOR_10),

                11 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.4, 0.1,
                                   COLOR_11),

                12 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 1.3, 0.1,
                                   COLOR_12),

                13 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.4, 0.1,
                                   COLOR_13),

                14 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.1, 0.1,
                                   COLOR_14),

                15 => d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.1, 0.1,
                                   COLOR_15),
                _ => {
                    d3.draw_cube(Vector3::new(pixel.x as f32 * 0.1, 0.0, pixel.y as f32 * 0.1), 0.1, 0.1, 0.1, Color::RED);
                  //  println!("INVALID: {}",&pixel.color)
                }
            }
        }
    }
}