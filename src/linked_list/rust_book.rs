use std::rc::Rc;

#[derive(Debug)]
enum LinkedList {
    Data(i32,Rc<LinkedList>),
    Nil,
}

fn main() {
    let a = Rc::new(LinkedList::Data(5, Rc::new(LinkedList::Nil)));
    println!("A is :- {:?}",Rc::strong_count(&a));
    let b = LinkedList::Data(5,Rc::new(LinkedList::Data(10, Rc::new(LinkedList::Nil))));
    
    let c = LinkedList::Data(1,Rc::clone(&a));
    println!("Count is :- {:?}",Rc::strong_count(&a));

    {
      let d = LinkedList::Data(1,Rc::clone(&a));
     println!("Count is :- {:?}",Rc::strong_count(&a));

                //  println!("C is :- {:?}",Rc::strong_count(&c));
    }
     println!("Count is :- {:?}",Rc::strong_count(&a));
    
}