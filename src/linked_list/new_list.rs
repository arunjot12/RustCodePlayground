pub fn new_list() {
    println!("Hello welcome to the new list");

    let new_list = ListNode::new(1).append(2).append(3);
    let remove_list = new_list.remove(2).expect("Unable to remove");
    remove_list.print();

}

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

   fn remove(self, val: i32) -> Option<Box<ListNode>> {
    if self.val == val {
        // Skip this node and return the rest
        self.next
    } else {
        Some(Box::new(ListNode {
            val: self.val,
            next: self.next.map(|node| node.remove(val))?,
        }))
    }
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
        print!("{} ", self.val);
        if let Some(next) = self.next {
            next.print();
        }
    }
}
