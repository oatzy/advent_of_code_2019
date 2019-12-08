use std::collections::HashMap;
use std::fs;
use std::u32::MAX;

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
}
