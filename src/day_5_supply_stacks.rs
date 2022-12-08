const MOVE_DATA_START_IDX: usize = 10;

// No desire to parse crate stack diagram into vectors, so hardcoding it is
pub fn get_initial_stack_state() -> Vec<Vec<char>> {
    return vec![
        vec!['W', 'R', 'F'],
        vec!['T', 'H', 'M', 'C', 'D', 'V', 'W', 'P'],
        vec!['P', 'M', 'Z', 'N', 'L'],
        vec!['J', 'C', 'H', 'R'],
        vec!['C', 'P', 'G', 'H', 'Q', 'T', 'B'],
        vec!['G', 'C', 'W', 'L', 'F', 'Z'],
        vec!['W', 'V', 'L', 'Q', 'Z', 'J', 'G', 'C'],
        vec!['P', 'N', 'R', 'R', 'F', 'W', 'T', 'V', 'C'],
        vec!['J', 'W', 'H', 'G', 'R', 'S', 'V']
    ];
}

pub fn top_stack_crates(move_data: &Vec<String>) -> Vec<Vec<char>> {
    let mut stacks = get_initial_stack_state();

    for crate_move in move_data[MOVE_DATA_START_IDX..].iter() {
        let spl_str: Vec<&str> = crate_move.split_whitespace().collect();
        let quantity = spl_str[1].parse::<i32>().unwrap();
        let from_stack = spl_str[3].parse::<usize>().unwrap() - 1;
        let to_stack = spl_str[5].parse::<usize>().unwrap() - 1;

        for _ in 0..quantity {
            let popped_crate = stacks[from_stack].pop().unwrap();
            stacks[to_stack].push(popped_crate);
        }
    }

    // TODO have this return just the last char from each stack as a String
    return stacks;
}

// CRSCZWLVT wrong
pub fn top_stack_crates_multi_move(move_data: &Vec<String>) -> Vec<Vec<char>> {
    let mut stacks = get_initial_stack_state();

    for crate_move in move_data[MOVE_DATA_START_IDX..].iter() {
        let spl_str: Vec<&str> = crate_move.split_whitespace().collect();
        let quantity = spl_str[1].parse::<i32>().unwrap();
        let from_stack = spl_str[3].parse::<usize>().unwrap() - 1;
        let to_stack = spl_str[5].parse::<usize>().unwrap() - 1;

        let mut crates_to_move: Vec<char> = Vec::new();
        println!("{:?}", crates_to_move);
        for _ in 0..quantity {
            crates_to_move.push(stacks[from_stack].pop().unwrap());
        }
        
        println!("{crate_move}");
        println!("{:?}", crates_to_move);
        for _ in 0..crates_to_move.len() {
            let popped_crate = crates_to_move.pop().unwrap();
            stacks[to_stack].push(popped_crate);
        }

        println!("{:?}", crates_to_move);
        println!("BREAK");
        for stack in stacks.iter() {
            println!("{:?}", stack);
        }
    }

    // TODO have this return just the last char from each stack as a String
    return stacks;
}