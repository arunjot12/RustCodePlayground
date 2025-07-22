use std::convert;

pub fn new_list() {
    println!("Hello welcome to the new list");

    let new_list = ListNode::new(2).append(4).append(3);
    // let second_list = ListNode::new(5).append(6).append(4);

    // let third = ListNode::add_two_numbers(Some(Box::new(new_list)), Some(Box::new(second_list)));
    // third.unwrap().print();

    let convert_one = ListNode::convert(Some(Box::new(new_list)));
    convert_one.unwrap().print();
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

        let a :Option<Box<ListNode>>=  match l1 {
            (Some(ref n1)) => {
                string_l1.push_str(&n1.val.to_string());
                let value: i32 = string_l1.parse().unwrap();

                println!("Value is {}",value);

                let convert_l1 = Some(Box::new(ListNode {
                    val: value,
                    next: Self::convert(n1.next.clone()),
                }));
                return convert_l1;
            }
            _ => None,
        };

      let b =   Some(Box::new(ListNode{
            val:0,
            next: None
        }));

        println!("String after insertion is {:?}", string_l1);
        b
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
