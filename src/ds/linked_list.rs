
pub trait Integer32 {}
impl Integer32 for i32 {}

#[derive(PartialEq, Eq, Debug, Default)]
pub struct ListNode<T>
where
    T: Integer32,
{
    pub next: Option<Box<ListNode<T>>>,
    pub val: T,
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