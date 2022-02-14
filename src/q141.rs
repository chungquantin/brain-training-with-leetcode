pub struct Solution {}

pub trait Integer32 {}
impl Integer32 for i32 {}

#[derive(PartialEq, Eq, Debug, Default)]
pub struct ListNode<T>
where
    T: Integer32,
{
    next: Option<Box<ListNode<T>>>,
    val: T,
}

impl ListNode<i32> {
    pub fn push(&mut self, node: i32) {
        let mut cur_node = self;
        loop {
            if cur_node.next.is_none() {
                cur_node.next = Some(Box::new(ListNode::<i32> {
                    next: Some(Box::new(ListNode::<i32>::default())),
                    val: node,
                }));
                break;
            }
            cur_node = cur_node.next.as_deref_mut().unwrap();
        }
    }

    pub fn init(nodes: Vec<i32>) -> Self {
        let mut list_node = ListNode::<i32>::default();
        for node in nodes {
            list_node.push(node)
        }
        return list_node;
    }
}

impl Solution {
    pub fn has_cycle(head: &ListNode<i32>) -> bool {
        let mut runner: &ListNode<i32> = head;
        let mut walker: &ListNode<i32> = head;
        loop {
            if runner.next.is_none() {
                break;
            }
            let next_runner: &ListNode<i32> = &runner.next.as_ref().unwrap();
            if next_runner.next.is_none() {
                break;
            }
            runner = next_runner.next.as_ref().unwrap();
            walker = walker.next.as_ref().unwrap();
            if runner == walker {
                return true;
            }
        }
        return false;
    }
}
