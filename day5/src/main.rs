use adventlib;

fn main() {
    adventlib::input_helpers::print_puzzle_header(5);
    adventlib::measure_execution_time_us(run, 10000);
    //run(true);
}

fn run(do_print: bool)
{
    let input = include_str!("../input.txt");

    // Represent the data as 9 stacks
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(9);
    let mut stack1: Vec<char> = Vec::with_capacity(72); // 9 stacks of maximum 8 starting values, so a stack should never be deeper than 72
    let mut stack2: Vec<char> = Vec::with_capacity(72);
    let mut stack3: Vec<char> = Vec::with_capacity(72);
    let mut stack4: Vec<char> = Vec::with_capacity(72);
    let mut stack5: Vec<char> = Vec::with_capacity(72);
    let mut stack6: Vec<char> = Vec::with_capacity(72);
    let mut stack7: Vec<char> = Vec::with_capacity(72);
    let mut stack8: Vec<char> = Vec::with_capacity(72);
    let mut stack9: Vec<char> = Vec::with_capacity(72);

    let mut lines = input.lines();

    // Read the lines that make up the initial stack puzzle input
    // We will push the lines into a vector to allow us to read it backwards when parsing and creating our numeric stacks
    let mut temp_stack: Vec<&str> = Vec::with_capacity(8); // 8 Initial lines
    for _ in  1..=8
    {
        temp_stack.push(lines.next().unwrap());
    }

    // Iterate over the initial stack values, backwards (by popping the vector)
    // Push values into our stacks.
    // I really dislike this wall of match statements, but couldn't come up with a less verbose alternative when dealing with flat stack variables
    for _ in 0..temp_stack.len()
    {
        let mut chars = temp_stack.pop().unwrap().chars();
        match chars.nth(1)
        {
            Some(x) => {if x.is_alphabetic() {stack1.push(x)}}
            None => {}
        }
        match chars.nth(3)
        {
            Some(x) => {if x.is_alphabetic() {stack2.push(x)}}
            None => {}
        }
        match chars.nth(3)
        {
            Some(x) => {if x.is_alphabetic() {stack3.push(x)}}
            None => {}
        }
        match chars.nth(3)
        {
            Some(x) => {if x.is_alphabetic() {stack4.push(x)}}
            None => {}
        }
        match chars.nth(3)
        {
            Some(x) => {if x.is_alphabetic() {stack5.push(x)}}
            None => {}
        }
        match chars.nth(3)
        {
            Some(x) => {if x.is_alphabetic() {stack6.push(x)}}
            None => {}
        }
        match chars.nth(3)
        {
            Some(x) => {if x.is_alphabetic() {stack7.push(x)}}
            None => {}
        }
        match chars.nth(3)
        {
            Some(x) => {if x.is_alphabetic() {stack8.push(x)}}
            None => {}
        }
        match chars.nth(3)
        {
            Some(x) => {if x.is_alphabetic() {stack9.push(x)}}
            None => {}
        }
    }
    // Push all the stacks to the stacks vector
    stacks.push(stack1);
    stacks.push(stack2);
    stacks.push(stack3);
    stacks.push(stack4);
    stacks.push(stack5);
    stacks.push(stack6);
    stacks.push(stack7);
    stacks.push(stack8);
    stacks.push(stack9);

    // Deep copy the stacks for use in part 2
    let mut stacks2: Vec<Vec<char>> = stacks.clone();

    // Advance the lines pointer to the start of the instructions
    lines.next();
    lines.next();

    // Begin parsing and executing commands
    loop{
        let line_opt = lines.next(); // Break when at the end of file
        let line = match line_opt
        {
            Some(x) => x,
            None => break
        };
        // Split on whitespace, then skip the text to grab the numbers
        let mut words = line.split_whitespace();
        let move_amount = words.nth(1).unwrap().parse::<usize>().unwrap();
        let source_stack_id = words.nth(1).unwrap().parse::<usize>().unwrap();
        let dest_stack_id = words.nth(1).unwrap().parse::<usize>().unwrap();

        // Execute -- part 1
        for _ in 0..move_amount
        {
            let source_stack = stacks.iter_mut().nth(source_stack_id - 1).unwrap();
            let val = source_stack.pop().unwrap();
            let dest_stack = stacks.iter_mut().nth(dest_stack_id - 1).unwrap();
            dest_stack.push(val);
        }

        // Execute -- part 2
        let source_stack = stacks2.iter_mut().nth(source_stack_id - 1).unwrap();
        let start_idx = source_stack.len() - move_amount;
        let vals: Vec<char> = source_stack.drain(start_idx..).collect();
        let dest_stack = stacks2.iter_mut().nth(dest_stack_id - 1).unwrap();
        for val in vals
        {
            dest_stack.push(val);
        }
    }

    // Part 1 Result
    let mut top_chars = String::new();
    for stack in stacks.iter()
    {
        top_chars += &*stack.last().unwrap().to_string();
    }
    let mut top_chars2 = String::new();
    for stack in stacks2.iter()
    {
        top_chars2 += &*stack.last().unwrap().to_string();
    }
    if do_print {println!("Part 1 - The top values are: {}", top_chars);}
    if do_print {println!("Part 2 - The top values are: {}", top_chars2);}
}