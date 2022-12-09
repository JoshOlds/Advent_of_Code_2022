use adventlib;
use rustc_hash::FxHashMap; // Use FxHashMap for faster hashing (Default Rust hashing is secure but slow)

fn main() {
    adventlib::input_helpers::print_puzzle_header(6);
    // Measure both parts because part1 is blazing fast and worth mentioning separately
    adventlib::measure_execution_time_ns(part1, 10000);
    adventlib::measure_execution_time_us(part2, 10000);
    //run(true);
}

fn part1(do_print: bool)
{
    let input = include_str!("../input.txt");
    // Windows lets you iterate over a sliding view of a slice. as_bytes lets you deal with raw byte values which is fast
    let mut windows = input.as_bytes().windows(4);

    let mut index: u32 = 0;
    loop {
        let result = windows.next(); // Grab the next window, check if we are at the end of the input
        let window = match result
        {
            Some(x) => x,
            None => break
        };
        // Check if the values in the window are unique
        let (unique, advance_count) = is_window_unique(window);
        if unique { break; } // If so, stop looping

        // is_window_unique informs us how far to advance the iterator (so we don't repeat unique checks where unnecessary)
        for _ in 0..advance_count
        {
            windows.next();
        }
        // Update the index
        index += advance_count + 1;
    }
    if do_print{println!("Part 1 - Start Sequence found at index: {}", index + 4)}
}

fn part2(do_print: bool)
{
    // repeat setup process, everything is the same except a different is_window_unique implementation
    let input = include_str!("../input.txt");
    let mut windows = input.as_bytes().windows(14); // bigger windows this time
    let mut index = 0;
    loop{
        let result = windows.next();
        let window = match result
        {
            Some(x) => x,
            None => break
        };
        let (unique, advance_count) = is_window_unique_part2(window);
        if unique { break; }
        for _ in 0..advance_count
        {
            windows.next();
        }
        index += advance_count + 1;
    }

    if do_print{println!("Part 2 - Message Sequence found at index: {}", index + 14)}
}

// If statement based approach to solving if the 4 values in the window are unique
// We return a tuple, where the u32 value informs us how far to advance our window
// This is to prevent checking the same failing cases when we know there are repeat values
fn is_window_unique(window: &[u8]) -> (bool, u32)
{
    // Check the latter values first to inform us if we can exit early and advance our window
    if window[2] == window[3]
    {
        return (false, 2);
    }
    if window[1] == window[2] || window[1] == window[3]
    {
        return (false, 1);
    }
    if window[0] == window[1] || window[0] == window[2] || window[0] == window[3]
    {
        return (false, 0);
    }
    return (true, 0);
}

// Window-size-invariant approach to solving for unique values in our window.
// Implemented using a HashTable. Notably, iterates over the window backwards to allow us to
// abort early and advance our window when we see the first repeat value
fn is_window_unique_part2(window: &[u8]) -> (bool, u32)
{
    // FxHashMap is much faster than Rust default Hash Maps
    let mut seen_values: FxHashMap<&u8, u32> = FxHashMap::default();

    // Reverse the iterator, and enumerate to get access to the index (which counts up from zero even though we are walking the iterator in reverse)
    for (idx, value) in window.iter().rev().enumerate() {
        if seen_values.contains_key(value) {
            return (false, (13 - idx) as u32);
        }
        // Otherwise, add the value to the set.
        seen_values.insert(value, 1);
    }
    return (true, 0)
}
