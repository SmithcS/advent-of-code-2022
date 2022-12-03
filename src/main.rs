mod io_helper;
mod calorie_counting;
 
fn main() {    
    let input = io_helper::read_input_file_into_vector("day_1.txt");
    let max_calories = calorie_counting::find_max_calories(input);
    println!("{max_calories}");
}
