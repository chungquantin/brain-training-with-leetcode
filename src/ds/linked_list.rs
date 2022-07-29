pub trait Integer32 {}
impl Integer32 for i32 {}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ListNode<T>
where
    T: Integer32,
{
    pub next: Option<Box<ListNode<T>>>,
    pub val: T,
}

impl ListNode<i32> {
    pub fn insert(&mut self, node: i32) {
        let mut cur_node = self;
        loop {
            if cur_node.next.is_none() {
                cur_node.next = Some(Box::new(ListNode::<i32> {
                    next: None,
                    val: node,
                }));
                break;
            }
            cur_node = cur_node.next.as_deref_mut().unwrap();
        }
    }

    pub fn insert_head(&mut self, node: i32) {
        self.val = node
    }

    pub fn init(&mut self, nodes: Vec<i32>) {
        self.insert_head(*nodes.get(0).unwrap());
        let slice_nodes = &nodes[1..nodes.len()];
        for node in slice_nodes {
            self.insert(*node)
        }
    }

    pub fn info(&mut self) {
        let mut cur_node = self;
        loop {
            if cur_node.next.is_none() {
                break;
            }
            cur_node = cur_node.next.as_mut().unwrap()
        }
    }
}

#[test]
fn test() {
    let linked_list = &mut ListNode::<i32>::default();
    linked_list.init(vec![1, 2, 3, 4, 5]);
    linked_list.insert(6);
    linked_list.insert(7);
    linked_list.info();
}
