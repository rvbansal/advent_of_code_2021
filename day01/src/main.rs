use itertools::Itertools;

fn count_increases(input: &str) -> usize {
    input
        .split('\n')
        .map(|depth| depth.parse::<u32>().unwrap())
        .tuple_windows::<(u32, u32)>()
        .filter(|(a, b)| b > a)
        .count()
}

fn count_increases_window(input: &str) -> usize {
    input
        .split('\n')
        .map(|depth| depth.parse::<u32>().unwrap())
        .tuple_windows::<(u32, u32, u32)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows::<(u32, u32)>()
        .filter(|(a, b)| b > a)
        .count()
}

fn main() {
    println!(
        "Depth increases: {}",
        count_increases(include_str!("input.txt"))
    );
    println!(
        "Depth increases (window = 3): {}",
        count_increases_window(include_str!("input.txt"))
    )
}
