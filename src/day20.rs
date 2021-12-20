use super::common;
use std::path::Path;

fn print_image(img: &[char], width: usize) {
    for i in 0..img.len() {
        if (i % width) == 0 {
            print!("\n");
        }
        print!("{}", img[i]);
    }
    print!("\n");
}

fn lookup_str_to_number(lookup_str: &[char]) -> u32 {
    assert_eq!(9, lookup_str.len());
    let mut output = 0;
    for i in 0..lookup_str.len() {
        if lookup_str[i] == '#' {
            output |= 1 << (8 - i);
        } else {
            assert_eq!('.', lookup_str[i]);
        }
    }
    output
}

/// Returns a triple of (new image, new width, new height)
fn enhance_image(default_pixel: char, image: &[char], width: usize, height: usize, enhancer: &[char]) -> (Vec<char>, usize, usize) {
    let new_image_width = width + 2; // Add row top and bottom
    let new_image_height = height + 2; // Add col left and right
    let mut new_image = Vec::new();
    new_image.resize(new_image_width * new_image_height, '.');

    for col in 0..(new_image_height as i32) {
        for row in 0..(new_image_width as i32) {
            let src_row = row - 1;
            let src_col = col - 1;

            let mut lookup_string = Vec::new();

            // Tap the image in descending row order
            for j in [-1,0,1] {
                let y_tap = src_col + j;
                
                for i in [-1,0,1] {
                    let x_tap = src_row + i;
                    
                    if y_tap < 0 || y_tap >= (height as i32) {
                        lookup_string.push(default_pixel);
                        continue;
                    } else if x_tap < 0 || x_tap >= (width as i32) {
                        lookup_string.push(default_pixel);
                        continue;
                    }

                    let tap = x_tap as usize + (y_tap as usize * width);
                    lookup_string.push(image[tap]);
                }
            }
                
            let lookup_idx = lookup_str_to_number(&lookup_string);
            let new_image_idx = row as usize + (col as usize * new_image_width);
            new_image[new_image_idx] = enhancer[lookup_idx as usize];
        }
    }


    print_image(&image, width);
    println!("====== Before ======");
    print_image(&new_image, new_image_width);
    println!("====== After ======");

    (new_image, new_image_width, new_image_height)
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day20_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let enhancement = {
        bytes.iter().map(|b| *b as char)
            .take_while(|c| !common::is_newline(*c))
            .collect::<Vec<_>>()
    };

    let (mut image, mut width) = {
        let mut skip_to = 0;
        while !common::is_newline(bytes[skip_to] as char) {
            skip_to += 1;
        }
        while bytes[skip_to] as char != '#' &&
              bytes[skip_to] as char != '.' {
            skip_to += 1;
        }

        let width = bytes.iter().skip(skip_to).position(|b| common::is_newline(*b as char)).unwrap();
        let image = bytes.iter().skip(skip_to)
            .map(|b| *b as char)
            .filter(|c| (*c == '#') || (*c == '.'))
            .collect::<Vec<_>>();
        (image, width)
    };

    let mut height = image.len() / width;
    let iterations = 2;
    for gen in 0..iterations {
        let default_pixel = if gen % 2 == 0 { '.' } else { '#' };
        let (new_image, new_width, new_height) = enhance_image(default_pixel, &image, width, height, &enhancement);
        image = new_image;
        width = new_width;
        height = new_height;
    }

    let lit_pixels = image.iter().filter(|c| **c == '#').count();
    println!("Num lit pixels {}", lit_pixels);
}