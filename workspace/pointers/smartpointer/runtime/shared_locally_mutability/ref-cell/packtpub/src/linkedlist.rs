

#![allow(dead_code, unused_variables)]


/// linkedlist
///
/// ## Commands
///
/// ```cargo test -q -p packtpub-smartpointer-ref-cell-linkedlist_lib --test linkedlist -- --nocapture``
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `unimplemented`
///
/// ## Example
/// //```rust,no_run,compile_fail,ignore


#[derive(Clone)]
struct Node<T> where T: Sized + Clone {
    value: T,
    next: Link<T>,
}
impl<T> Node<T> where T: Sized + Clone {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}
type Link<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(Clone)]
pub struct List<T> where T: Sized + Clone {
    head: Link<T>,
    tail: Link<T>,
    pub length: usize,
}
impl<T> List<T> where T: Sized + Clone {
    pub fn new_empty() -> List<T> {
        List { head: None, tail: None, length: 0 }
    }
    pub fn append(&mut self, value: T) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()), 
            None => self.head = Some(new.clone())
        }; 
        self.length += 1;
        self.tail = Some(new);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;
    #[bench]
    fn bench_list_append(b: &mut Bencher) {
        let mut list = List::new_empty();
        b.iter(|| {
            list.append(10);
        });
    }
    #[test]
    fn test_list_new_empty() {
        let mut list: List<i32> = List::new_empty();
        assert_eq!(list.length, 0);
        assert_eq!(list.pop(), None);
    } 
    #[test]
    fn test_list_append() {
        let mut list = List::new_empty();
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        assert_eq!(list.length, 5);
    }
    #[test]
    fn test_list_pop() {
        let mut list = List::new_empty();
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        assert_eq!(list.length, 5);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.length, 0);
        assert_eq!(list.pop(), None);
    }
}


