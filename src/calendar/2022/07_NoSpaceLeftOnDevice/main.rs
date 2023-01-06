mod data;
mod errors;
mod parser;

use data::DirectoryRef;
use errors::Error;
use parser::LogParser;
use std::{ fs::File, io::{BufRead, BufReader} };

fn parse_file_system_log(file_path: &str) -> Result<DirectoryRef, Error> {
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    LogParser::default()?.parse_log_lines(reader.lines())
}

struct SizeTreeNode {
    total_size: usize,
    children: Vec<SizeTreeNode>
}

impl SizeTreeNode {
    fn depth_first<'a>(&'a self) -> DepthFirstIterator<'a> {
        DepthFirstIterator { traverse_stack: vec![self], current: None }
    }
}

impl From<&DirectoryRef> for SizeTreeNode {
    fn from(directory: &DirectoryRef) -> Self {
        let mut node = SizeTreeNode { total_size: 0, children: Vec::<_>::new() };
        
        node.total_size = directory.borrow().files.values().map(|file| file.borrow().size).sum();

        for directory_child in directory.borrow().directories.values() {
            let child_node = SizeTreeNode::from(directory_child);
            node.total_size += child_node.total_size;
            node.children.push(child_node);
        }

        node
    }
}

struct DepthFirstIterator<'a> {
    traverse_stack: Vec<&'a SizeTreeNode>,
    current: Option<&'a SizeTreeNode>
}

impl<'a> Iterator for DepthFirstIterator<'a> {
    type Item = &'a SizeTreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            for child in &current.children {
                self.traverse_stack.push(child)
            }
        }

        self.current = self.traverse_stack.pop();
        self.current
    }
}

fn sum_all_dir_sizes_at_most(root: &SizeTreeNode, max_size: usize) -> usize {
    root.depth_first()
        .filter(|directory| directory.total_size <= max_size)
        .map(|directory| directory.total_size)
        .sum()
}

fn find_size_of_directory_to_free(root: &SizeTreeNode, total_space: usize, needed_space: usize) -> Option<usize> {
    let unused_space = total_space - root.total_size;
    let space_to_free = if needed_space > unused_space { needed_space - unused_space } else { 0 };
    root.depth_first()
        .filter(|directory| directory.total_size >= space_to_free)
        .min_by_key(|directory| directory.total_size)
        .map(|directory| directory.total_size)
}

fn main() {
    match parse_file_system_log("inputs/2022/07/NoSpaceLeftOnDevice.txt") {
        Ok(root) => {
            let size_tree = SizeTreeNode::from(&root);
            let solution1 = sum_all_dir_sizes_at_most(&size_tree, 100000);
            let solution2 = find_size_of_directory_to_free(&size_tree, 70000000, 30000000).unwrap();
            println!("Solution 1 : {solution1}");
            println!("Solution 2 : {solution2}");
        }
        Err(err) => println!("{err:?}")
    }
}