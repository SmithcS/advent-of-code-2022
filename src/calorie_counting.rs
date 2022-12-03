pub fn find_max_calories(calories: Vec<Vec<i32>>) -> i32 {
    let mut max_calories: i32 = 0;

    for calorie_vector in calories.iter() {
        let local_max_calories: i32 = calorie_vector.iter().sum();
        if local_max_calories > max_calories { max_calories = local_max_calories; }
    }

    max_calories
}
