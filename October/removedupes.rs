
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;

impl Solution {
    pub fn delete_duplicates(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();

        while let Some(node) = current {
            while node.next.as_ref().is_some_and(|next| next.val == node.val) {
                let next_next = node.next.as_mut().unwrap().next.take();
                node.next = next_next;
            }

            current = node.next.as_mut();
        }

        head
    }
}

fn create_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for &value in values {
        *tail = Some(Box::new(ListNode::new(value)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn print_list(head: &Option<Box<ListNode>>) {
    let mut current = head.as_ref();

    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next.as_ref();
    }

    println!();
}

fn main() {
    let input = vec![1, 1, 2, 3, 3];

    println!("Original list: {:?}", input);

    let head = create_list(&input);
    let result = Solution::delete_duplicates(head);

    print!("After removing duplicates: ");
    print_list(&result);
}
