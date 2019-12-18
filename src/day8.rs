use aoc_runner_derive::aoc;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;
const LAYER_SIZE: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

fn count_digits(line: &[u8], digit: char) -> usize {
    line.iter().filter(|&&c| c == digit as u8).count()
}

#[aoc(day8, part1)]
fn validate_image(image: &[u8]) -> usize {
    let line = image.chunks_exact(LAYER_SIZE)
        .min_by_key(|line| count_digits(line, '0')).unwrap();

    count_digits(line, '1' ) * count_digits(line, '2')
}

#[aoc(day8, part2)]
fn print_image(layers: &[u8]) -> &'static str {
    let mut image = ['2' as u8; LAYER_SIZE];

    for layer in layers.chunks_exact(LAYER_SIZE) {
        for i in 0..LAYER_SIZE {
            if image[i] == '2' as u8 {
                image[i] = layer[i];
            }
        }
    }

    for line in image.chunks_exact(IMAGE_WIDTH) {
        let line = line.iter()
            .map(|&pixel| {
                match pixel as char {
                    '1' => 'X',
                    '0' => ' ',
                    e => panic!("Non black or white pixel {}", e)
                }
            })
            .collect::<String>();

        println!("{}", line);
    }
    println!();

    ""
}
