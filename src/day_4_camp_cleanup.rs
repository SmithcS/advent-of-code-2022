pub fn count_pairs(pairs: &Vec<String>) -> (i32, i32) {
    let mut contained_pairs = 0;
    let mut overlapping_pairs = 0;

    for pair in pairs.iter() {
        let ranges: Vec<Vec<i32>> = pair.split(",")
            .map(|range| range.split("-")
                .map(|val| val.parse::<i32>().unwrap())
                .collect())
            .collect();

        if pairs_contains(&ranges[0], &ranges[1]) { contained_pairs += 1; }
        if pairs_overlap(&ranges[0], &ranges[1]) { overlapping_pairs += 1; }
    }

    return (contained_pairs, overlapping_pairs);
}

fn pairs_contains(pair1: &Vec<i32>, pair2: &Vec<i32>) -> bool {
    if pair1[0] == pair2[0] || pair1[1] == pair2[1] { return true; } 
    if pair1[0] > pair2[0] { return pair1[1] < pair2[1] } // e.g. 3-5, 2-6
    pair2[1] < pair1[1] // e.g. 2-6, 4-5
}

fn pairs_overlap(pair1: &Vec<i32>, pair2: &Vec<i32>) -> bool {
    if pair1[0] == pair2[0] || pair1[1] == pair2[1] { return true; }
    if pair1[0] > pair2[0] { return pair1[0] <= pair2[1]; } // e.g. 3-5, 1-2
    pair2[0] <= pair1[1] // e.g. 3-5, 6-7
}