use std::mem;

pub struct List {
    head: Link,
}

impl List{
    pub fn new() -> Self {
        List{ head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32){
        let new_node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Below, you are removing the current head: Link which is
        // self.head. You are matching against that Box<Node>
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            // After matching against the Box<Node>
            // We want to set the self.head to the next node
            // since we are popping from the list.
            // Then, we return the current node's value.
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

enum Link {
    Empty,
    More(Box<Node>),
}


struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod test_first_list {
    use super::*;
    #[test]
    fn linked_list_basic_operations(){
        let mut list = List::new();

        // test push and pop
        list.push(1000);
        list.push(2000);
        list.push(3000);

        assert_eq!(list.pop(), Some(3000));
        assert_eq!(list.pop(), Some(2000));
        assert_eq!(list.pop(), Some(1000));

        // test empty list pop none
        assert_eq!(list.pop(), None);



    }
}