use std::collections::HashSet;

struct RopeMove {
    direction: Direction,
    magnitude: i32
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    // Given a direction, return a new Point representing where this Point will
    // have moved. 
    fn get_updated_pos(&self, direction: &Direction) -> Point {
        let updated_pos = match direction {
            Direction::Up    => { Point { x: self.x, y: self.y + 1 } },
            Direction::Down  => { Point { x: self.x, y: self.y - 1 } },
            Direction::Left  => { Point { x: self.x - 1, y: self.y} },
            Direction::Right => { Point { x: self.x + 1, y: self.y} }
        };
        return updated_pos;
    }

    // Given a Point, return a Point that represents where this Point must move
    // to 'follow' the other. 
    fn follow_point(&self, point: &Point) -> Point {
        let delta_x = point.x - &self.x; 
        let delta_y = point.y - &self.y;

        let mut delta_x_self = 0;
        let mut delta_y_self = 0;
        if delta_x.abs() > 1 { 
            delta_x_self = delta_x.signum();
            if delta_y.abs() > 0 { delta_y_self = delta_y.signum(); }
        }
        if delta_y.abs() > 1 { 
            delta_y_self = delta_y.signum(); 
            if delta_x.abs() > 0 { delta_x_self = delta_x.signum(); }
        }

        return Point { x: &self.x + delta_x_self, y: &self.y + delta_y_self };
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

/* 
Given a vector of rope movements that describes how the 'head' of a rope with
num_rope_links links moves, calculates the number unqiue points that the last link
or the 'tail' visits.
*/
pub fn calc_unique_tail_positions(rope_movements: &Vec<String>, num_rope_links: usize) -> usize {   
    let mut rope_links = vec![Point { x: 0, y: 0 }; num_rope_links];
    let mut seen_tail_positions = HashSet::new();
    let mut curr_head_position = Point { x: 0, y: 0 };

    for movement in rope_movements.iter() {
        let rope_move = parse_move_from_movement_str(movement);
        
        for _ in 0..rope_move.magnitude {
            rope_links[0] = rope_links[0].get_updated_pos(&rope_move.direction);
            curr_head_position = rope_links[0].clone();

            for link_idx in 1..rope_links.len() {
                rope_links[link_idx] = rope_links[link_idx].follow_point(&curr_head_position);
                curr_head_position = rope_links[link_idx].clone();
            }

            seen_tail_positions.insert(rope_links.last().unwrap().clone());
        }
    }

    return seen_tail_positions.len();
}

fn parse_move_from_movement_str(movement_str: &String) -> RopeMove {
    let spl_movement: Vec<&str> = movement_str.split_whitespace().collect();

    let direction = match spl_movement[0] {
        "U" => { Direction::Up },
        "D" => { Direction::Down },
        "L" => { Direction::Left },
        "R" => { Direction::Right }
        _   => { panic!("Unexpected direction") }
    };

    return RopeMove {
        direction: direction,
        magnitude: spl_movement[1].parse::<i32>().unwrap()
    };
}