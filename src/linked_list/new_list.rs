pub fn new_list() {
    println!("Hello welcome to the new list");

    let new_list = ListNode::new(1);

    let add_node = new_list.append(2);
     let add_node = add_node.append(5);
    add_node.print(); // Output: 1 2
}

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    fn append(self, val: i32) -> ListNode {
        ListNode {
            val: self.val,
            next: Some(Box::new(match self.next {
                Some(node) => node.append(val),
                None => ListNode::new(val),
            })),
        }
    }

    fn print(self) {
        print!("{}", self.val);
        if let Some(next) = self.next{
              next.print();
        }
    }
}
