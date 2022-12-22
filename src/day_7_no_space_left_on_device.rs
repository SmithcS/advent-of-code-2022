use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    name: String,
    parent: String,
    children: Vec<String>,
    size: u64
}

#[derive(Debug, PartialEq, Eq)]
enum CmdType {
    ChangeDir,
    ListFiles,
    AddDir,
    AddFile,
}

const MAX_SIZE: u64 = 100_000;
const TOTAL_DISK_SPACE: u64 = 70_000_000;
const MIN_UNUSED_SPACE: u64 = 30_000_000;

pub fn calc_total_size(cmd_history: &Vec<String>) -> u64 {
    let mut filetree = HashMap::new();
    let mut dir_names = Vec::new();
    
    let root_node = Node {
        name: "/".to_string(),
        parent: "".to_string(),
        children: vec![],
        size: 0
    };

    filetree.insert("/".to_string(), root_node);
    let mut curr_node_key = "/".to_string();

    for cmd in cmd_history[1..].iter() {
        let parsed_cmd: Vec<&str> = cmd.split_whitespace().collect();
        let cmd_type = get_cmd_type(&parsed_cmd);

        match &cmd_type {
            &CmdType::ChangeDir => {
                let dst_dir = parsed_cmd[2];
                if dst_dir == ".." {
                    curr_node_key = filetree.get(&curr_node_key).unwrap().parent.clone();
                } else {
                    curr_node_key = get_full_name(&curr_node_key, &dst_dir.to_string());
                }
            },
            &CmdType::AddDir => {
                let dirname = parsed_cmd[1].to_string();
                let expanded_dirname = get_full_name(&curr_node_key, &dirname);

                // Insert new node into the tree, update parent to include node
                // as child, and make note of the name of the newly added dir
                filetree.insert(
                    expanded_dirname.clone(),
                    Node {
                        name: expanded_dirname.clone(),
                        parent: filetree.get(&curr_node_key).unwrap().name.clone(),
                        children: vec![],
                        size: 0
                    }
                );
                filetree.get_mut(&curr_node_key).unwrap().children.push(expanded_dirname.clone());
                dir_names.push(expanded_dirname);
            },
            &CmdType::AddFile => {
                let filename = parsed_cmd[1].to_string();
                let expanded_filename = get_full_name(&curr_node_key, &filename);
                let filesize = parsed_cmd[0].parse::<u64>().unwrap();
                let parent_name = filetree.get(&curr_node_key).unwrap().name.clone();

                // Insert new node into the tree, update parent to include node
                // as child, and update sizes of all dirs above this node to 
                // account for it's size 
                filetree.insert(
                    expanded_filename.clone(),
                    Node {
                        name: expanded_filename.clone(),
                        parent: parent_name,
                        children: Vec::new(),
                        size: filesize
                    }
                );
                filetree.get_mut(&curr_node_key).unwrap().children.push(expanded_filename.clone());
                update_sizes(&mut filetree, filesize, expanded_filename);
            },
            &CmdType::ListFiles => {
                continue;
            }
        }
    }

    let size_sum_below_max: u64 = dir_names.iter()
        .map(|dir_name| filetree
            .get(dir_name)
            .unwrap()
            .size)
        .filter(|size| size <= &MAX_SIZE)
        .sum();

    let size_of_file_to_delete = get_size_of_file_to_delete(&filetree, &dir_names);
    println!("size of file to delete: {}", size_of_file_to_delete);

    return size_sum_below_max;
}

fn update_sizes(filetree: &mut HashMap<String, Node>, size: u64, start_node_key: String) {
    let mut curr_node_key = start_node_key;
    while curr_node_key != "" {
        filetree.get_mut(&curr_node_key).unwrap().size += size;
        curr_node_key = filetree.get(&curr_node_key).unwrap().parent.clone();
    }
}

fn get_cmd_type(parsed_cmd: &Vec<&str>) -> CmdType {    
    if parsed_cmd[0] == "$" {
        if parsed_cmd[1] == "cd" { return CmdType::ChangeDir; }
        return CmdType::ListFiles;
    }

    if parsed_cmd[0] == "dir" { return CmdType::AddDir; }
    return CmdType::AddFile
}

fn get_full_name(curr_path: &String, new_item: &String) -> String {
    if curr_path == "/" { return format!("{curr_path}{new_item}"); }
    return format!("{curr_path}/{new_item}")
}

fn get_size_of_file_to_delete(filetree: &HashMap<String, Node>, dir_names: &Vec<String>) -> u64 {
    let used_space = filetree.get("/").unwrap().size;
    let available_space = TOTAL_DISK_SPACE - used_space;
    let space_to_delete = MIN_UNUSED_SPACE - available_space;

    let dir_sizes = dir_names.iter()
        .map(|dir_name| filetree
            .get(dir_name)
            .unwrap()
            .size)
        .collect::<Vec<u64>>();

    // Start with maximum possible disk space, and reduce down while remaining
    // above the required space to delete
    let mut size_of_dir_to_delete = &TOTAL_DISK_SPACE;
    for size in dir_sizes.iter() {
        if size >= &space_to_delete && size < size_of_dir_to_delete {
            size_of_dir_to_delete = size;
        }
    }

    return *size_of_dir_to_delete;
}
