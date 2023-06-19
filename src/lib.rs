#![allow(dead_code)]
#![forbid(unsafe_code)]

use std::marker::PhantomData;

#[derive(Debug, Clone)]
struct HasLeft;

#[derive(Debug, Clone)]
struct HasRight;

#[derive(Debug, Clone)]
struct HasBoth;

#[derive(Debug, Clone)]
struct Nothing;

#[derive(Debug, Clone)]
struct OneOrBoth<T1, T2, State = Nothing> {
    left: Option<T1>,
    right: Option<T2>,
    state: PhantomData<State>,
}

impl<T1, T2> OneOrBoth<T1, T2, HasLeft> {
    fn unwrap_left(self) -> T1 {
        self.left.unwrap()
    }

    fn insert_right(self, right: T2) -> OneOrBoth<T1, T2, HasBoth> {
        OneOrBoth {
            left: self.left,
            right: Some(right),
            state: PhantomData::<HasBoth>,
        }
    }
}

impl<T1, T2> OneOrBoth<T1, T2, HasRight> {
    fn unwrap_right(self) -> T2 {
        self.right.unwrap()
    }
    fn insert_left(self, left: T1) -> OneOrBoth<T1, T2, HasBoth> {
        OneOrBoth {
            left: Some(left),
            right: self.right,
            state: PhantomData::<HasBoth>,
        }
    }
}

impl<T1, T2> OneOrBoth<T1, T2, HasBoth> {
    fn unwrap_left(self) -> T1 {
        self.left.unwrap()
    }
    fn unwrap_right(self) -> T2 {
        self.right.unwrap()
    }
    fn unwrap_both(self) -> (T1, T2) {
        (self.left.unwrap(), self.right.unwrap())
    }
}

impl<T1, T2> OneOrBoth<T1, T2, Nothing> {
    fn insert_both(self, left: T1, right: T2) -> OneOrBoth<T1, T2, HasBoth> {
        OneOrBoth {
            left: Some(left),
            right: Some(right),
            state: PhantomData::<HasBoth>,
        }
    }
    fn insert_left(self, left: T1) -> OneOrBoth<T1, T2, HasLeft> {
        OneOrBoth {
            left: Some(left),
            right: self.right,
            state: PhantomData::<HasLeft>,
        }
    }
    fn insert_right(self, right: T2) -> OneOrBoth<T1, T2, HasRight> {
        OneOrBoth {
            left: self.left,
            right: Some(right),
            state: PhantomData::<HasRight>,
        }
    }
}

impl<T1, T2> OneOrBoth<T1, T2> {
    fn new() -> Self {
        Self {
            left: None,
            right: None,
            state: PhantomData::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::OneOrBoth;

    #[test]
    fn insert_left() {
        let either: OneOrBoth<&str, &str> = OneOrBoth::new();
        let either = either.insert_left("Left value inserted!");
        assert_eq!(either.clone().unwrap_left(), "Left value inserted!");
    }

    #[test]
    fn insert_right() {
        let either: OneOrBoth<&str, &str> = OneOrBoth::new();
        let either = either.insert_right("Right value inserted!");
        assert_eq!(either.clone().unwrap_right(), "Right value inserted!");
    }

    #[test]
    fn insert_both_at_once() {
        let either: OneOrBoth<&str, &str> = OneOrBoth::new();
        let either = either.insert_both("Left value inserted!", "Right value inserted!");
        assert_eq!(either.clone().unwrap_left(), "Left value inserted!");
        assert_eq!(either.clone().unwrap_right(), "Right value inserted!");
        assert_eq!(
            either.clone().unwrap_both(),
            ("Left value inserted!", "Right value inserted!")
        );
    }

    #[test]
    fn insert_both_with_separate_methods() {
        let either: OneOrBoth<&str, &str> = OneOrBoth::new();
        let either = either.insert_left("Left value inserted!");
        let either = either.insert_right("Right value inserted!");
        assert_eq!(either.clone().unwrap_left(), "Left value inserted!");
        assert_eq!(either.clone().unwrap_right(), "Right value inserted!");
        assert_eq!(
            either.clone().unwrap_both(),
            ("Left value inserted!", "Right value inserted!")
        );
    }
}
