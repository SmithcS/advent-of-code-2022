mod io_helper;
mod calorie_counting;
mod rock_paper_scissors;
mod day_3_rucksack_reorganization;
 
fn main() {
    // Day 1
    let input = io_helper::read_input_file_into_vector("day_1.txt");
    let max_calories = calorie_counting::find_max_calories(&input);
    let top_3_max_calories = calorie_counting::find_top_3_max_calories(&input);
    println!("Max calories held by single elf: {max_calories}");
    println!("Max calories held by top 3 elves: {top_3_max_calories}");

    // Day 2
    let game_strategy = io_helper::read_input_file_into_vector_string("day_2.txt");
    let total_score = rock_paper_scissors::calculate_game_score(&game_strategy, false);
    let total_score_new_strat = rock_paper_scissors::calculate_game_score(&game_strategy, true);
    println!("Total score based on game_strategy: {total_score}");
    println!("Total score based on game_strategy alt: {total_score_new_strat}");

    // Day 3
    let rucksack_contents = io_helper::read_input_file_into_vector_string("day_3.txt");
    let priority_value_sum = day_3_rucksack_reorganization::get_prirority_sum(&rucksack_contents);
    let priority_value_sum_group_of_3 = day_3_rucksack_reorganization::get_prirority_sum_group_of_3(&rucksack_contents);
    println!("{priority_value_sum_group_of_3:?}");
}
