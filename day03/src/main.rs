fn count_digits(binary_numbers: &Vec<&str>, pos: usize) -> (usize, usize) {
    let digits = binary_numbers
        .iter()
        .map(|line| line[pos..pos + 1].to_string())
        .collect::<Vec<_>>();

    (
        digits.iter().filter(|&i| i == "0").count(),
        digits.iter().filter(|&i| i == "1").count(),
    )
}

fn get_power_consumption(input: &str) -> u32 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let len = lines[0].len();

    let digit_counts = (0..len)
        .map(|i| count_digits(&lines, i))
        .collect::<Vec<_>>();

    let epsilon = u32::from_str_radix(
        digit_counts
            .iter()
            .map(|(count0, count1)| if count1 > count0 { "1" } else { "0" })
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();

    let gamma = u32::from_str_radix(
        digit_counts
            .iter()
            .map(|(count0, count1)| if count0 > count1 { "1" } else { "0" })
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();

    epsilon * gamma
}

fn get_life_support_rating(input: &str) -> u32 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let len = lines[0].len();

    let mut o2 = lines.clone();
    for i in 0..len {
        let (count0, count1) = count_digits(&o2.clone(), i);
        let selected = if count0 > count1 { "0" } else { "1" };
        o2 = o2
            .iter()
            .filter(|&j| j[i..i + 1] == *selected)
            .map(|k| k.clone())
            .collect::<Vec<&str>>();
        if o2.len() == 1 {
            break;
        }
    }

    let o2_rating = u32::from_str_radix(o2[0], 2).unwrap();

    let mut co2 = lines.clone();
    for i in 0..len {
        let (count0, count1) = count_digits(&co2.clone(), i);
        let selected = if count0 > count1 { "1" } else { "0" };
        co2 = co2
            .iter()
            .filter(|&j| j[i..i + 1] == *selected)
            .map(|k| k.clone())
            .collect::<Vec<&str>>();
        if co2.len() == 1 {
            break;
        }
    }

    let co2_rating = u32::from_str_radix(co2[0], 2).unwrap();
    o2_rating * co2_rating
}

fn main() {
    println!(
        "Power capacity: {}",
        get_power_consumption(include_str!("input.txt"))
    );
    println!(
        "Life support rating: {}",
        get_life_support_rating(include_str!("input.txt"))
    );
}
