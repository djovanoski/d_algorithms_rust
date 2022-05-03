#![allow(unused)]
use std::{fmt::Display, marker::PhantomData, ptr::NonNull};

struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Self {
            element,
            prev: None,
            next: None,
        }
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

pub struct LinkedList<T> {
    length: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _marker: PhantomData<Box<Node<T>>>,
}

// Impl public functions
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push_front(&mut self, element: T) {
        self.push_front_node(Box::new(Node::new(element)));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_element)
    }

    pub fn push_back(&mut self, element: T) {
        self.push_back_node(Box::new(Node::new(element)))
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(Node::into_element)
    }

    pub fn push_at(&mut self, element: T, index: usize) {
        if self.len() < index {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head == None {
            self.push_front_node(Box::new(Node::new(element)))
        } else if self.len() == index {
            self.push_back_node(Box::new(Node::new(element)))
        } else {
            self.push_at_node(Box::new(Node::new(element)), index)
        }
    }
}

/// Push node to the front(head)
/// 1. assign self.head to the node next
/// 2. assign None to node.prev as we are becoming the head and this node must to know that it
///    points to None
/// 3. Get the pointer to the node
/// 4. Match the Option of the self.head
///     if is None this node become tail
///     if is Some on the node before add pointer to this node
/// 5. this node also is a head
///
/// So we end up with a linked list where the head and tail point to same Node if we start with
/// empty Linkedlist
impl<T> LinkedList<T> {
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head;
        node.prev = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        match self.head {
            None => self.tail = node_ptr,
            Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).prev = node_ptr },
        }
        self.head = node_ptr;
        self.length += 1;
    }

    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
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
}

impl<T> LinkedList<T> {
    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        node.next = None;
        node.prev = self.tail;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr },
        }

        self.tail = node_ptr;
        self.length += 1;
    }

    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
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
}

impl<T> LinkedList<T> {
    fn push_at_node(&mut self, mut node: Box<Node<T>>, index: usize) {
        if let Some(mut ith_node) = self.head {
            for i in 0..index {
                println!("{}", i);
                unsafe {
                    match (*ith_node.as_ptr()).next {
                        None => panic!("Index out of bounds"), // This will never happen as we have checked out of bounds previosly
                        Some(next_ptr) => ith_node = next_ptr,
                    }
                }
            }

            unsafe {
                node.prev = (*ith_node.as_ptr()).prev;
                node.next = Some(ith_node);
                if let Some(p) = (*ith_node.as_ptr()).prev {
                    (*p.as_ptr()).next =
                        Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
                    self.length += 1;
                };
            }
        }
    }
}


impl<T> LinkedList<T> {
    pub fn pop_at(&mut self, index: usize) -> Option<T> {
        if self.len() < index {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head == None {
            self.pop_front()
        } else if self.length == index {
            self.pop_back()
        } else {
            self.pop_at_node(index).map(Node::into_element)
        }
    }

    fn pop_at_node(&mut self, index: usize) -> Option<Box<Node<T>>> {
        if let Some(mut ith_node) = self.head {
            for _ in 0..index {
                unsafe {
                    match (*ith_node.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(next_ptr) => ith_node = next_ptr,
                    }
                }
            }

            unsafe {
                let old_ptr = Box::from_raw(ith_node.as_ptr());
                if let Some(mut prev) = old_ptr.prev {
                    prev.as_mut().next = old_ptr.next
                }
                if let Some(mut next) = old_ptr.next {
                    next.as_mut().prev = old_ptr.prev
                }

                self.length -= 1;
                Some(old_ptr)
            }
        } else {
            None
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front_node().is_some() {}
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.head {
            Some(node) => write!(f, "[{}]", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.element, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.element),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_push_head() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(2);
        list.push_front(3);
        list.push_front(12);
        println!("{}", list);
    }

    #[test]
    fn test_push_at() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(12);
        list.push_back(13);
        list.push_back(14);
        list.push_at(2, 2);
        println!("{}", list);
    }

    #[test]
    fn test_pop_at_index() {
        let mut list = LinkedList::<i32>::new();
        list.push_at(12, 0);
        list.push_back(11);
        list.push_front(13);
        let _ = list.pop_at(2);
        println!("{}", list);
    }
}
