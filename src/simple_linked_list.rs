struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(element: T, next: Option<Box<Node<T>>>) -> Self {
        Node {
            data: element,
            next,
        }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        let node = Box::new(Node::new(element, self.head.take()));
        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.len {
            0 => None,
            _ => {
                self.len -= 1;
                self.head.take().map(|node| {
                    let node = *node;
                    self.head = node.next;
                    node.data
                })
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        let mut next = self.head.as_ref().map(|node| &**node);
        while let Some(node) = next {
            rev_list.push(node.data.clone());
            next = node.next.as_ref().map(|node| &**node);
        }
        rev_list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self { //vec to sim
        let mut list = SimpleLinkedList::new();
        for i in item {
            list.push(i.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(data) = self.pop() {
            vec.push(data);
        }
        vec.reverse();
        vec
    }
}



#[test]
fn test_new_list_is_empty() {
    let list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert_eq!(list.len(), 0, "list's length must be 0");
}

#[test]
//#[ignore]
fn test_push_increments_length() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    assert_eq!(list.len(), 1, "list's length must be 1");
    list.push(2);
    assert_eq!(list.len(), 2, "list's length must be 2");
}

#[test]
//#[ignore]
fn test_pop_decrements_length() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.pop();
    assert_eq!(list.len(), 1, "list's length must be 1");
    list.pop();
    assert_eq!(list.len(), 0, "list's length must be 0");
}

#[test]
//#[ignore]
fn test_pop_returns_last_added_element() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    assert_eq!(list.pop(), Some(2), "Element must be 2");
    assert_eq!(list.pop(), Some(1), "Element must be 1");
    assert_eq!(list.pop(), None, "No element should be contained in list");
}

#[test]
//#[ignore]
fn test_peek_returns_head_element() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert_eq!(list.peek(), None, "No element should be contained in list");
    list.push(2);
    assert_eq!(list.peek(), Some(&2), "Element must be 2");
    assert_eq!(list.peek(), Some(&2), "Element must be still 2");
}

#[test]
//#[ignore]
fn test_from_slice() {
    let array = ["1", "2", "3", "4"];
    let mut list = SimpleLinkedList::from(array.as_ref());
    assert_eq!(list.pop(), Some("4"));
    assert_eq!(list.pop(), Some("3"));
    assert_eq!(list.pop(), Some("2"));
    assert_eq!(list.pop(), Some("1"));
}

#[test]
//#[ignore]
fn test_reverse() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut rev_list = list.rev();
    assert_eq!(rev_list.pop(), Some(1));
    assert_eq!(rev_list.pop(), Some(2));
    assert_eq!(rev_list.pop(), Some(3));
    assert_eq!(rev_list.pop(), None);
}

#[test]
//#[ignore]
fn test_into_vector() {
    let mut v = Vec::new();
    let mut s = SimpleLinkedList::new();
    for i in 1..4 {
        v.push(i);
        s.push(i);
    }
    let s_as_vec: Vec<i32> = s.into();
    assert_eq!(v, s_as_vec);
}
