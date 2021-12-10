use std::fs;

fn main() {
    let input = fs::read_to_string("./resources/day6.txt")
        .expect("File not found")
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .fold([0; 9], |mut fish_counts, count| {
            fish_counts[count] += 1;
            fish_counts
        });

    let fishes = (0..256)
        .into_iter()
        .fold(input, |mut input, _| {
            input.rotate_left(1);
            input[6] += input[8];
            input
        })
        .iter()
        .sum::<usize>();

    println!("{}", fishes);
}
