pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left == None,
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right == None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    pub fn is_bst(&self) -> bool {
        self.rec_is_bst(Some(0))
    }

    fn rec_is_bst(&self, node_id: Option<usize>) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            if let Some(left) = node.id_left {
                // check whether value in this node > value in its left child
                if node.key <= (&self.nodes[left]).key {
                    return false;
                }
            }
            if let Some(right) = node.id_right {
                // check whether value in this node <= value in its right child
                if node.key > (&self.nodes[right]).key {
                    return false;
                }
            }

            let is_bst_left = self.rec_is_bst(node.id_left);
            let is_bst_right = self.rec_is_bst(node.id_right);

            return is_bst_left && is_bst_right;
        }

        true
    }

    pub fn maximum_path_sum(&self) -> u32 {
        let mut m = u32::MIN;
        let r = self.rec_maximum_path_sum(Some(0), &mut m);
        std::cmp::max(r, m)
    }

    fn rec_maximum_path_sum(&self, index: Option<usize>, global_max: &mut u32) -> u32 {
        if let Some(index) = index {
            assert!(index < self.nodes.len(), "Parent node id does not exist");
            let current_node = &self.nodes[index];

            let left_sum = self.rec_maximum_path_sum(current_node.id_left, global_max);
            let right_sum = self.rec_maximum_path_sum(current_node.id_right, global_max);

            let current_max_sum = current_node.key + left_sum + right_sum;
            *global_max = (*global_max).max(current_max_sum);

            return current_node.key + left_sum.max(right_sum);
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
    }

    #[test]
    fn test_is_bst_1() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.is_bst(), true);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.is_bst(), true);

        tree.add_node(2, 20, false); // id 2
        assert_eq!(tree.is_bst(), false);
    }

    #[test]
    fn test_is_bst_2() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.is_bst(), true);

        tree.add_node(0, 5, false); // id 1
        tree.add_node(0, 22, true); // id 2

        assert_eq!(tree.is_bst(), false);

        tree.add_node(2, 20, false); // id 2
        assert_eq!(tree.is_bst(), false);
    }

    #[test]
    fn test_maximum_path_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.maximum_path_sum(), 10);

        tree.add_node(0, 5, false); // id 1
        tree.add_node(1, 22, false); // id 2

        assert_eq!(tree.maximum_path_sum(), 37);

        tree.add_node(2, 7, false); // id 3
        tree.add_node(3, 20, false); // id 4

        assert_eq!(tree.maximum_path_sum(), 64);
    }
}
