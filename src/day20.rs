use super::common;
use std::path::Path;

/// Same as get_box_taps, but the taps are in ascending row order:
/// x+ __
/// +y|[0 1 2]
///    [3 4 5]
///    [6 7 8]
pub fn get_box_taps_row_order(idx: usize, width: usize, height: usize) -> [Option<usize>; 9] {
    let (x, y) = {
        let (x, y) = common::get_grid_xy(idx, width, height);
        (x as i32, y as i32)
    };

    let mut coords: [Option<usize>; 9] = Default::default();
    let mut coord_i = 0;

    for d_y in [-1,0,1] {
        let next_y = y + d_y;
        if next_y < 0 || next_y >= height as i32 {
            continue;
        }
    
        for d_x in [-1,0,1] {
            let next_x = x + d_x;
            if next_x < 0 || next_x >= width as i32 {
                continue;
            }

            let tap = next_x as usize + (next_y as usize * width);
            coords[coord_i] = Some(tap);
            coord_i += 1;
        }
    }
    
    
    coords
}

fn print_image(img: &[char], width: usize) {
    for i in 0..img.len() {
        if (i % width) == 0 {
            print!("\n");
        }
        print!("{}", img[i]);
    }
    print!("\n");
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day20_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let enhancement = {
        bytes.iter().map(|b| *b as char)
            .take_while(|c| !common::is_newline(*c))
            .collect::<Vec<_>>()
    };

    let (image, width) = {
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

    let height = image.len() / width;

    let new_image_width = width + 2; // Add row top and bottom
    let new_image_height = height + 2; // Add col left and right
    let mut new_image = Vec::new();
    new_image.resize(new_image_width * new_image_height, '.');


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
                        lookup_string.push('.');
                        continue;
                    }
                    else if x_tap < 0 || x_tap >= (width as i32) {
                        lookup_string.push('.');
                        continue;
                    }

                    let tap = x_tap as usize + (y_tap as usize * width);
                    lookup_string.push(image[tap]);
                }
            }
              
            let lookup_idx = lookup_str_to_number(&lookup_string);
            let new_image_idx = row as usize + (col as usize * new_image_width);
            new_image[new_image_idx] = enhancement[lookup_idx as usize];
        }
    }

    println!("Before:");
    print_image(&image, width);
    println!("After:");
    print_image(&new_image, new_image_width);
}