type Link<T> = Option<Box<Node<T>>>;
pub struct List<T> {
    head: Link<T>,
}
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem: elem,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        // Below, you are removing the current head: Link which is
        // self.head. You are matching against that Box<Node>
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

// IntoIter trait
pub struct IntoIter<T>(List<T>); // a tuple struct
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // get the first element in the tuple struct
        self.0.pop()
    }
}

// Iter trait
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>, // Node lives as long as Iter
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // Iter {next: self.head.as_deref().map(|node|&*node)}  // lifetime annotations are required
        Iter {next: self.head.as_deref()} // lifetime annotations for this function is now optional
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T; // Item lives as long as Iterator

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node); // we dont have to use &**node if we used as_deref()
            &node.elem
        })
    }
}

// Iter mut trait
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T>{
    pub fn iter_mut(& mut self) -> IterMut<'_, T> { // TODO: understand what '_ is really doing
        IterMut{ next:self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
// Drop Trait
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


#[cfg(test)]
mod test_first_list {
    use super::*;
    #[test]
    fn linked_list_basic_operations() {
        let mut list = List::new();

        // test push and pop
        list.push(1000);
        list.push(2000);
        list.push(3000);

        //test peek
        assert_eq!(Some(&mut 3000), list.peek_mut());
        list.peek_mut().map(|value| *value = 3333);
        assert_eq!(Some(&3333), list.peek());

        assert_eq!(list.pop(), Some(3333));
        assert_eq!(list.pop(), Some(2000));
        assert_eq!(list.pop(), Some(1000));

        // test empty list pop none
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn linked_list_iterator_operations(){
        let mut list = List::new();
        list.push(10);
        list.push(9);
        list.push(8);
        list.push(7);

        //iter tests
        let mut iter = list.iter();
        assert_eq!(Some(&7), iter.next());
        assert_eq!(Some(&8), iter.next());
        assert_eq!(Some(&9), iter.next());
        assert_eq!(Some(&10), iter.next());

        //iter mut tests
        for i in list.iter_mut(){
            *i = *i + 10
        }

        //into iter test
        let mut iter = list.into_iter();
        assert_eq!(Some(17), iter.next());
        assert_eq!(Some(18), iter.next());
        assert_eq!(Some(19), iter.next());
        assert_eq!(Some(20), iter.next());

    
    }
}
