#![allow(unused)]
use std::fmt::Debug;

pub fn bubble_sort<T>(v: &mut [T])
where
    T: PartialOrd + Debug,
{
    for _ in 0..v.len() {
        // println!("{:?}", v);
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1)
            }
        }
    }
}

pub fn merge_sort<T>(mut v: Vec<T>) -> Vec<T>
where
    T: Debug + PartialOrd,
{
    if v.len() <= 1 {
        return v;
    }
    let mut result: Vec<T> = Vec::with_capacity(v.len());
    let r_half = v.split_off(v.len() / 2);
    let l_half = merge_sort(v);
    let r_half = merge_sort(r_half);

    let mut l_half_it = l_half.into_iter();
    let mut r_half_it = r_half.into_iter();
    let mut l_peek = l_half_it.next();
    let mut r_peek = r_half_it.next();

    loop {
        match l_peek {
            Some(ref l_val) => match r_peek {
                Some(ref r_val) => {
                    if r_val < l_val {
                        result.push(r_peek.take().unwrap());
                        r_peek = r_half_it.next();
                    } else {
                        result.push(l_peek.take().unwrap());
                        l_peek = l_half_it.next();
                    }
                }
                None => {
                    result.push(l_peek.take().unwrap());
                    result.extend(r_half_it);
                    return result;
                }
            },
            None => {
                if let Some(r_val) = r_peek {
                    result.push(r_val);
                }
                result.extend(r_half_it);
                return result;
            }
        }
    }
}

pub fn pivot<T>(v: &mut [T]) -> usize
where
    T: PartialOrd + Debug,
{
    let mut p = 0;
    for i in 1..v.len() {
        // println!("{:?}",v );
        if v[i] < v[p] {
            // Move our pivor forward 1, and put its element before it
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1
        }
    }
    p
}

pub fn quick_sort<T>(v: &mut [T])
where
    T: PartialOrd + Debug,
{
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

pub fn fibonacci_dynamic(n: i32) -> (i32, i32) {
    // (result, previous)
    if n == 0 {
        return (1, 0);
    }
    let (a, b) = fibonacci_dynamic(n - 1);
    (a + b, a)
}


pub fn fibonaci_iter(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut res = 1;

    for _ in 1..n {
        res = a + b;
        a = b;
        b = res;
    }

    res
}


pub fn fibonaci(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    fibonaci(n-1) + fibonaci(n -2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 9, 12, 3, 13];
        bubble_sort(&mut v);
        assert_eq!(v, vec![3, 4, 6, 9, 12, 13]);
    }
    #[test]
    fn test_merge_sort() {
        let v = vec![12, 9, 4];
        let v = merge_sort(v);
        assert_eq!(v, vec![4, 9, 12]);
    }

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 8, 2, 7, 9, 5, 13, 8];
        let p = pivot(&mut v);
        assert_eq!(p, 2);
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 12, 9, 17, 23, 7, 2];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 4, 6, 7, 8, 9, 12, 17, 23]);
    }

    #[test] 
    fn test_fibonaci() {
        let (a, b) = fibonacci_dynamic(5);
        assert_eq!(a, 8);
    }

    #[test]
    fn test_fibonaci_normal() {
        let a = fibonaci(5);
        assert_eq!(a, 8);
    }

    #[test]
    fn test_fibonaci_iter() {
        let a = fibonaci_iter(5);
        assert_eq!(a, 8);
    }
}
