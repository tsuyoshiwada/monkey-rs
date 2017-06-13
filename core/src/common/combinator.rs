trait Parser<T, E> {
    fn next(&mut self) -> Option<T>;
    fn preview(&self) -> Option<&T>;
    fn current_pos(&self) -> (i32, i32);
    fn error(&self, message: &str) -> E;
}

macro_rules! next {
    ($p:expr) => {
        match $p.next() {
            Some(x) => x,
            None => return Err($p.error("unexpected eof")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::vec_deque::VecDeque;

    struct TP {
        input: VecDeque<i32>,
    }

    impl TP {
        fn new(input: Vec<i32>) -> TP {
            TP { input: VecDeque::from(input) }
        }
    }

    impl Parser<i32, String> for TP {
        fn next(&mut self) -> Option<i32> {
            self.input.pop_front()
        }

        fn preview(&self) -> Option<&i32> {
            self.input.front()
        }

        fn current_pos(&self) -> (i32, i32) {
            (0, 0)
        }

        fn error(&self, message: &str) -> String {
            message.to_string()
        }
    }

    macro_rules! with_res {
        (test $i:ident, $b:block) => {
            #[test]
            fn $i() {
                let result = || {
                    $b;
                    Ok(())
                };
                result().unwrap()
            }
        };
        (panic $e:expr, $i:ident, $b:block) => {
            #[test]
            #[should_panic(expected = $e)]
            fn $i() {
                let result = || {
                    $b;
                    Ok(())
                };
                result().unwrap()
            }
        };
    }

    with_res!(test next_success, {
        let mut p = TP::new(vec![1, 2, 3]);
        assert_eq!(next!(p), 1);
        assert_eq!(next!(p), 2);
        assert_eq!(next!(p), 3);
    });

    with_res!(panic "unexpected eof", next_fail_empty, {
        let mut p = TP::new(vec![]);
        assert_eq!(next!(p), 1);
    });
}