use std::cmp::max;

const TARGET_CYCLES: [usize; 7] = [0, 20, 60, 100, 140, 180, 220];

enum Instruction {
    NOOP { cycles: u8 },
    ADDX { cycles: u8, value: i16 }
}

// Let's try to do this without actually simulating it. 
// we will build the schedule, where schedule[n] is the registervalue at that time
pub fn calc_signal_strength_sum(signal_instructions: &Vec<String>) -> i16 {
    let mut add_schedule = vec![0 as i16; signal_instructions.len() * 2];
    let mut last_instruction_idx: usize = 0;

    for (cycle, instruction_string) in signal_instructions.iter().enumerate()  {
        let instruction = parse_instruction_string(instruction_string);

        match instruction {
            Instruction::NOOP { cycles }=> {
                last_instruction_idx = max(
                    cycle,
                    last_instruction_idx + (cycles as usize)
                );
             },
            Instruction::ADDX { cycles, value }=> {
                let effective_at_idx = max(
                    cycle + (cycles as usize),
                    last_instruction_idx + (cycles as usize)
                );
                add_schedule[effective_at_idx] = value;
                last_instruction_idx = effective_at_idx;
            }
        }
    }

    let mut signal_strength_sum = 0;
    for target_cycle in TARGET_CYCLES[0..].iter() {
        let register_value = 1 + add_schedule[0..*target_cycle]
            .iter()
            .sum::<i16>();
        let signal_strength = register_value * (*target_cycle as i16);

        signal_strength_sum += signal_strength;
    }

    let crt_row_len = 6;
    let crt_col_len = 40;
    let mut crt_screen_output = vec![vec![""; crt_col_len]; crt_row_len];

    let mut current_cycle = 0;
    for row in 0..crt_row_len {
        for col in 0..crt_col_len {
            let register_value = 1 + add_schedule[0..(current_cycle + 1)]
                .iter()
                .sum::<i16>();
            if ((register_value - 1)..(register_value + 2)).contains(&(col as i16)) {
                crt_screen_output[row][col] = "#";
            } else {
                crt_screen_output[row][col] = ".";
            }
            current_cycle += 1;
        }
    }

    for row in crt_screen_output.iter() {
        println!("{:?}", row.join(""));
    }

    return signal_strength_sum;
}

fn parse_instruction_string(instruction_string: &String) -> Instruction {
    let spl_string: Vec<&str> = instruction_string.split_whitespace().collect();
    
    if spl_string[0] == "noop" { 
        return Instruction::NOOP { cycles: (1) };
    }

    return Instruction::ADDX { 
        cycles: (2), 
        value: (spl_string[1].parse::<i16>().unwrap()) 
    };
}
