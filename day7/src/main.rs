use std::collections::HashMap;

struct FileSystem {
    file_system: HashMap<String, Option<u64>>,
    current_path: Vec<String>,
    total_file_system_size: u64,
}

impl FileSystem {
    /// Apply a command to the file system (e.g. cd, ls)
    fn apply(&mut self, command: &str) {
        let cmd_split = command.split(" ").collect::<Vec<&str>>();

        match cmd_split[..] {
            // Jump to root dir (pop directories on path until root, to avoid angering the borrow checker)
            ["$", "cd", "/"] => {
                while self.current_path.len() > 0 {
                    self.current_path.pop();
                }
            }
            // Jump current path to parent
            ["$", "cd", ".."] => {
                self.current_path.pop();
            }
            // Step down from current directory into a child directory (create if it does not exist)
            ["$", "cd", dir] => {
                // If dir exists in file system off current path, update current path
                // Otherwise add that amended path to the file system
                self.current_path.push(dir.to_string());
                if !self.file_system.contains_key(&self.current_path.join("/")) {
                    self.file_system.insert(self.current_path.join("/"), None);
                }
            }
            // Ignore: We count the files 'listed' on subsequent lines
            ["$", "ls"] => {}
            // Ignore: Directories have no intrinsic size other than the files they contain
            ["dir", _] => {}
            // Update the file system with the file size listed
            [file_size, _file_name] => {
                // Add file_size to current path (dir) (and all parent dir sizes!)
                let mut tmp_path = self.current_path.clone();
                let file_size_int = file_size.parse::<u64>().ok().unwrap();

                // Update total file system size (avoid double counting files)
                self.total_file_system_size += file_size_int;

                // For the current directory, and all parent directories, update directory file size
                while tmp_path.len() > 0 {
                    let dir_size = self.file_system.get(&tmp_path.join("/")).unwrap();
                    if dir_size.is_some() {
                        let updated_dir_size = dir_size.unwrap() + file_size_int;
                        self.file_system
                            .insert(tmp_path.join("/"), Some(updated_dir_size));
                    } else {
                        self.file_system
                            .insert(tmp_path.join("/"), Some(file_size_int));
                    }
                    tmp_path.pop();
                }
            }
            _ => {}
        }
    }
}

/// Use HashMap representation for file structure
/// Until I understand rust Box/heap allocation & Rc pointers
fn main() {
    // Create file system
    let mut fs = FileSystem {
        file_system: HashMap::new(),
        current_path: Vec::new(),
        total_file_system_size: 0,
    };

    // Parse commands
    let commands = include_str!("day7.txt").lines().collect::<Vec<&str>>();

    // Apply each command in turn to the file system
    for command in commands {
        fs.apply(command);
    }

    // Pretty-print filesystem
    // println!("{:#?}", fs.file_system);

    // Part 1: Sum of directories below 100_000 in size
    let file_size_sum: u64 = fs
        .file_system
        .values()
        .into_iter()
        .filter(|x| x.is_some())
        .filter(|x| x.unwrap() <= 100_000)
        .map(|x| x.unwrap())
        .sum();

    println!("Part 1: {:#?}", file_size_sum);

    // Part 2: Find size of smallest directory we can delete to free up 30_000_000 space
    let additional_space_required = 30_000_000 - (70_000_000 - fs.total_file_system_size);
    let file_size_of_dir_to_delete: u64 = fs
        .file_system
        .values()
        .into_iter()
        .filter(|x| x.is_some())
        .filter(|x| x.unwrap() >= additional_space_required)
        .map(|x| x.unwrap())
        .min()
        .unwrap();

    println!("Part 2: {:#?}", file_size_of_dir_to_delete);
}
