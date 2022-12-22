use std::collections::HashSet;

/* 
Scan the entire vector in each direction; left -> right, right -> left, top -> bottom,
and bottom -> top to find trees that are 'visible'. We use a Set to keep track of
visible tree to avoid double-counting, e.g. if a tree is seen left -> right but
also in top -> bottom.
*/
pub fn find_visible_trees(tree_heights: &Vec<Vec<i32>>) -> usize {
    let mut seen_visible_trees = HashSet::new();
    
    let row_len = tree_heights[0].len();
    let col_len = tree_heights.len();

    for (row_idx, row) in tree_heights.iter().enumerate() {
        // To save doing multiple loops, we look from right -> left and left -> right
        // in the same loop by utilizing indexes rather than .iter()
        let mut lr_row_max_height = -1;
        let mut rl_row_max_height = -1;

        let mut lr_col_idx = 0;
        let mut rl_col_idx = row_len;
        for _ in 0..row_len {
            rl_col_idx -= 1; // To avoid underflow on final iteration, we must subtract first

            if row[lr_col_idx] > lr_row_max_height {
                seen_visible_trees.insert((row_idx, lr_col_idx));
                lr_row_max_height = row[lr_col_idx];
            }

            if row[rl_col_idx] > rl_row_max_height {
                seen_visible_trees.insert((row_idx, rl_col_idx));
                rl_row_max_height = row[rl_col_idx];
            }

            lr_col_idx += 1;
        }
    }


    for col_idx in 0..col_len {
        // To save doing multiple loops, we look from top -> bottom and bottom -> rop
        // in the same loop by utilizing indexes rather than .iter()
        let mut tb_col_max_height = -1;
        let mut bt_col_max_height = -1;
        
        let mut tb_row_idx = 0;
        let mut bt_row_idx = col_len;
        for _ in 0..row_len {
            bt_row_idx -= 1; // To avoid underflow on final iteration, we must subtract first

            if tree_heights[tb_row_idx][col_idx] > tb_col_max_height {
                seen_visible_trees.insert((tb_row_idx, col_idx));
                tb_col_max_height = tree_heights[tb_row_idx][col_idx];
            }

            if tree_heights[bt_row_idx][col_idx] > bt_col_max_height {
                seen_visible_trees.insert((bt_row_idx, col_idx));
                bt_col_max_height = tree_heights[bt_row_idx][col_idx];
            }

            tb_row_idx += 1;
        }
    }

    return seen_visible_trees.len();
}

pub fn find_max_scenic_score(tree_heights: &Vec<Vec<i32>>) -> usize {
    let mut max_scenic_score = 0;

    let row_len = tree_heights[0].len();
    let col_len = tree_heights.len();

    for row_idx in 0..row_len {
        for col_idx in 0..col_len {
            let scenic_score = calc_scenic_score(row_idx, col_idx, tree_heights);
            if scenic_score > max_scenic_score { max_scenic_score = scenic_score; }
        }
    }

    return max_scenic_score;
}


fn calc_scenic_score(x_pos: usize, y_pos: usize, tree_heights: &Vec<Vec<i32>>) -> usize {
    let mut scenic_score = 1; // Total scenic score
    let mut directional_scenic_score = 0; // Score in one direction, e.g. up, down, left, right
    
    let candidate_tree_size = tree_heights[x_pos][y_pos];
    let tree_x_len = tree_heights[0].len();
    let tree_y_len = tree_heights.len();
    
    let mut neigh_x_pos = x_pos;
    while neigh_x_pos > 0 {
        neigh_x_pos -= 1;
        let neigh_tree_size = tree_heights[neigh_x_pos][y_pos];
        if neigh_tree_size < candidate_tree_size { 
            directional_scenic_score += 1;
        } else if neigh_tree_size == candidate_tree_size {
            directional_scenic_score += 1;
            break;
        } else {
            break;
        }
    }
    scenic_score *= directional_scenic_score;
    directional_scenic_score = 0;

    neigh_x_pos = x_pos;
    while neigh_x_pos < tree_x_len - 1 {
        neigh_x_pos += 1;
        let neigh_tree_size = tree_heights[neigh_x_pos][y_pos];
        if neigh_tree_size < candidate_tree_size { 
            directional_scenic_score += 1;
        } else if neigh_tree_size == candidate_tree_size {
            directional_scenic_score += 1;
            break;
        } else {
            break;
        }
    }
    scenic_score *= directional_scenic_score;
    directional_scenic_score = 0;

    let mut neigh_y_pos = y_pos;
    while neigh_y_pos > 0 {
        neigh_y_pos -= 1;
        let neigh_tree_size = tree_heights[x_pos][neigh_y_pos];
        if neigh_tree_size < candidate_tree_size { 
            directional_scenic_score += 1;
        } else if neigh_tree_size == candidate_tree_size {
            directional_scenic_score += 1;
            break;
        } else {
            break;
        }
    }
    scenic_score *= directional_scenic_score;
    directional_scenic_score = 0;

    neigh_y_pos = y_pos;
    while neigh_y_pos < tree_y_len - 1 {
        neigh_y_pos += 1;
        let neigh_tree_size = tree_heights[x_pos][neigh_y_pos];
        if neigh_tree_size < candidate_tree_size { 
            directional_scenic_score += 1;
        } else if neigh_tree_size == candidate_tree_size {
            directional_scenic_score += 1;
            break;
        } else {
            break;
        }
    }
    scenic_score *= directional_scenic_score;

    return scenic_score;
}