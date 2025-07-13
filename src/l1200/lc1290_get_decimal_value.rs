#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
    let mut ret = 0;
    while let Some(ref node) = head {
        ret = (ret << 1) | node.val;
        head = node.next.clone();
    }
    ret
}
