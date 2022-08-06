pub struct Node<T> {
    pub data: T,
    pub children: Vec<Node<T>>
}

impl <T: Clone + PartialEq> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, children: vec![] }
    }

    // from this node, navigate to the node with the given data.
    // make the node to navigate to if necessary
    pub fn navigate_to(&mut self, data: T) -> usize {

        let new_child_data = data.clone();

        return match self.find_child_index(data) {
            None => {
                self.add_child(new_child_data)
            }
            Some(idx) => {
                idx
            }
        }
    }

    // add a child to this node, without checks
    fn add_child(&mut self, data: T) -> usize {
        self.children.push(Node::new(data));
        return self.children.len() - 1;
    }

    // find index of child with certain data
    pub fn find_child_index(&self, data: T) -> Option<usize> {

        for idx in 0..self.children.len() {
            if self.children[idx].data == data {
                return Some(idx);
            }
        }

        return None;
    }
}