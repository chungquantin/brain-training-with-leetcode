#[path = "./ds/linked_list.rs"] mod linked_list;

pub use linked_list::ListNode;

pub struct Solution {}

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
