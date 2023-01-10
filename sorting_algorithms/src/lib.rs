use std::fmt::Debug;
mod b_rand;

pub fn bubble_sort<T: PartialOrd > ( v: &mut [T] ) {
    for _ in 0..v.len() {
        for i in 0..v.len() - 1 {
            if v[i] > v[i+1] {
                v.swap(i, i+1);
            }
        }
    }
}


pub fn merge_sort<T: PartialOrd + Debug> ( mut v: Vec<T>) -> Vec<T> {

    if v.len() <= 1 {
        return v;
    }
    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    // bring them together

    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();

    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => {
                match b_peek {
                    Some (ref b_val)  => {
                        if b_val < a_val {
                            res.push(b_peek.take().unwrap());
                            b_peek = b_it.next();
                        }else {
                            res.push(a_peek.take().unwrap());
                            a_peek = a_it.next();
                        }
                    }
                    None => {
                        res.push(a_peek.take().unwrap());
                        res.extend(a_it);
                        return res;
                    }
                }
            }
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res
            }
        }
    }



}

pub fn pivot<T: PartialOrd>(v: &mut[T]) -> usize {
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p+1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}


pub fn quick_sort<T: PartialOrd + Debug + Send>(v: &mut[T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);

    let (a, b) = v.split_at_mut(p);

    quick_sort(a);
    quick_sort(&mut b[1..]);
}

struct RawSend<T>(*mut [T]);

unsafe impl<T> Send for RawSend<T> {}

pub fn threaded_quick_sort<T:'static + PartialOrd + Debug + Send>(v: &mut[T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);

    let (a, b) = v.split_at_mut(p);

    let raw_a: *mut [T] = a as *mut [T];
    let raw_s = RawSend(raw_a);

    unsafe {
        let handle = std::thread::spawn(move || {
            let raw_s =raw_s;
            threaded_quick_sort(&mut *raw_s.0);
        });
    
        threaded_quick_sort(&mut b[1..]);
    
        handle.join().ok();
    }
    

}


pub fn quick_sort_rayon<T: PartialOrd + Debug + Send>(v: &mut[T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);

    let (a, b) = v.split_at_mut(p);

    rayon::join(||quick_sort_rayon(a), || quick_sort_rayon(&mut b[1..]));

}

 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![1,4,2,6,3,0];
        bubble_sort(&mut v);
        assert_eq!(v, vec![0,1,2,3,4,6]);
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![1,4,2,6,3];
        let sorted = merge_sort(v);
        dbg!(&sorted);
        assert_eq!(sorted, vec![1,2,3,4,6]);
    }

    #[test]
    fn test_pivot() {
        let mut v = vec![4,6,1,8,11,13,3];
        let p = pivot(&mut v);

        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![3,2,5,1,6,2];
        quick_sort(&mut v);
        assert_eq!(v, vec![1,2,2,3,5,6]);
    }

    #[test]
    fn test_threaded_quick_sort() {
        let mut v = vec![3,2,5,1,6,2];
        threaded_quick_sort(&mut v);
        assert_eq!(v, vec![1,2,2,3,5,6]);
    }

    #[test]
    fn test_quick_sort_rayon() {
        let mut v = vec![3,2,5,1,6,2];
        quick_sort_rayon(&mut v);
        assert_eq!(v, vec![1,2,2,3,5,6]);
    }

    
}
