use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl TransactionLog {
    fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => old_tail.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        }
        self.length += 1;
        self.tail = Some(new);
    }
}

#[cfg(test)]
mod tests {
    use crate::TransactionLog;

    #[test]
    fn test_new_empty() {
        let tn_log = TransactionLog::new_empty();
        assert!(tn_log.head.is_none());
        assert!(tn_log.tail.is_none());
        assert_eq!(tn_log.length, 0)
    }
}
