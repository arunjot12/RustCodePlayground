use std::convert;

pub fn new_list() {
    println!("Hello welcome to the new list");

    let new_list = ListNode::new(2).append(4).append(3);
    let second_list = ListNode::new(5).append(6).append(4);

    let convert_one = ListNode::convert(Some(Box::new(new_list)));
    let convert_second = ListNode::convert(Some(Box::new(second_list)));
    let third = ListNode::add_two_numbers(convert_one, convert_second);
    third.unwrap().print();
}

#[derive(Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    fn convert(l1: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut string_l1 = String::new();
        println!("Existing String {:?}", string_l1);
        println!("Panic Panic Panic Panic");

        let mut new = Vec::new();
        let mut current = l1.as_ref();

        while let Some(node) = current {
            new.push(node.val);
            current = node.next.as_ref();
        }

        let number_str: String = new.iter().map(|d| d.to_string()).collect();
        println!("Collected Number String: {}", number_str);

        let convert_value: i32 = number_str.parse().unwrap();

        Some(Box::new(ListNode {
            val: convert_value,
            next: None,
        }))
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                println!("Val 1: {}, Val 2: {}, Sum: {}", n1.val, n2.val, sum);
                Some(Box::new(ListNode {
                    val: sum,
                    next: Self::add_two_numbers(n1.next, n2.next),
                }))
            }
            _ => None,
        }
    }

    fn remove(self, val: i32) -> Option<Box<ListNode>> {
        println!("self.val {}, val:  {}", self.val, val);
        if self.val == val {
            println!("Skipping");
            // Skip this node and return the rest
            self.next
        } else {
            println!("Continuing with others");
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
