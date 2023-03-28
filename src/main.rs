use rand::prelude::*;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::process;

const SIZE: usize = 100;

fn init_screen(screen: &mut [i32]) {
    let mut rng = thread_rng();

    for row in 1..SIZE - 1 {
        for col in 1..SIZE - 1 {
            screen[row * SIZE + col] = rng.gen_range(0..=255);
        }
    }
}

fn run(iterations: usize, addition: i32, screen: &mut [i32]) {
    let mut rng = thread_rng();

    for _ in 0..iterations {
        for row in 1..SIZE - 1 {
            for col in 1..SIZE - 1 {
                let mut total: i32 = 0;

                total = total.wrapping_add(screen[(row - 1) * SIZE + (col - 1)]);
                total = total.wrapping_add(screen[(row - 1) * SIZE + col]);
                total = total.wrapping_add(screen[(row - 1) * SIZE + (col + 1)]);
                total = total.wrapping_add(screen[row * SIZE + (col - 1)]);
                total = total.wrapping_add(screen[row * SIZE + (col + 1)]);
                total = total.wrapping_add(screen[(row + 1) * SIZE + (col - 1)]);
                total = total.wrapping_add(screen[(row + 1) * SIZE + col]);
                total = total.wrapping_add(screen[(row + 1) * SIZE + (col + 1)]);
                if addition < 0 {
                    total += rng.gen_range(0..=255);
                } else {
                    total += addition;
                }
                total /= 8;

                screen[row * SIZE + col] = total;
            }
        }
    }
}

fn save_png(filename: &str, screen: &[i32]) {
    let path = Path::new(filename);
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, SIZE as u32, SIZE as u32);

    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let mut data8: Vec<u8> = Vec::new();

    for p in screen {
        data8.push(*p as u8)
    }

    writer.write_image_data(data8.as_slice()).unwrap();
}

fn usage_exit(args: &[String]) {
    eprintln!("usage: {} filename iterations [addition]", args[0]);
    process::exit(1);
}

fn parse_cl() -> (String, usize, i32) {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        usage_exit(&args);
    }

    let iterations: usize = match args[2].parse() {
        Ok(v) => v,
        Err(_) => {
            usage_exit(&args);
            0
        }
    };

    let filename = args[1].clone();

    let addition: i32 = if args.len() == 4 {
        match args[3].parse() {
            Ok(v) => v,
            Err(_) => {
                usage_exit(&args);
                0
            }
        }
    } else {
        -1
    };

    (filename, iterations, addition)
}

fn main() {
    let mut screen = [0; SIZE * SIZE];

    let (filename, iterations, addition) = parse_cl();

    init_screen(&mut screen);
    run(iterations, addition, &mut screen);
    save_png(&filename, &screen);
}
