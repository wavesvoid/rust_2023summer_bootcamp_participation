use std::sync::{Arc, Mutex, RwLock};


// It is safe to use RefCell and Rc
// because the Developer guarantees it will be in sync
type NodeItem<T> = Arc<RwLock<Node<T>>>;


// Needed to select action for DLL
enum NodeInsPosition {
    Head,
    Tail
}



pub struct DLList<T>(Mutex<DLListIterior<T>>);
impl<T> DLList<T> {
    pub fn new() -> Self {
        Self(Mutex::new(DLListIterior::new()))
    }

    pub fn push_back(&self, data: T) {
        self.0.lock().unwrap().push_back(data);
    }
    pub fn push_front(&self, data: T) {
        self.0.lock().unwrap().push_front(data);
    }
    
    
    pub fn pop_back(&self) -> Option<T> {
        self.0.lock().unwrap().pop_back()
    }
    pub fn pop_front(&self) -> Option<T> {
        self.0.lock().unwrap().pop_front()
    }
}


struct DLListIterior<T> {
    head: Option<NodeItem<T>>,
    tail: Option<NodeItem<T>>,
}

impl<T> DLListIterior<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        self.guarded_insert(data, NodeInsPosition::Head);
    }

    pub fn push_back(&mut self, data: T) {
        self.guarded_insert(data, NodeInsPosition::Tail);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let data = self.guarded_drop();
        if data.is_some() {
            return data;
        }
        self.drop_head()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let data = self.guarded_drop();
        if data.is_some() {
            return data;
        }
        self.drop_tail()
    }

    fn guarded_insert(&mut self, data: T, whereat: NodeInsPosition) {
        let new_node = Arc::new(RwLock::new(Node::new(
            data,
            None, // prev
            None, // next
        )));

        // If head and tail are empty
        if self.head.is_none() && self.tail.is_none() {
            self.head = Some(Arc::clone(&new_node));
            self.tail = Some(Arc::clone(&new_node));
            return;
        }
        
        match whereat {
            NodeInsPosition::Head => {
                if let Some(head) = self.head.take() {
                    head.write().unwrap().prev.replace(Arc::clone(&new_node));
                    new_node.write().unwrap().next.replace(head);
                }
                self.head.replace(new_node);
            }
            NodeInsPosition::Tail => {
                if let Some(tail) = self.tail.take() {
                    tail.write().unwrap().next.replace(Arc::clone(&new_node));
                    new_node.write().unwrap().prev.replace(tail);
                }
                self.tail.replace(new_node);
            }
        }
    }

    /// Check if head has the same node as tail
    fn guarded_drop(&mut self) -> Option<T> {
        let mut head_eqals_tail = false;

        let ptrs = (self.head.as_ref(), self.tail.as_ref());
        if let (Some(head), Some(tail)) = ptrs {
            if Arc::ptr_eq(head, tail) {
                head_eqals_tail = true;
            }
        }
        
        if head_eqals_tail {
            self.head.take().map(|arc| Arc::downgrade(&arc));
            return self.drop_tail();
        }

        None
    }

    /// Extract node from list and return its data
    fn drop_head(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            self.head = head.write().unwrap().next.take();

            if let Some(next) = self.head.as_ref() {
                let mut next_ptr = next.write().unwrap();
                next_ptr.prev.take().map(|ptr| Arc::downgrade(&ptr));
            }

            return Some(Arc::into_inner(head).unwrap().into_inner().unwrap().data);
        }
        None
    }

    /// Extract node from list and return its data
    fn drop_tail(&mut self) -> Option<T> {
        if let Some(tail) = self.tail.take() {
            self.tail = tail.write().unwrap().prev.take();

            if let Some(next) = self.tail.as_ref() {
                let mut next_ptr = next.write().unwrap();
                next_ptr.next.take().map(|ptr| Arc::downgrade(&ptr));
            }
            
            let inner = Arc::into_inner(tail).unwrap();
            return Some(inner.into_inner().unwrap().data);
        }
        None
    }
}

impl<T> Iterator for DLListIterior<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { self.pop_front() }
}

// Prevent memory leakage when list goes out of scope
// We could actually use Weak references instead of Drop above
impl<T> Drop for DLListIterior<T> {
    fn drop(&mut self) { self.into_iter().count(); }
}


struct Node<T> {
    data: T,
    prev: Option<NodeItem<T>>,
    next: Option<NodeItem<T>>,
}

impl<T> Node<T> {
    fn new(data: T,
           prev: Option<NodeItem<T>>,
           next: Option<NodeItem<T>>) -> Node<T>
    {
        Self { data, prev, next }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    use std::thread;

    #[test]
    fn test_ll() {
        let ll = DLList::new();

        thread::scope(|s| {
            let list: Vec<i32> = (1..25).collect();

            // Fill-in Dyn Linked LIst in multithreading
            //
            let shared_ll = &ll;
            s.spawn(move || {
                (1..25).into_iter().for_each(|i| {
                    shared_ll.push_front(i);
                });
            });
            
            s.spawn(move || {
                (1..25).into_iter().for_each(|i| {
                    shared_ll.push_back(i);
                });
            });

            
            s.spawn(move || {
                let mut checklist_front = Vec::<i32>::new();
                let mut checklist_back = Vec::<i32>::new();

                // Collect DLL into vectors
                //
                (1..25).into_iter().for_each(|_| {
                    checklist_front.push(shared_ll.pop_front().unwrap());
                    checklist_back.push(shared_ll.pop_back().unwrap());
                });

                // Sort chaotically built lists
                //
                checklist_front.sort();
                checklist_back.sort();

                assert!(checklist_front.eq(&list));
                assert!(checklist_back.eq(&list));
            });
        });


        ll.push_back(43);
        ll.push_back(44);
        ll.push_back(45);

        ll.push_front(2);
        ll.push_front(1);
        ll.push_front(3);
        ll.push_front(4);

        assert_eq!(ll.pop_back(), Some(45));
        assert_eq!(ll.pop_back(), Some(44));
        assert_eq!(ll.pop_back(), Some(43));

        assert_eq!(ll.pop_front(), Some(4));
        assert_eq!(ll.pop_front(), Some(3));
        assert_eq!(ll.pop_front(), Some(1));
        assert_eq!(ll.pop_front(), Some(2));
    }
}