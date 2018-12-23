use std::mem;
use std::ptr;

pub struct LinkedList<T> {
    head: *const ListNode<T>,
}

pub struct ListNode<T> {
    prev: *mut ListNode<T>,
    next: *mut ListNode<T>,
    pub val: T,
}

#[allow(dead_code)]
pub struct LinkedListIter<'a, T> {
    start: Option<&'a ListNode<T>>,
    ptr: &'a ListNode<T>,
}

#[allow(dead_code)]
impl<T> LinkedList<T> {
    pub fn new(val: T) -> LinkedList<T> {
        let p: *mut ListNode<T> = Box::into_raw(
            box ListNode {
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
                val,
            });
        unsafe {
            (*p).prev = p;
            (*p).next = p;
        }
        LinkedList { head: p }
    }

    pub fn head(&self) -> &ListNode<T> {
        unsafe { self.head.as_ref().unwrap() }
    }

    pub fn head_mut(&mut self) -> &mut ListNode<T> {
        unsafe { (self.head as *mut ListNode<T>).as_mut().unwrap() }
    }

    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            start: None,
            ptr: self.head(),
        }
    }
}

#[allow(dead_code)]
impl<T> ListNode<T> {
    fn new(prev: *mut ListNode<T>, next: *mut ListNode<T>, val: T) -> *mut ListNode<T> {
        Box::into_raw(box ListNode { prev, next, val })
    }

    pub fn as_head(&self) -> LinkedList<T> {
        LinkedList { head: self }
    }

    pub fn insert_after(&mut self, val: T) -> &mut ListNode<T> {
        let p = ListNode::<T>::new(self, self.next, val);
        unsafe {
            (*self.next).prev = p;
        }
        self.next = p;
        return unsafe { p.as_mut().unwrap() };
    }

    /// Remove current node and return next node
    pub fn remove(&mut self) -> &mut ListNode<T> {
        unsafe {
            let ret = self.next.as_mut().unwrap();
            (*self.prev).next = self.next;
            (*self.next).prev = self.prev;
            mem::drop(Box::from_raw(self));
            return ret;
        }
    }

    pub fn next(&self) -> &ListNode<T> {
        unsafe { self.next.as_ref().unwrap() }
    }

    pub fn next_mut(&mut self) -> &mut ListNode<T> {
        unsafe { self.next.as_mut().unwrap() }
    }

    pub fn prev(&self) -> &ListNode<T> {
        unsafe { self.prev.as_ref().unwrap() }
    }

    pub fn prev_mut(&mut self) -> &mut ListNode<T> {
        unsafe { self.prev.as_mut().unwrap() }
    }
}

impl<'a, T: 'a> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.start {
            None => self.start = Some(self.ptr),
            Some(val) => if ptr::eq(val, self.ptr) { return None; }
        };
        let p = self.ptr;
        unsafe {
            self.ptr = self.ptr.next.as_ref().unwrap();
        }
        return Some(&p.val);
    }
}
