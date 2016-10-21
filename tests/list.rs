#[macro_use]
extern crate porus;

use porus::traits::*;
use porus::collections::list;

pub mod common;
use common::drop::{Counter, Item};


#[test]
fn test_drop() {
    let counter = Counter::new();
    {
        let stack = &mut list::new();

        for _ in 0..5 {
            Stack::push(stack, Item::new(counter.clone()));
        }
    }

    assert!(counter.borrow().count() == 5)
}


#[test]
fn test_deque() {
    common::deque::test_deque(&mut list::new());
    common::deque::test_deque(&mut list::with_capacity(5));
}


#[test]
#[should_panic(expected="empty")]
fn test_deque_empty_pop_front() {
    common::deque::test_empty_pop_front(&mut list::new());

}


#[test]
#[should_panic(expected="empty")]
fn test_deque_empty_pop_back() {
    common::deque::test_empty_pop_back(&mut list::new());
}


#[test]
#[should_panic(expected="overflow")]
fn test_deque_bounded_push_front_overflow() {
    common::deque::test_bounded_push_front_overflow(&mut list::with_capacity(5));
}


#[test]
#[should_panic(expected="overflow")]
fn test_deque_bounded_push_back_overflow() {
    common::deque::test_bounded_push_back_overflow(&mut list::with_capacity(5));
}
