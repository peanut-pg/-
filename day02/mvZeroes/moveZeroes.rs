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

fn move_zeroes2(nums: &mut Vec<i32>) {
    let mut a = VecDeque::new();
    let mut j = 0;
    for n in nums.to_owned() {
        if n == 0 {
            a.push_back(i);
        } else {
            if a.len() > 0 {
                if let Some(t) = a.pop_front() {
                    nums.swap(t, i);
                    a.push_back(i);
                }
            }
        }
        j+=1;
    }
}


// 这个的思路和go 中的那个代码的思路是类似的
// ```
//func moveZeroes5(nums []int) {
//   	j := 0
//   	for i:=0;i<len(nums);i++ {
//   		if nums[i] ==0 {
//   			continue
//   		}
//   		nums[i], nums[j] = nums[j], nums[i]
//   		j++
//   	}
//   }
// ```
fn move_zeroes3(nums: &mut Vec<i32>) {
    let mut j = 0;
    for i in 0..nums.len() {
        if nums[i] == 0 {
            continue
        }
        let tmp = nums[i];
        nums[i] = nums[j];
        nums[j] = tmp;

        j+=1;
    }
}

fn main() {
    let mut aa = vec![0,1,0,3,12];
    move_zeroes3(&mut aa);
    println!("{:?}", aa);
}