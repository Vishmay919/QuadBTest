// Defining the structure for a binary tree node
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    // Calculating the maximum depth of the binary tree
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Using a match statement to handle the recursive nature of the problem
        match root {
            Some(node) => {
                // Recursively finding the depth of the left and right subtrees
                let left_depth = TreeNode::max_depth(node.borrow().left.clone());
                let right_depth = TreeNode::max_depth(node.borrow().right.clone());
                // Returning the greater depth plus one for the current level
                1 + std::cmp::max(left_depth, right_depth)
            },
            None => 0, // Returning 0 for a non-existent node (base case)
        }
    }
}
