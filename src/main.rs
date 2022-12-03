mod io_helper;
mod calorie_counting;
 
fn main() {    
    let input = io_helper::read_input_file_into_vector("day_1.txt");
    let max_calories = calorie_counting::find_max_calories(&input);
    let top_3_max_calories = calorie_counting::find_top_3_max_calories(&input);
    println!("Max calories held by single elf: {max_calories}");
    println!("Max calories held by top 3 elves: {top_3_max_calories}");
}
