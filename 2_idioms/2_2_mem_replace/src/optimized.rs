use std::mem;



#[derive(Clone, Debug, PartialEq)]
struct Trinity<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Trinity<T> {
    fn rotate(&mut self) {
        mem::swap(&mut self.a, &mut self.b);
        mem::swap(&mut self.b, &mut self.c);
    }
}

#[derive(Debug)]
struct Solver<T> {
    expected: Trinity<T>,
    unsolved: Vec<Trinity<T>>,
}

impl<T: PartialEq> Solver<T> {
    fn resolve(&mut self) {
        // This "replace" is equal to below "take()"
        // let mut unsolved = mem::replace(&mut self.unsolved, Default::default());
        let mut unsolved = mem::take(&mut self.unsolved);
        unsolved.retain_mut(|t| {
            for _ in 0..3 {
                if t == &self.expected { return false; }
                t.rotate();
            }
            true
        });
        self.unsolved = unsolved;
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized() {
        let expected_result = vec![Trinity { a: 2, b: 1, c: 3 }];
        let mut s = Solver {
            expected: Trinity { a: 1, b: 2, c: 3 },
            unsolved: vec![
                Trinity { a: 1, b: 2, c: 3 },
                Trinity { a: 2, b: 1, c: 3 },
                Trinity { a: 2, b: 3, c: 1 },
                Trinity { a: 3, b: 1, c: 2 },
            ],
        };
        s.resolve();

        assert_eq!(s.unsolved, expected_result)
    }
}