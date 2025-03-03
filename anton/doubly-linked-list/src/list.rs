use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

pub struct List<T> where T: Ord {
	head: Link<T>,
	tail: Link<T>,
	count: usize
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> where T: Ord {
	elem: T,
	next: Link<T>,
	prev: Link<T>,
}

impl<T> Node<T> where T: Ord {
	fn new(elem: T) -> Rc<RefCell<Self>> {
		Rc::new(RefCell::new(Node {
			elem: elem,
			prev: None,
			next: None
		}))
	}
}

impl<T> List<T> where T: Ord {
	pub fn new() -> Self {
		Self {
			head: None,
			tail: None,
			count: 0
		}
	}

	pub fn len(&self) -> usize {
		self.count
	}


    fn insert_helper(&mut self, cur: Link<T>, elem: T) {
        match cur {
            Some(old_head) => {
                if old_head.borrow().elem > elem {
                    // Insert before old_head
                    let new_node = Node::new(elem);
                    new_node.borrow_mut().next = Some(old_head.clone());
                    new_node.borrow_mut().prev = old_head.borrow().prev.clone();

                    if let Some(prev_node) = old_head.borrow_mut().prev.take() {
                        prev_node.borrow_mut().next = Some(new_node.clone());
                    } else {
                        // We're inserting at the head
                        self.head = Some(new_node.clone());
                    }

                    old_head.borrow_mut().prev = Some(new_node);
                } else {
                    // Continue searching
                    self.insert_helper(old_head.borrow_mut().next.clone(), elem);
                }
            }
            None => {
                // Insert as the last element
                let new_node = Node::new(elem);
                if self.tail.is_none() {
                    // List is empty
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                } else {
                    // Update the tail
                    self.tail.as_ref().unwrap().borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().prev = self.tail.clone();
                    self.tail = Some(new_node);
                }
            }
        }
    }

    pub fn insert(&mut self, elem: T) {
        let cur = self.head.take();
        self.insert_helper(cur, elem);
    }

	pub fn pop_max(&mut self) -> Option<T> {
		self.tail.take().map(|old_tail| {
			match old_tail.borrow_mut().prev.take() {
				Some(new_tail) => {
					new_tail.borrow_mut().next.take();
					self.tail = Some(new_tail);
				}
				None => {
					self.head.take();
				}
			}
			Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
		})
	}

	pub fn pop_min(&mut self) -> Option<T> {
		self.head.take().map(|old_head| {
			match old_head.borrow_mut().next.take() {
				Some(new_head) => {
					new_head.borrow_mut().prev.take();
					self.head = Some(new_head);
				}
				None => {
					self.tail.take();
				}
			}
			Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
		})
	}

	pub fn peek_min(&self) -> Option<Ref<T>> {
		self.head.as_ref().map(|node| {
			Ref::map(node.borrow(), |node| &node.elem)
		})
	}

	pub fn peek_max(&self) -> Option<Ref<T>> {
		self.tail.as_ref().map(|node| {
			Ref::map(node.borrow(), |node| &node.elem)
		})
	}

	pub fn peek_min_mut(&mut self) -> Option<RefMut<T>> {
		self.head.as_ref().map(|node| {
			RefMut::map(node.borrow_mut(), |node| &mut node.elem)
		})
	}

	pub fn peek_max_mut(&mut self) -> Option<RefMut<T>> {
		self.tail.as_ref().map(|node| {
			RefMut::map(node.borrow_mut(), |node| &mut node.elem)
		})
	}

	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
}

impl<T> Drop for List<T> where T: Ord {
	fn drop(&mut self) {
		while self.pop_min().is_some() {}
	}
}

pub struct IntoIter<T>(List<T>) where T: Ord;

impl<T> Iterator for IntoIter<T> where T: Ord {
	type Item = T;

	fn next(&mut self) -> Option<T> {
		self.0.pop_min()
	}
}

impl<T> DoubleEndedIterator for IntoIter<T> where T: Ord {
	fn next_back(&mut self) -> Option<T> {
		self.0.pop_max()
	}
}

#[test]
fn basics() {
	let mut list = List::new();

	assert_eq!(list.pop_min(), None);

	list.insert(5);
	list.insert(7);
	list.insert(3);

	for elem in list.head.iter() {
		let i: i32 = elem.borrow().elem;
		println!("{}", i);
	}

	/*assert_eq!(list.pop_min(), Some(3));
	assert_eq!(list.pop_min(), Some(5));
	assert_eq!(list.pop_min(), Some(7));*/

/*	list.insert(4);
	list.insert(5);

	assert_eq!(list.pop_min(), Some(5));
	assert_eq!(list.pop_min(), Some(4));

	assert_eq!(list.pop_min(), Some(1));
	assert_eq!(list.pop_min(), None);

	assert_eq!(list.pop_max(), None);

	list.insert(1);
	list.insert(2);
	list.insert(3);

	assert_eq!(list.pop_max(), Some(3));
	assert_eq!(list.pop_max(), Some(2));

	list.insert(4);
	list.insert(5);

	assert_eq!(list.pop_max(), Some(5));
	assert_eq!(list.pop_max(), Some(4));

	assert_eq!(list.pop_max(), Some(1));
	assert_eq!(list.pop_max(), None);*/
}

#[test]
fn peek() {
	let mut list = List::new();
	assert!(list.peek_min().is_none());
	assert!(list.peek_max().is_none());
	assert!(list.peek_min_mut().is_none());
	assert!(list.peek_max_mut().is_none());

	list.insert(1); list.insert(2); list.insert(3);

	assert_eq!(&*list.peek_min().unwrap(), &3);
	assert_eq!(&mut *list.peek_min_mut().unwrap(), &mut 3);
	assert_eq!(&*list.peek_max().unwrap(), &1);
	assert_eq!(&mut *list.peek_max_mut().unwrap(), &mut 1);
}

#[test]
fn into_iter() {
	let mut list = List::new();
	list.insert(1); list.insert(2); list.insert(3);

	let mut iter = list.into_iter();
	assert_eq!(iter.next(), Some(3));
	assert_eq!(iter.next_back(), Some(1));
	assert_eq!(iter.next(), Some(2));
	assert_eq!(iter.next_back(), None);
	assert_eq!(iter.next(), None);
}
