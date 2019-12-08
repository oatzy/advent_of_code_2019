use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::ops::Add;
use std::u32::MAX;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Pixel {
    Black,
    White,
    Transparent,
}

impl From<u32> for Pixel {
    fn from(value: u32) -> Self {
        match value {
            0 => Pixel::Black,
            1 => Pixel::White,
            2 => Pixel::Transparent,
            _ => panic!("unexpected pixel value {}", value),
        }
    }
}

impl Add for Pixel {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (p, Pixel::Transparent) => p,
            (_, p) => p,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Layer {
    pixels: Vec<Pixel>,
}

impl Add for Layer {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Layer {
            pixels: self
                .pixels
                .iter()
                .zip(other.pixels)
                .map(|(&x, y)| x + y)
                .collect(),
        }
    }
}

struct Image {
    layers: Vec<Layer>,
    width: u32,
    height: u32,
}

impl Image {
    fn from_raw(pixels: &Vec<u32>, width: u32, height: u32) -> Self {
        let pixels = &pixels[..];

        let layer_size = (width * height) as usize;
        let layers = pixels
            .chunks(layer_size)
            .map(|layer| Layer {
                pixels: layer.iter().map(|&x| Pixel::from(x)).collect(),
            })
            .collect();

        Image {
            layers: layers,
            width: width,
            height: height,
        }
    }

    fn flatten(self) -> FlatImage {
        // reverse because we want to merge bottom-up
        let mut layers = self.layers.into_iter().rev();
        let base = layers.next().unwrap();
        let flat_layer = layers.fold(base, |acc, x| acc + x);

        FlatImage {
            pixels: flat_layer.pixels,
            width: self.width,
            height: self.height,
        }
    }
}

struct FlatImage {
    pixels: Vec<Pixel>,
    width: u32,
    #[allow(dead_code)]
    height: u32,
}

impl fmt::Display for FlatImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pixels = &self.pixels[..];
        for row in pixels.chunks(self.width as usize) {
            write!(
                f,
                "{}\n",
                row.iter()
                    .map(|x| {
                        match x {
                            Pixel::Black => "█".to_string(),
                            Pixel::White => " ".to_string(),
                            Pixel::Transparent => panic!("got unexpecte transparent pixel"),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            )?
        }
        Ok(())
    }
}

fn checksum(pixels: &Vec<u32>, width: usize, height: usize) -> u32 {
    let pixels = &pixels[..];

    let layer_size = width * height;

    let mut min0 = MAX;
    let mut result = 0;
    let mut counter = HashMap::<u32, u32>::new();

    for layer in pixels.chunks(layer_size) {
        counter.clear();
        for pixel in layer {
            *counter.entry(*pixel).or_insert(0) += 1;
        }
        if *counter.get(&0).unwrap() < min0 {
            min0 = *counter.get(&0).unwrap();
            result = counter.get(&1).unwrap() * counter.get(&2).unwrap();
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day08.txt").unwrap();
    let input: Vec<u32> = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let width = 25;
    let height = 6;

    let part1 = checksum(&input, width, height);
    println!("{}", part1);

    let image = Image::from_raw(&input, width as u32, height as u32);
    let image = image.flatten();
    print!("{}", image);
}

mod test {
    #[test]
    fn test_add_pixels() {
        use super::Pixel;

        assert_eq!(Pixel::Black + Pixel::White, Pixel::White);
        assert_eq!(Pixel::White + Pixel::Black, Pixel::Black);
        assert_eq!(Pixel::Black + Pixel::Transparent, Pixel::Black);
        assert_eq!(Pixel::White + Pixel::Transparent, Pixel::White);
        assert_eq!(Pixel::Transparent + Pixel::Transparent, Pixel::Transparent);
    }

    #[test]
    fn test_add_layer() {
        use super::{Layer, Pixel};

        let lower = Layer {
            pixels: vec![
                Pixel::Black,
                Pixel::Transparent,
                Pixel::Transparent,
                Pixel::Transparent,
            ],
        };
        let upper = Layer {
            pixels: vec![
                Pixel::White,
                Pixel::White,
                Pixel::Transparent,
                Pixel::Transparent,
            ],
        };

        assert_eq!(
            lower + upper,
            Layer {
                pixels: vec![
                    Pixel::White,
                    Pixel::White,
                    Pixel::Transparent,
                    Pixel::Transparent
                ]
            }
        );
    }

    #[test]
    fn test_flatten() {
        use super::{Image, Pixel};

        let image = Image::from_raw(&vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0], 2, 2);
        let image = image.flatten();

        assert_eq!(
            image.pixels,
            vec![Pixel::Black, Pixel::White, Pixel::White, Pixel::Black]
        );
    }
}
