use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, not_line_ending},
    combinator::{iterator, map, value},
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug, Clone)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
struct DirectoryNode {
    index: Option<usize>,
    entries: HashMap<String, EntryNode>,
    parent: Option<usize>,
}

impl DirectoryNode {
    fn new() -> Self {
        Self {
            index: None,
            entries: HashMap::new(),
            parent: None,
        }
    }
    fn add_to_storage(
        storage: &mut Vec<DirectoryNode>,
        mut directory: Self,
        parent: Option<usize>,
    ) -> usize {
        let index = storage.len();
        directory.index = Some(index);
        directory.parent = parent;
        storage.push(directory);
        index
    }
    fn directory_sizes(&self, storage: &Vec<DirectoryNode>) -> (usize, Vec<usize>) {
        let mut own_size: usize = 0;
        let mut sub_directory_sizes: Vec<usize> = Vec::new();
        for entry in self.entries.values() {
            match entry {
                EntryNode::DirectoryNode(dir) => {
                    let (size, sub_sizes) = storage[*dir].directory_sizes(storage);
                    own_size += size;
                    sub_directory_sizes.extend(sub_sizes);
                }
                EntryNode::FileNode(file) => own_size += file.size,
            }
        }
        sub_directory_sizes.push(own_size);
        (own_size, sub_directory_sizes)
    }
}

#[derive(Debug)]
struct FileNode {
    name: String,
    size: usize,
}

#[derive(Debug)]
enum EntryNode {
    DirectoryNode(usize),
    FileNode(FileNode),
}

struct FileInfo {
    name: String,
    size: usize,
}

enum EntryInfo {
    DirectoryInfo(String),
    FileInfo(FileInfo),
}

fn number(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse::<usize>().unwrap())(input)
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    value(Command::Ls, terminated(tag("ls"), line_ending))(input)
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("cd ")(input)?;
    let (input, path) = terminated(alt((not_line_ending, tag("/"))), line_ending)(input)?;
    Ok((input, Command::Cd(path.to_string())))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    preceded(tag("$ "), alt((parse_ls, parse_cd)))(input)
}

fn parse_directory(input: &str) -> IResult<&str, EntryInfo> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = terminated(not_line_ending, line_ending)(input)?;
    Ok((input, EntryInfo::DirectoryInfo(name.to_string())))
}

fn parse_file(input: &str) -> IResult<&str, EntryInfo> {
    let (input, size) = terminated(number, tag(" "))(input)?;
    let (input, name) = terminated(not_line_ending, line_ending)(input)?;
    Ok((
        input,
        EntryInfo::FileInfo(FileInfo {
            name: name.to_string(),
            size,
        }),
    ))
}

fn parse_entry(input: &str) -> IResult<&str, EntryInfo> {
    alt((parse_directory, parse_file))(input)
}

fn do_the_thing(input: &str) -> usize {
    // Redefine so we can mutate it locally
    let mut input = input;
    // This Vec is the storage arena, so we can have references to parents
    let mut directories = Vec::<DirectoryNode>::new();
    let root = DirectoryNode::new();
    let root = DirectoryNode::add_to_storage(&mut directories, root, None);
    let mut current_dir = root;
    while !input.is_empty() {
        let (rest, command) = parse_command(input).unwrap();
        input = rest;
        match command {
            Command::Cd(path) => match path.as_str() {
                "/" => current_dir = root,
                ".." => current_dir = directories[current_dir].parent.unwrap(),
                _ => {
                    let dir = directories[current_dir].entries.get(&path).unwrap();
                    match dir {
                        EntryNode::DirectoryNode(dir) => current_dir = *dir,
                        _ => panic!("Not a directory"),
                    }
                }
            },
            Command::Ls => {
                let mut entries = iterator(input, parse_entry);
                entries.for_each(|entry_info| match entry_info {
                    EntryInfo::DirectoryInfo(name) => {
                        let dir = DirectoryNode::new();
                        let dir =
                            DirectoryNode::add_to_storage(&mut directories, dir, Some(current_dir));
                        directories[current_dir]
                            .entries
                            .insert(name, EntryNode::DirectoryNode(dir));
                    }
                    EntryInfo::FileInfo(file) => {
                        let file = FileNode {
                            name: file.name,
                            size: file.size,
                        };
                        directories[current_dir]
                            .entries
                            .insert(file.name.clone(), EntryNode::FileNode(file));
                    }
                });
                let (rest, _) = entries.finish().unwrap();
                input = rest;
            }
        }
    }
    let (_, sub_sizes) = directories[root].directory_sizes(&directories);
    sub_sizes.into_iter().filter(|size| *size <= 100_000).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("$ cd /
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
" => 95437)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}
