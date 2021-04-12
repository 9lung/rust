use std::{
    alloc::*,
    slice,
};

struct Vec {
    data : Box<[isize]>,
    length : usize,
    capacity : usize,
}

fn vec_new() -> Vec {
    Vec { 
        data: Box::new([-1]), 
        length: 0, 
        capacity: 1, 
    }
}

fn vec_push(vec: &mut Vec , n: isize) {
    if vec.length == vec.capacity {
        let new_capacity = vec.capacity * 2;
        let mut new_data = unsafe {
            let ptr = alloc(Layout::array::<isize>(new_capacity).unwrap())
                as *mut isize;
            Box::from_raw(slice::from_raw_parts_mut(ptr, new_capacity))
        };

        for i in 0..vec.length {
            new_data[i] = vec.data[i];
        }

        vec.data = new_data;
        vec.capacity = new_capacity;
    }

    vec.data[vec.length] = n;
    vec.length += 1;
}

fn main() {
    let mut vec : Vec = vec_new();
    vec_push(&mut vec, 107);
    vec_push(&mut vec, 100);

    let n: &isize = &vec.data[0];
}
