use std::sync::{Arc, Weak};


trait DLL<T> {
    /// Insert new node at the beginning of the list
    fn push_front() -> Result<(), LLError>;
    /// Insert new node at the end
    fn push_back() -> Result<(), LLError>;
    /// Take from the head
    fn pop_front() -> Option<T>;
    /// Take from back
    fn pop_back() -> Option<T>;
}



struct Node<T> {
    data: Box<T>,
    prev: Option<RwLock<Node>>
    next: Option<RwLock<Node>>
}

struct DLList {
    head: Option<RwLock<Node>>,
    tail: Option<RwLock<Node>>,
}


impl<T> DLL<T> for DLList {
    fn push_front(self, data: T) -> Result<(), LLError> {
        if let Some(p) = self.prev {
            let node = Node::new(data, None, Arc::downgrade(&p));
            p.add_or_replace_prev()
        }
    }
}


#[cfg(test)]
mod tests {
    fn test_insertion() {

    }
}