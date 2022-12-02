use adventlib;

fn main() {
    adventlib::input_helpers::print_puzzle_header(2);
    adventlib::measure_execution_time_us(run, 10000);
}

fn run(do_print: bool)
{
    let input = include_str!("../input.txt");

    let mut total_score : usize = 0;
    for line in input.lines()
    {
        // Rust cant iterate over strings, so you must convert to chars explicitly
        let mut chars = line.chars();
        // the call to nth consumes the iterator value, so the second nth call to position 1 actually skips the space and moves on to the second character
        total_score += evaluate_score(chars.nth(0).unwrap(), chars.nth(1).unwrap());
    }

    if do_print{println!("Part 1 - Total score: {}", total_score)}

    // Part 2
    let mut total_score : usize = 0;
    for line in input.lines()
    {
        // Rust cant iterate over strings, so you must convert to chars explicitly
        let mut chars = line.chars();
        // the call to nth consumes the iterator value, so the second nth call to position 1 actually skips the space and moves on to the second character
        total_score += evaluate_score_part2(chars.nth(0).unwrap(), chars.nth(1).unwrap());
    }

    if do_print{println!("Part 2 - Total score: {}", total_score)}
}

fn evaluate_score(elf_move: char, player_move: char) -> usize
{
    let mut score : usize = 0;
    match player_move
    {
        'X' => {
            score += 1;
            match elf_move{
                'A' => score += 3,
                'B' => score += 0,
                'C' => score += 6,
                _ => panic!("Invalid elf move")
        }},
        'Y' => {
            score += 2;
            match elf_move{
                'A' => score += 6,
                'B' => score += 3,
                'C' => score += 0,
                _ => panic!("Invalid elf move")
            }},
        'Z' => {
            score += 3;
            match elf_move{
                'A' => score += 0,
                'B' => score += 6,
                'C' => score += 3,
                _ => panic!("Invalid elf move")
            }},
        _ => panic!("Invalid player move")
    }
    score
}

fn evaluate_score_part2(elf_move: char, player_move: char) -> usize
{
    let mut score : usize = 0;
    match player_move
    {
        'X' => {
            match elf_move{
                'A' => score += 3,
                'B' => score += 1,
                'C' => score += 2,
                _ => panic!("Invalid elf move")
            }},
        'Y' => {
            match elf_move{
                'A' => score += 4,
                'B' => score += 5,
                'C' => score += 6,
                _ => panic!("Invalid elf move")
            }},
        'Z' => {
            match elf_move{
                'A' => score += 8,
                'B' => score += 9,
                'C' => score += 7,
                _ => panic!("Invalid elf move")
            }},
        _ => panic!("Invalid player move")
    }
    score
}