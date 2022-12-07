use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!(
        "{:?}",
        get_total_size_of_some_directories(std::io::stdin().lock())
    );
}

fn get_total_size_of_some_directories(input: impl BufRead) -> (u64, u64) {
    let mut file_system = parse_file_system(input);
    update_directory_size(0, &mut file_system);
    (
        got_total_size_of_directories_with_at_most(100000, &file_system.directories),
        get_smallest_directory_size_with_at_least(
            30000000 - (70000000 - file_system.directories[0].size),
            &file_system.directories,
        ),
    )
}

fn got_total_size_of_directories_with_at_most(size: u64, directories: &[Directory]) -> u64 {
    directories
        .iter()
        .filter(|v| v.size <= size)
        .map(|v| v.size)
        .sum()
}

fn get_smallest_directory_size_with_at_least(size: u64, directories: &[Directory]) -> u64 {
    directories
        .iter()
        .filter(|v| v.size >= size)
        .min_by_key(|v| v.size)
        .unwrap()
        .size
}

fn parse_file_system(input: impl BufRead) -> FileSystem {
    let mut file_system = FileSystem {
        directories: vec![Directory::default()],
        ..Default::default()
    };
    let mut current_directory = usize::MAX;
    let mut last_command = None;
    for line in input.lines().map(|v| v.unwrap()) {
        if let Some(value) = line.strip_prefix("$ ") {
            let command = parse_command(value);
            if let Command::ChangeDirectory(v) = &command {
                match v.as_str() {
                    "/" => current_directory = 0,
                    ".." => {
                        current_directory =
                            file_system.directories[current_directory].parent.unwrap()
                    }
                    name => {
                        current_directory =
                            file_system.directories[current_directory].directories[name]
                    }
                }
            }
            last_command = Some(command);
        } else if matches!(last_command, Some(Command::ListDirectory)) {
            match parse_ls_output(&line) {
                Output::Directory(mut v) => {
                    let directory_index = file_system.directories.len();
                    file_system.directories[current_directory]
                        .directories
                        .insert(v.name.clone(), directory_index);
                    v.parent = Some(current_directory);
                    file_system.directories.push(v);
                }
                Output::File(v) => {
                    let file_index = file_system.files.len();
                    let directory = &mut file_system.directories[current_directory];
                    directory.files.insert(v.name.clone(), file_index);
                    file_system.files.push(v);
                }
            }
        }
    }
    file_system
}

fn update_directory_size(directory_index: usize, file_system: &mut FileSystem) {
    let mut directories = vec![directory_index];
    let mut to_visit = Vec::new();
    while let Some(directory_index) = directories.pop() {
        to_visit.push(directory_index);
        directories.extend(
            file_system.directories[directory_index]
                .directories
                .values(),
        );
    }
    to_visit.reverse();
    for directory_index in to_visit {
        let directory = &file_system.directories[directory_index];
        let sub_directories_size = directory
            .directories
            .values()
            .map(|v| file_system.directories[*v].size)
            .sum::<u64>();
        let files_size = directory
            .files
            .values()
            .map(|v| file_system.files[*v].size)
            .sum::<u64>();
        file_system.directories[directory_index].size = sub_directories_size + files_size;
    }
}

fn parse_command(value: &str) -> Command {
    if let Some(suffix) = value.strip_prefix("cd ") {
        return Command::ChangeDirectory(suffix.to_string());
    }
    if value == "ls" {
        return Command::ListDirectory;
    }
    unreachable!();
}

fn parse_ls_output(value: &str) -> Output {
    if let Some(suffix) = value.strip_prefix("dir ") {
        return Output::Directory(Directory {
            name: suffix.to_string(),
            ..Default::default()
        });
    }
    let (size, name) = value.split_once(' ').unwrap();
    Output::File(File {
        name: name.to_string(),
        size: u64::from_str(size).unwrap(),
    })
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(String),
    ListDirectory,
}

#[derive(Debug)]
enum Output {
    File(File),
    Directory(Directory),
}

#[derive(Default, Debug)]
struct FileSystem {
    directories: Vec<Directory>,
    files: Vec<File>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

#[derive(Default, Debug)]
struct Directory {
    name: String,
    size: u64,
    parent: Option<DirectoryIndex>,
    files: HashMap<String, FileIndex>,
    directories: HashMap<String, DirectoryIndex>,
}

type DirectoryIndex = usize;
type FileIndex = usize;

#[test]
fn example_test() {
    let buffer = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#
    .as_bytes();
    assert_eq!(
        get_total_size_of_some_directories(buffer),
        (95437, 24933642)
    );
}
