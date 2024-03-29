use std::fs::File;
use std::io::Read;
use std::ops::IndexMut;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let layers = input
        .as_bytes()
        .chunks_exact(25 * 6)
        .map(|c| Vec::from(c))
        .map(|v| v.iter().map(|u| *u - 48).collect())
        .collect::<Vec<Vec<u8>>>();

    let mut least_zeroes = 99999;
    let mut target = Vec::new();
    for layer in layers {
        let num_zeroes = count_occurrences(&layer, 0);
        if num_zeroes < least_zeroes {
            target = layer;
            least_zeroes = num_zeroes;
        }
    }
    let ones = count_occurrences(&target, 1);
    let twos = count_occurrences(&target, 2);

    println!("{} = {} * {}", ones * twos, ones, twos);
}

fn count_occurrences(layer: &Vec<u8>, target: u8) -> i32 {
    layer
        .iter()
        .fold(0, |count, i| if *i == target { count + 1 } else { count })
}

fn part2() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut layers = input
        .as_bytes()
        .chunks_exact(25 * 6)
        .map(|c| Vec::from(c))
        .map(|v| v.iter().map(|u| *u - 48).collect())
        .collect::<Vec<Vec<u8>>>();

    layers.reverse();

    let mut image = Vec::new();
    image.resize(25 * 6, 2);

    let image = layers.iter().fold(image, |mut image, layer| {
        for (index, &new_pixel) in layer.iter().enumerate() {
            let existing_pixel = *image.get(index).unwrap_or(&2);

            *image.index_mut(index) = if new_pixel == 2 {
                existing_pixel
            } else {
                new_pixel
            };
        }

        image
    });

    for line in image.iter().as_slice().chunks_exact(25) {
        for byte in line {
            if *byte == 0 {
                print!("\u{2588}")
            } else {
                print!(" ");
            }
        }

        println!();
    }
}

#[cfg(test)]
mod test {
    use crate::count_occurrences;

    #[test]
    fn test_count() {
        let vec: Vec<u8> = vec![0, 1, 1];

        assert_eq!(0, count_occurrences(&vec, 2));
        assert_eq!(1, count_occurrences(&vec, 0));
        assert_eq!(2, count_occurrences(&vec, 1));
    }
}
