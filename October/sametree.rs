use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,

            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();

                p.val == q.val
                && Self::is_same_tree(p.left.clone(), q.left.clone())
                && Self::is_same_tree(p.right.clone(), q.right.clone())
            }

            _ => false,
        }
    }

}


fn main() {
    let p = Rc::new(RefCell::new(TreeNode::new(1)));
    let p_left = Rc::new(RefCell::new(TreeNode::new(2)));
    let p_right = Rc::new(RefCell::new(TreeNode::new(3)));

    p.borrow_mut().left = Some(Rc::clone(&p_left));
    p.borrow_mut().right = Some(Rc::clone(&p_right));

    // Tree 2: identical to Tree 1
    let q = Rc::new(RefCell::new(TreeNode::new(1)));
    let q_left = Rc::new(RefCell::new(TreeNode::new(2)));
    let q_right = Rc::new(RefCell::new(TreeNode::new(3)));

    q.borrow_mut().left = Some(Rc::clone(&q_left));
    q.borrow_mut().right = Some(Rc::clone(&q_right));

    let result = Solution::is_same_tree(Some(p), Some(q));

    println!("Are the trees identical? {}", result);
}
