mod elements;
mod node;

use crate::word_tree::elements::WordElement;
use crate::word_tree::node::Node as Leaf;

pub struct WordTree {
    root: Leaf<WordElement>
}

impl WordTree {
    pub fn new() -> Self {
        Self { root: Leaf::new(WordElement::BeginWord) }
    }

    // add word to tree
    pub fn insert(&mut self, new_word: &str) {
        let mut curr_node = &mut self.root;
        for letter in new_word.chars() {
            let new_curr_node_idx = curr_node.navigate_to(WordElement::Letter(letter));
            curr_node = &mut curr_node.children[new_curr_node_idx];
        }

        curr_node.navigate_to(WordElement::EndWord);
    }

    // check if a word is present in the tree
    pub fn search(&self, word: &str) -> bool {
        let mut curr_node = &self.root;

        // traverse the tree letter by letter
        for letter in word.chars() {
            // check if the current node has the target letter
            match curr_node.find_child_index(WordElement::Letter(letter)) {
                None => {
                    // if the letter is not a child of the current node, it is not in the tree
                    return false;
                }
                Some(idx) => {
                    // letter is a child of the current node, so go to that node
                    curr_node = &curr_node.children[idx];
                }
            }
        }

        /* At this point, curr_node has made it to the last letter of the word. If curr_node
         * has a child that marks the end of the word, then the word actually exists in the
         * tree. This is important because otherwise, the search would match any input that had
         * the same starting letters as another word in the tree. For example, without this
         * check, a tree with only the word "teatime" in it would also match the word "teati".
         */
        return match curr_node.find_child_index(WordElement::EndWord) {
            None => {false}
            Some(_) => {true}
        }
    }

    // suggest WordElements to follow an input string which could possibly produce a valid word
    pub fn suggest(&self, base: &str) -> Option<Vec<WordElement>> {
        let mut suggestions = Vec::<WordElement>::new();

        let mut curr_node = &self.root;

        // traverse to the target node
        for letter in base.chars() {
            match curr_node.find_child_index(WordElement::Letter(letter)) {
                None => {
                    // this base string isn't valid!
                    return None;
                }
                Some(idx) => {
                    curr_node = &curr_node.children[idx];
                }
            }
        }

        // at this point, curr_node's children have all valid suggestions
        for child_node in &curr_node.children {
            suggestions.push(child_node.data.clone());
        }

        return Some(suggestions);
    }
}