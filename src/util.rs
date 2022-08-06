use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::word_tree::WordTree;

pub fn build_tree_from_file(filename: &str) -> WordTree {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut tree = WordTree::new();

    for line in reader.lines() {
        tree.insert(&line.unwrap());
    }

    return tree;
}