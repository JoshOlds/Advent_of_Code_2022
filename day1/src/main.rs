use adventlib;

fn main() {
    adventlib::input_helpers::print_puzzle_header(1);
    adventlib::measure_execution_time_us(run, 10000);
}

fn run(do_print: bool)
{
    let input = include_str!("../input.txt");

    // Vector of Vectors of calorie values (initialize vectors bigger than needed to speed up execution time)
    let mut elf_vector: Vec<Vec<usize>> = Vec::with_capacity(1000);

    let mut calorie_vector: Vec<usize> = Vec::with_capacity(100);
    for calorie_val in input.lines()
    {
        if calorie_val.is_empty() // If we are at a blank line, the previous elf is complete
        {
            elf_vector.push(calorie_vector.clone());
            calorie_vector.clear();
        } else {
            calorie_vector.push(calorie_val.parse::<usize>().unwrap()); // Convert to usize and accumulate in a calorie vector
        }
    }

    // Create a new vector of sums for the elf values
    let mut elf_sum_vector: Vec<usize> = elf_vector.iter()
        .map(|cal_vec| cal_vec.iter().sum())
        .collect();

    // Find the highest calorie value elf
    let max_ref = elf_sum_vector.iter()
        .max()
        .unwrap();

    // Get index of max value - used for deleting later
    let index = elf_sum_vector.iter().position(|element| element == max_ref).unwrap();

    if do_print // Only print on the final run when measuring execution time
    {
        println!("Part 1: The maximum calorie count elf is Elf #{}, and he is carrying {} calories", index, max_ref);
    }

    // Part 2
    // Sort the vector, reverse it, take the first three items, and sum them
    elf_sum_vector.sort_unstable();
    let max: usize = elf_sum_vector.into_iter().rev().take(3).sum();

    if do_print
    {
        println!("Part 2: The three elves with the most food are carrying {} calories.", max);
    }




}