# Day 8: Space Image Format

- Difficulty: ⭐

## Description

In this problem, we need to decode an image that is encoded in a format that is similar to a bitmap image. The image is composed of layers, and each layer is composed of pixels. The image is encoded in a single line, and the pixels are represented by the numbers 0, 1 and 2. Each layer is composed of 25x6 pixels, and each number represents different colors, 0: Black, 1: White, 2: transparent.

## Problem solution

Part 1 was really easy to solve, we just need to count the number of 0s, 1s and 2s in each layer and then multiply the number of 1s and 2s in the layer that has the least number of 0s. I decided to do it a bit more functional, but this can also be done with various for loops easily:

```rust
fn part1(input: &str) -> u64 {
    let width = 25;
    let height = 6;

    let layers: Vec<(u64, u64, u64)> = input
        .as_bytes()
        .chunks(width * height)
        .map(|layer| {
            layer
                .iter()
                .fold((0, 0, 0), |(zeros, ones, twos), &b| match b {
                    b'0' => (zeros + 1, ones, twos),
                    b'1' => (zeros, ones + 1, twos),
                    b'2' => (zeros, ones, twos + 1),
                    _ => unreachable!(),
                })
        })
        .collect();

    layers
        .iter()
        .min_by_key(|(zeros, _, _)| *zeros)
        .map(|(_, ones, twos)| ones * twos)
        .unwrap()
}
```

Part 2 was also really easy, the idea is to decode the image taking into account that the layers are stacked on top of each other, so we need to take the first non-transparent pixel in each position and print the image. I decided to store the values in a vector of bytes, and then print the image taking into account the size of the image:

```rust
fn part2(input: &str) {
    let width = 25;
    let height = 6;

    let layers: Vec<Vec<u8>> = input
        .as_bytes()
        .chunks(width * height)
        .map(|layer| layer.to_vec())
        .collect();

    let mut image = vec![b'2'; width * height];

    for layer in layers {
        for (i, &b) in layer.iter().enumerate() {
            if image[i] == b'2' {
                image[i] = b;
            }
        }
    }

    for row in image.chunks(width) {
        for &b in row {
            print!("{}", if b == b'1' { '█' } else { ' ' });
        }
        println!();
    }
}
```

This could be improved probably reading it as a byte array directly, and building the layers just one time but it takes less than 0.1ms to run, so I think I'm fine with that.

## Conclusion

I think this problem was really easy. There is not much to think about it, and the solution is straightforward. I think I took a bit more time to write it because I was trying hard to do it in a functional way, but still, it didn't take me long to solve it. It's always good to see problems whose solution is ASCII art :).
