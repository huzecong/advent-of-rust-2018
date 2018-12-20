#![feature(box_syntax)]

use std::fs;
use std::mem;
use std::ptr;

struct LinkedList<T> {
    head: *const ListNode<T>,
}

struct ListNode<T> {
    prev: *mut ListNode<T>,
    next: *mut ListNode<T>,
    val: T,
}

#[allow(dead_code)]
struct LinkedListIter<'a, T> {
    start: Option<&'a ListNode<T>>,
    ptr: &'a ListNode<T>,
}

#[allow(dead_code)]
impl<T> LinkedList<T> {
    fn new(val: T) -> LinkedList<T> {
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

    fn head(&self) -> &ListNode<T> {
        unsafe { self.head.as_ref().unwrap() }
    }

    fn head_mut(&mut self) -> &mut ListNode<T> {
        unsafe { (self.head as *mut ListNode<T>).as_mut().unwrap() }
    }

    fn iter(&self) -> LinkedListIter<T> {
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

    fn as_head(&self) -> LinkedList<T> {
        LinkedList { head: self }
    }

    fn insert_after(&mut self, val: T) -> &mut ListNode<T> {
        let p = ListNode::<T>::new(self, self.next, val);
        unsafe {
            (*self.next).prev = p;
        }
        self.next = p;
        return unsafe { p.as_mut().unwrap() };
    }

    /// Remove current node and return next node
    fn remove(&mut self) -> &mut ListNode<T> {
        unsafe {
            let ret = self.next.as_mut().unwrap();
            (*self.prev).next = self.next;
            (*self.next).prev = self.prev;
            mem::drop(Box::from_raw(self));
            return ret;
        }
    }

    fn next(&self) -> &ListNode<T> {
        unsafe { self.next.as_ref().unwrap() }
    }

    fn next_mut(&mut self) -> &mut ListNode<T> {
        unsafe { self.next.as_mut().unwrap() }
    }

    fn prev(&self) -> &ListNode<T> {
        unsafe { self.prev.as_ref().unwrap() }
    }

    fn prev_mut(&mut self) -> &mut ListNode<T> {
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

fn parse(s: &str) -> (usize, usize) {
    let parts = s.trim().split_whitespace().collect::<Vec<_>>();
    let f = |idx: usize| parts[idx].parse::<usize>().ok().unwrap();
    return (f(0), f(6));
}

fn simulate(n_players:usize, n_rounds:usize) -> Vec<usize> {
    let mut scores = vec![0; n_players];
    let mut list = LinkedList::new(0);
    let mut cur = list.head_mut();
    for round in 1..=n_rounds {
        if round % 23 == 0 {
            let player = (round - 1) % n_players;
            scores[player] += round;
            for _ in 0..7 { cur = cur.prev_mut() }
            scores[player] += cur.val;
            cur = cur.remove();
        } else {
            cur = cur.next_mut().insert_after(round);
        }
//        println!("{:?}", list.iter().collect::<Vec<_>>());
    }
    return scores;
}

fn main() {
    let input = fs::read_to_string("input.txt").ok().unwrap();
    let (n_players, n_rounds) = parse(&input);

    // Part 1
    let scores = simulate(n_players, n_rounds);
    println!("{}", scores.iter().max().unwrap());

    // Part 2
    let scores = simulate(n_players, n_rounds * 100);
    println!("{}", scores.iter().max().unwrap());
}
