use std::collections::HashSet;
use std::collections::VecDeque;

pub const PACKET_WINDOW_SIZE: usize = 4;
pub const MESSAGE_WINDOW_SIZE: usize = 14;

pub fn find_start_marker(datastream: &String, distinct_chars: usize) -> usize {
    // Initialize our char_window and char_set with the first PACKET_WINDOW_SIZE characters
    let mut char_window = VecDeque::new();
    let mut char_set = HashSet::new();
    for stream_char in datastream[0..distinct_chars].chars() {
        char_window.push_back(stream_char);
        char_set.insert(stream_char);
    }

    for (idx, stream_char) in datastream[distinct_chars..].chars().enumerate() {
        // If our set has reached distinct_chars in length, we known the required number of unique
        // chars have been found and the start marker recieved. We add distinct_chars to the current
        // index to account for the chars processed at initialization.
        if char_set.len() == distinct_chars { return idx + distinct_chars; }

        char_set.remove(&char_window.pop_front().unwrap());
        char_window.push_back(stream_char);

        for window_char in char_window.iter() {
            // This .contains is likely useless; .insert works regardless. I thought it may be more
            // performant but didn't test, but my guess is it's not
            if !char_set.contains(window_char) { char_set.insert(*window_char); }
        }
    }

    return 0;
}
