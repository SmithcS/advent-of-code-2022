mod io_helper;
mod day_1_calorie_counting;
mod day_2_rock_paper_scissors;
mod day_3_rucksack_reorganization;
mod day_4_camp_cleanup;
mod day_5_supply_stacks;
mod day_6_tuning_trouble;
mod day_7_no_space_left_on_device;
mod day_8_treetop_tree_house;
 
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

    // Day 5
    let move_data = io_helper::read_input_file_into_vector_string("day_5.txt");
    let arranged_crates = day_5_supply_stacks::top_stack_crates(&move_data);
    let arranged_crates_multi_move = day_5_supply_stacks::top_stack_crates_multi_move(&move_data);
    println!("{:?} {:?}", arranged_crates, arranged_crates_multi_move);

    // Day 6 
    let datastream_buffer = io_helper::read_input_file_as_string("day_6.txt");
    let start_of_packet = day_6_tuning_trouble::find_start_marker(&datastream_buffer, day_6_tuning_trouble::PACKET_WINDOW_SIZE);
    let start_of_message = day_6_tuning_trouble::find_start_marker(&datastream_buffer, day_6_tuning_trouble::MESSAGE_WINDOW_SIZE);
    println!("start of packer marker idx: {}", start_of_packet);
    println!("start of message marker idx: {}", start_of_message);

    // Day 7
    let cmd_history = io_helper::read_input_file_into_vector_string("day_7.txt");
    let output = day_7_no_space_left_on_device::calc_total_size(&cmd_history);
    println!("{}", output);

    // Day 8
    let tree_heights = io_helper::read_input_file_as_vector_int("day_8.txt");
    let visible_trees = day_8_treetop_tree_house::find_visible_trees(&tree_heights);
    let max_scenic_score = day_8_treetop_tree_house::find_max_scenic_score(&tree_heights);
    println!("visible trees: {}, max scenic score: {}", visible_trees, max_scenic_score);
}
