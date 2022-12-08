mod io_helper;
mod day_1_calorie_counting;
mod day_2_rock_paper_scissors;
mod day_3_rucksack_reorganization;
mod day_4_camp_cleanup;
mod day_5_supply_stacks;
 
fn main() {
    // Day 1
    let input = io_helper::read_input_file_into_vector("day_1.txt");
    let max_calories = day_1_calorie_counting::find_max_calories(&input);
    let top_3_max_calories = day_1_calorie_counting::find_top_3_max_calories(&input);
    println!("Max calories held by single elf: {max_calories}");
    println!("Max calories held by top 3 elves: {top_3_max_calories}");

    // Day 2
    let game_strategy = io_helper::read_input_file_into_vector_string("day_2.txt");
    let total_score = day_2_rock_paper_scissors::calculate_game_score(&game_strategy, false);
    let total_score_new_strat = day_2_rock_paper_scissors::calculate_game_score(&game_strategy, true);
    println!("Total score based on game_strategy: {total_score}");
    println!("Total score based on game_strategy alt: {total_score_new_strat}");

    // Day 3
    let rucksack_contents = io_helper::read_input_file_into_vector_string("day_3.txt");
    let priority_value_sum = day_3_rucksack_reorganization::get_prirority_sum(&rucksack_contents);
    let priority_value_sum_group_of_3 = day_3_rucksack_reorganization::get_prirority_sum_group_of_3(&rucksack_contents);
    println!("{priority_value_sum_group_of_3:?}");

    // Day 4
    let pairs = io_helper::read_input_file_into_vector_string("day_4.txt");
    let num_pairs = day_4_camp_cleanup::count_pairs(&pairs);
    println!("{:?} {:?}", num_pairs.0, num_pairs.1);

    let move_data = io_helper::read_input_file_into_vector_string("day_5.txt");
    let arranged_crates = day_5_supply_stacks::top_stack_crates(&move_data);
    let arranged_crates_multi_move = day_5_supply_stacks::top_stack_crates_multi_move(&move_data);
    //println!("{:?}", arranged_crates);
    //println!("{:?}", arranged_crates_multi_move);
    println!();
    for stack in arranged_crates_multi_move.iter() {
        println!("{:?}", stack);
    }
}
