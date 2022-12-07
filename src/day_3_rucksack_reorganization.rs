pub fn get_prirority_sum(rucksacks: &Vec<String>) -> i32 {
    let mut dup_items = Vec::new();

    for rucksack in rucksacks.iter() {
        let split_pos = rucksack.len() / 2;
        let compartment1: &str = &rucksack[0..split_pos];
        let compartment2: &str = &rucksack[split_pos..];

        let mut found = false;
        for comp1_item in compartment1.chars() {
            if found { break; }

            for comp2_item in compartment2.chars() {
                if comp1_item == comp2_item {
                    dup_items.push(comp1_item);
                    found = true;
                    break;
                }
            }
        }
    }

    let priority_value_sum: i32 = dup_items.iter()
        .map(|item| get_prirority_value(*item))
        .sum();

    priority_value_sum
}

pub fn get_prirority_sum_group_of_3(rucksacks: &Vec<String>) -> i32 {
    let mut dup_items = Vec::new();

    // In steps of 3, iterate through one bag's items and see if it exists in
    // the other two. If it does, we've found the shared item and add it to
    // our vector to be summed using priority values later.
    let step_size = 3;
    for current_step in (0..rucksacks.len()).step_by(step_size) {
        let rucksack_group = &rucksacks[current_step..current_step + step_size];
        
        for item in rucksack_group[0].chars() {
            if !rucksack_group[1].contains(item) { continue; }
            if !rucksack_group[2].contains(item) { continue; }
            dup_items.push(item);
            break;
        }
    }

    let priority_value_sum: i32 = dup_items.iter()
        .map(|item| get_prirority_value(*item))
        .sum();

    priority_value_sum
}

fn get_prirority_value(item: char) -> i32 {
    // Doing some nonsense with ascii values to get priority
    if item as i32 > 90 {
        return item as i32 - 96;
    }   
    return item as i32 - 64 + 26;
}