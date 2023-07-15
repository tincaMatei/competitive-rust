/// General Fenwick Tree struct that can be customized easily.
pub struct FenwickTree<T> {
    pub data: Vec<T>,
}

#[inline]
fn lsb(val: usize) -> usize {
    val & (!val + 1)
}

impl<T> FenwickTree<T> {
    /// Create an empty Fenwick Tree with default values.
    pub fn new(n: usize) -> FenwickTree<T> 
    where T: Default {
        FenwickTree {
            data: (0..n + 1).map(|_| { T::default() }).collect()
        }
    }

    /// Create a Fenwick Tree from its underlying data.
    pub fn from_data(data: Vec<T>)  -> FenwickTree<T>{
        FenwickTree {
            data
        }
    }

    /// Update a Fenwick Tree at the given position.
    ///
    /// `update` is a function that receives the Fenwick Tree mutable reference, so it applies the
    /// update on that node.
    pub fn update<F>(&mut self, mut pos: usize, update: F)
    where F: Fn(&mut T) {
        if pos == 0 || pos >= self.data.len() {
            panic!("Update happens outside of Fenwick Tree bounds: {}, length is {}.", pos, self.data.len())
        }
    
        while pos < self.data.len() {
            update(&mut self.data[pos]);
            pos += lsb(pos);
        }
    }

    /// Query the Fenwick Tree at a given position.
    ///
    /// `neutral` is the neutral element of the ring on which the Fenwick Tree works. For instance,
    /// when doing sums over ranges, `neutral` should be 0.
    ///
    /// `composition` should combine the resultant type with a node from the Fenwick Tree and
    /// return a new number, that is the "sum" of the two.
    pub fn query<Q, F>(&self, mut pos: usize, neutral: Q, composition: F) -> Q
    where F: Fn(Q, &T) -> Q,
          Q: Copy {
        let mut res = neutral;

        if pos >= self.data.len() {
            panic!("Query on Fenwick Tree outside bounds: {}", pos);
        }

        while pos > 0 {
            res = composition(res, &self.data[pos]);
            pos -= lsb(pos);
        }

        res
    }

    /// Binary searches a property on the Fenwick Tree.
    ///
    /// `neutral` is the neutral element of the ring on which the Fenwick Tree works. For instance,
    /// when doing sums over ranges, `neutral` should be 0.
    ///
    /// `composition` should combine the resultant type with a node from the Fenwick Tree and
    /// return a new number, that is the "sum" of the two.
    ///
    /// `eval` should be an evaluation function that returns `true` if the given value is too
    /// small, or `false` if it is too large. Therefore, this function will return a pair (x, y) where `x`
    /// is the largest position such that `eval(query(x)) = true` and `y` is the lowest number such that
    /// `eval(query(y)) = false`. In particular, `y = x + 1`. This works on the assumption that `eval(query(0)) = true` 
    /// and `eval(query(n + 1)) = false`.
    pub fn bin_search<F, E, Q>(&self, eval: E, neutral: Q, composition: F) -> (usize, usize)
    where E: Fn(Q) -> bool,
          F: Fn(Q, &T) -> Q,
          Q: Copy {
        let mut pos = 0;
        let mut sum = neutral;

        for l in (0..30).rev() {
            if pos + (1 << l) < self.data.len() {
                let new_pos = pos + (1 << l);
                let new_sum = composition(sum, &self.data[new_pos]);

                if eval(new_sum) {
                    pos = new_pos;
                    sum = new_sum;
                }
            }
        }

        (pos, pos + 1)
    }
}

use std::ops::{Add, Sub};

impl<T> FenwickTree<T>
where T: Copy + Default + Add<Output = T> + Sub<Output = T> {
    /// Add a value to a position in the Fenwick Tree.
    pub fn add_value(&mut self, pos: usize, val: T) {
        self.update(pos, |e| { *e = *e + val; });
    }

    /// Find the prefix sum of the Fenwick Tree at a position.
    pub fn prefix_sum(&self, pos: usize) -> T {
        self.query(pos, T::default(), |s, e| { s + *e })
    }

    /// Find the range sum in the Fenwick Tree at two given positions.
    ///
    /// The query range takes both the `start` and the `end`. In particular, `range_sum(b, e) =
    /// sum(b..=e)`.
    pub fn range_sum(&self, start: usize, end: usize) -> T {
        self.query(end, T::default(), |s, e| { s + *e }) -
            self.query(start - 1, T::default(), |s, e| { s + *e })
    }

    /// Binary search on the prefix sums of the Fenwick Tree.
    pub fn bin_search_sum<E>(&self, eval: E) -> (usize, usize)
    where E: Fn(T) -> bool {
        self.bin_search(eval, T::default(), |s, e| { s + *e })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsb_1() {
        assert_eq!(lsb(1), 1);
    }

    #[test]
    fn test_lsb_6() {
        assert_eq!(lsb(6), 2);
    }

    #[test]
    fn test_lsb_8() {
        assert_eq!(lsb(8), 8);
    }

    #[test]
    fn test_lsb_10() {
        assert_eq!(lsb(10), 2);
    }

    #[test]
    fn test_addition() {
        let mut ft = FenwickTree::<i32>::new(5);

        ft.add_value(2, 5);
        ft.add_value(3, 4);

        assert_eq!(5, ft.prefix_sum(2));
        assert_eq!(9, ft.prefix_sum(3));
        assert_eq!(9, ft.prefix_sum(5));
        assert_eq!(vec![0, 0, 5, 4, 9, 0], ft.data);
    }
    
    #[test]
    fn test_additive_large() {
        use rand::rngs::SmallRng;
        use rand::{Rng, SeedableRng};
        
        const LEN: usize = 100;
        const Q: usize = 10000;

        let mut ft = FenwickTree::<i32>::new(LEN);
        let mut rng = SmallRng::seed_from_u64(269_696_969);
        
        let mut v = vec![0i32; 1 + LEN];

        for _ in 0..Q {
            let t = rng.gen_range(0..=1);
            
            match t {
            0 => {
                let (pos, val) = (rng.gen_range(1..=LEN), rng.gen_range(-1_000i32..=1_000i32));
                v[pos] += val;
                ft.add_value(pos, val);
            }
            1 => {
                let (mut a, mut b) = (rng.gen_range(1..=LEN), rng.gen_range(1..=LEN)) ;
                
                if a > b {
                    std::mem::swap(&mut a, &mut b);
                }

                let correct_sum = (a..=b).fold(0, |sum, e| { sum + v[e] });
                let ft_sum = ft.range_sum(a, b);

                assert_eq!(correct_sum, ft_sum);
            }
            _ => {
                panic!("Invalid operation");
            }
            }
        }
    }

    #[test]
    fn test_binary_search() {
        let mut ft = FenwickTree::<i32>::new(10);

        // index:       0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10
        // prefix sums: 0,  0,  3,  4,  6, 12, 16, 16, 20, 20, 25
        ft.add_value(2,  3);
        ft.add_value(3,  1);
        ft.add_value(4,  2);
        ft.add_value(5,  6);
        ft.add_value(6,  4);
        ft.add_value(8,  4);
        ft.add_value(10, 5);

        assert_eq!((5, 6), ft.bin_search_sum(|val| { val <= 12 }));
        assert_eq!((0, 1), ft.bin_search_sum(|val| { val <= -1 } ));
        assert_eq!((10, 11), ft.bin_search_sum(|val| { val <= 26 } ));
        assert_eq!((7, 8), ft.bin_search_sum(|val| { val <= 16 } ));
    }
}
