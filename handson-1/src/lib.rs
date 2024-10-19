pub struct Node {
    key: i32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: i32) -> Self {
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
    pub fn with_root(key: i32) -> Self {
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
    pub fn add_node(&mut self, parent_id: usize, key: i32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
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
    pub fn sum(&self) -> i32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> i32 {
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
        self.rec_is_bst(Some(0), None, None)
    }

    fn rec_is_bst(&self, node_id: Option<usize>, min: Option<i32>, max: Option<i32>) -> bool {
        if let Some(index) = node_id {
            let node = &self.nodes[index];
            if (max.is_some() && node.key >= max.unwrap())
                || (min.is_some() && node.key < min.unwrap())
            {
                return false;
            } else {
                return self.rec_is_bst(node.id_left, min, Some(node.key))
                    && self.rec_is_bst(node.id_right, Some(node.key), max);
            }
        }
        true
    }

    pub fn maximum_path_sum(&self) -> i32 {
        if self.nodes.len() <= 1 {
            return i32::MIN;
        }
        let mut max = i32::MIN;
        self.rec_maximum_path_sum(Some(0), &mut max);
        max
    }

    fn rec_maximum_path_sum(&self, index: Option<usize>, global_max: &mut i32) -> i32 {
        if let Some(index) = index {
            assert!(index < self.nodes.len(), "Node id is out of range");
            let current_node = &self.nodes[index];

            let left_sum = self.rec_maximum_path_sum(current_node.id_left, global_max);
            let right_sum = self.rec_maximum_path_sum(current_node.id_right, global_max);
            //println!("[{}]: L {} R {}", index, left_sum, right_sum);

            if current_node.id_left.is_some() && current_node.id_right.is_some() {
                let current_max_sum = current_node.key + left_sum + right_sum;
                *global_max = (*global_max).max(current_max_sum);

                return current_node.key + left_sum.max(right_sum);
            }
            if current_node.id_left.is_some() {
                return current_node.key + left_sum;
            } else if current_node.id_right.is_some() {
                return current_node.key + right_sum;
            } else {
                return current_node.key;
            }
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
    fn test_is_bst_3() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.is_bst(), true);

        tree.add_node(0, 10, false); // id 1
        tree.add_node(0, 10, true); // id 2

        assert_eq!(tree.is_bst(), false);
    }

    #[test]
    fn test_maximum_path_sum_1() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.maximum_path_sum(), i32::MIN);

        tree.add_node(0, 5, false); // id 1
        tree.add_node(1, 22, false); // id 2

        assert_eq!(tree.maximum_path_sum(), i32::MIN);

        tree.add_node(2, 7, false); // id 3
        tree.add_node(3, 20, false); // id 4

        assert_eq!(tree.maximum_path_sum(), i32::MIN);
    }

    #[test]
    fn test_maximum_path_sum_2() {
        let mut tree = Tree::with_root(3);
        tree.add_node(0, 4, true);
        tree.add_node(0, 5, false);
        tree.add_node(1, -10, true);
        tree.add_node(1, 4, false);

        assert_eq!(tree.maximum_path_sum(), 16);

        tree.add_node(2, -3, true);
        assert_eq!(tree.maximum_path_sum(), 13);
    }

    #[test]
    fn test_maximum_path_sum_3() {
        let mut tree = Tree::with_root(-15); // 0
        tree.add_node(0, 5, true); // 1
        tree.add_node(0, 6, false); // 2
        tree.add_node(1, -8, true); // 3
        tree.add_node(1, 1, false); // 4
        tree.add_node(3, 2, true); // 5
        tree.add_node(3, -3, false); // 6
        tree.add_node(2, 3, true); // 7
        tree.add_node(2, 9, false); // 8
        tree.add_node(8, 0, false); // 9
        tree.add_node(9, 4, true); // 10
        tree.add_node(9, -1, false); // 11
        tree.add_node(11, 10, true); // 12

        assert_eq!(tree.maximum_path_sum(), 27);
    }

    #[test]
    fn test_maximum_path_sum_4() {
        let mut tree = Tree::with_root(-3);
        tree.add_node(0, -4, true);
        tree.add_node(0, -5, false);
        tree.add_node(1, -10, true);
        tree.add_node(1, -4, false);

        assert_eq!(tree.maximum_path_sum(), -16);
    }
}
