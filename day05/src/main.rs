use nom::{
    bytes::complete::tag, character::complete::i32, character::complete::newline,
    multi::separated_list1, IResult,
};
use std::collections::HashMap;

type Point = (i32, i32);
type Line = (Point, Point);

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, x) = i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = i32(input)?;
    Ok((input, (x, y)))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, a) = parse_point(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, b) = parse_point(input)?;
    Ok((input, (a, b)))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_input(input: &str) -> Vec<Line> {
    let (extra_input, lines) = parse_lines(input).unwrap();
    assert_eq!(extra_input, "");
    lines
}

fn add_line_to_grid(line: &Line, grid: &mut HashMap<Point, usize>) {
    let ((x1, y1), (x2, y2)) = *line;
    let rise = y2 - y1;
    let run = x2 - x1;
    let (x_sign, y_sign) = (run.signum(), rise.signum());
    let mut x = x1;
    let mut y = y1;

    for _ in 0..=i32::max(rise.abs(), run.abs()) {
        grid.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
        x += x_sign;
        y += y_sign;
    }
}

fn count_overlaps(input: &str) -> usize {
    let lines = parse_input(input);
    let mut grid = HashMap::<Point, usize>::new();
    for line in lines {
        let ((x1, y1), (x2, y2)) = line;
        if x1 == x2 || y1 == y2 {
            add_line_to_grid(&line, &mut grid);
        }
    }
    grid.values().filter(|&v| *v > 1).count()
}

fn count_overlaps_diag(input: &str) -> usize {
    let lines = parse_input(input);
    let mut grid = HashMap::<Point, usize>::new();
    for line in lines {
        add_line_to_grid(&line, &mut grid);
    }
    grid.values().filter(|&v| *v > 1).count()
}

fn main() {
    println!(
        "Count overlaps: {}",
        count_overlaps(include_str!("input.txt"))
    );
    println!(
        "Count overlaps diag: {}",
        count_overlaps_diag(include_str!("input.txt"))
    );
}
