use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode{
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode{
    #[inline]
    pub fn new(val: i32) -> Self{
        TreeNode{
            val,
            left: None,
            right: None
        }
    }
}

struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn inorder(
            node: Option<Rc<RefCell<TreeNode>>>,
            result: &mut Vec<i32>,
           ){
               if let Some(node) = node {
                   let node = node.borrow();
                   inorder(node.left.clone(), result);
                   result.push(node.val);
                   inorder(node.right.clone(), result);
            }
        }

        let mut result = Vec::new();
        inorder(root, &mut result);
        result
    }
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(3)));
    let left_right = Rc::new(RefCell::new(TreeNode::new(4)));

    root.borrow_mut().left = Some(Rc::clone(&left));
    root.borrow_mut().right = Some(Rc::clone(&right));
    left.borrow_mut().right = Some(Rc::clone(&left_right));

    let result = Solution::inorder_traversal(Some(root));

    println!("Inorder traversal: {:?}", result);
}
