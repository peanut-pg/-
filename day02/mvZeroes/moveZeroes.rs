use std::collections::VecDeque;

fn move_zeroes(nums: &mut Vec<i32>) {
    let mut buf = VecDeque::new();
    let mut i = 0;
    for n in nums.to_owned() {
        if n == 0 {
            buf.push_back(i);
        } else {
            if buf.len() > 0 {
               if let Some(t) = buf.pop_front() {
                   nums.swap(t, i);
                   buf.push_back(i);
               } 
            }
        }
        i += 1;
    }
}

fn main() {
    let mut aa = vec![0,1,0,3,12];
    move_zeroes(&mut aa);
    println!("{:?}", aa);
}