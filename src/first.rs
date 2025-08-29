use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }


    // Check if the list is empty.
    // If it's empty, just return None
    // If it's not empty
    //     remove the head of the list
    //     remove its elem
    //     replace the list's head with its next
    //     return Some(elem)
    pub fn pop(&mut self) -> Option<i32> {
        match  mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        };
        result
    }

}