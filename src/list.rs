use std::fmt::Debug;

#[derive(Debug)]
pub enum List<T: Debug> {
    Nil,
    List {
        head: T,
        tail: Box<List<T>>,
    }
}

impl <T: Debug> List<T> {
    pub fn empty_list() -> List<T> {
        List::Nil
    }
    
    pub fn prepend(self, t: T) -> Self {
        List::List {
            head: t,
            tail: Box::new(self),
        }
    }
    
    pub fn head(&self) -> &T {
        match self {
            &List::Nil => panic!("NoSuchElementException"),
            &List::List {head: ref head, tail: _} => head,
        }
    }
    
    pub fn last(&self) -> &T {
        match self {
            &List::Nil => panic!("NoSuchElementException"),
            &List::List {head: ref head, tail: ref tail} => {
                if tail.is_empty() {
                    head
                } else {
                    tail.last()
                }
            },
        }
    }
    
    fn is_empty(&self) -> bool {
        match self {
            &List::Nil => true,
            &List::List {head: _, tail: _} => false,
        }
    }
}

// TODO
macro_rules! list {
    ($($n: expr),*) => {
        {
            let mut list = List::empty_list();
            $(
                if (list.is_empty()) {
                    list = list.prepend($n);
                } else {
                    list = List::List {
                        head: *list.head(),
                        tail: Box::new(
                            List::List {
                                head: $n,
                                tail: Box::new(List::Nil),
                            }
                        )
                    }
                }
            )*;
            list
        }
    };
}

// fn main() {
//     let list = list!(1, 2, 3, 4, 5);
//     assert_eq!(list.last(), &5);
    
//     let empty_list: List<i32> = List::Nil;
//     assert_eq!(empty_list.is_empty(), true);
    
//     tailrec(&list);
// }

fn tailrec<T: Debug>(list: &List<T>) {
    match list {
        &List::Nil => println!("list is nil"),
        &List::List {head: ref head, tail: ref tail} => {
            println!("head = {:#?}, tail is {}", head, if tail.is_empty() {
                "empty"
            } else {
                "not empty"
            });
            tailrec(tail);
        },
    }
}