use core::ptr::NonNull;
use core::marker::PhantomData;

pub struct LinkedList<T> {
    head : Option<NonNull<Node<T>>>,
    tail : Option<NonNull<Node<T>>>,
    length : usize,
    marker : PhantomData<Box<Node<T>>>,
}

pub struct Iter<'a, T : 'a> {
    head : Option<NonNull<Node<T>>>,
    tail : Option<NonNull<Node<T>>>,
    length : usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.length == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                // Need an unbound lifetime to get 'a
                let node = &*node.as_ptr();
                self.length -= 1;
                self.head = node.next;
                &node.data
            })
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head : None,
            tail : None,
            length : 0,
            marker : PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head == None
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head : self.head, tail : self.tail, length : self.length, marker : PhantomData
        }
    }

    pub fn push_front(&mut self, data : T) {
        let mut node = Box::new(Node::new(data));
        unsafe {
            node.next = self.head;
            node.prev = None;
            let node = Some(Box::leak(node).into());

            match self.head {
                None => self.tail = node,
                Some(head) => (*head.as_ptr()).prev = node,
            }
            self.head = node;
            self.length += 1;
        }
    }

    pub fn push_back(&mut self, data : T) {
        let mut node = Box::new(Node::new(data));
        unsafe {
            node.next = None;
            node.prev = self.tail;
            let node = Some(Box::leak(node).into());

            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }
            self.tail = node;
            self.length += 1;
        }
    }

    fn pop_front(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                Some(head) => (*head.as_ptr()).prev = None,
            }
            self.length -= 1;
            node
        })
    }

    fn pop_back(&mut self) -> Option<Box<Node<T>>> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,
                Some(tail) => (*tail.as_ptr()).next = None,
            }
            self.length -= 1;
            node
        })
    }

    pub fn pop_front_value(&mut self) -> T {
        if self.is_empty() {
            panic!("no value availabe");
        }
        self.pop_front().unwrap().data
    }

    pub fn pop_back_value(&mut self) -> T {
        if self.is_empty() {
            panic!("no value availabe");
        }
        self.pop_back().unwrap().data
    }

    pub fn peek(&self) -> &T {
        unsafe { 
            self.head.as_ref().map(|node| &node.as_ref().data).unwrap()
        }
    }
}

struct Node<T> {
    data : T,
    prev : Option<NonNull<Node<T>>>,
    next : Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data : T) -> Self {
        Node {
            data : data,
            prev : None,
            next : None
        }
    }
}
