mod word_tree;
mod util;

use crate::word_tree::WordTree;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use crate::util::build_tree_from_file;
    use crate::WordTree;

    #[test]
    fn insert_test() {
        // declare dictionary and adversaries
        let dictionary = [
            "r",
            "rust",
            "java",
            "javascript",
            "jquery",
            "typescript",
            "c",
            "c++",
            "go",
            "python",
            "perl",
        ];
        let adversaries = [
            "google",
            "pearl",
            "jav",
            "see"
        ];

        // build the tree
        let mut tree = WordTree::new();
        for word in dictionary {
            tree.insert(word);
        }

        // make sure all the words are in the dictionary
        for word in dictionary {
            assert!(
                tree.search(word),
                "The dictionary word \"{}\" was not matched by the search!",
                word
            );
        }

        // make sure none of the adversary words are in the dictionary
        for word in adversaries {
            assert!(
                !tree.search(word),
                "The adversarial word \"{}\" was incorrectly matched by the search!",
                word
            );
        }
    }

    #[test]
    fn build_tree_from_file_test() {
        let filename = "C:\\Users\\wille\\IdeaProjects\\wordsearch\\word_lists\\mostcommon1000.txt";

        test_with_wordlist(filename);
    }

    #[test]
    fn very_large_wordlist_test() {
        let filename = "C:\\Users\\wille\\IdeaProjects\\wordsearch\\word_lists\\verylarge.txt";

        test_with_wordlist(filename);
    }

    fn test_with_wordlist(filename: &str) {
        let mut tree = build_tree_from_file(filename);

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            assert!(tree.search(&line.unwrap()));
        }
    }
}
