use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

#[derive(Debug, PartialEQ, Eq)]
pub struct TreeNode{
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self{
        TreeNode{
            val,
            left: None,
            right: None
        }
    }
}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node = node.borrow();
                Self::is_mirror(node.left.clone(), node.right.clone())
            }
        }
    }

    fn is_mirror(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,

            (Some(l), Some(r)) => {
                let l = l.borrow();
                let r = r.borrow();

                l.val == r.val
                && Self::is_mirror(l.left.clone(), r.right.clone())
                && Self::is_mirror(l.right.clone(), r.left.clone())
            }

            _ => false,
        }
    }
}

fn main() {

    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(2)));

    left.borrow_mut().left =
    Some(Rc::new(RefCell::new(TreeNode::new(3))));
    left.borrow_mut().right =
    Some(Rc::new(RefCell::new(TreeNode::new(4))));

    right.borrow_mut().left =
    Some(Rc::new(RefCell::new(TreeNode::new(4))));
    right.borrow_mut().right =
    Some(Rc::new(RefCell::new(TreeNode::new(3))));

    root.borrow_mut().left = Some(left);
    root.borrow_mut().right = Some(right);

    let result = Solution::is_symmetric(Some(root));

    println!("Is the tree symmetric? {}", result);
}
