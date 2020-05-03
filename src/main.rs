extern crate rand;


use std::time::{Instant, Duration, SystemTime, UNIX_EPOCH};
use rand::Rng;
use rand::prelude::ThreadRng;
use rayon::ThreadPoolBuilder;


fn merge(array1: &Vec<i32>, array2: &Vec<i32>) -> Vec<i32> {
    let l1: usize = array1.len();
    let l2: usize = array2.len();
    let n: usize = l1 + l2;
    let mut res_arr: Vec<i32> = Vec::with_capacity(n);
    let mut i1: usize = 0;
    let mut i2: usize = 0;
    for _ in 0 .. n {
        if i1 < l1 && i2 < l2 {
            let a1 = array1[i1];
            let a2 = array2[i2];
            if a1 > a2 {
                res_arr.push(a2);
                i2 += 1;
            } else {
                res_arr.push(a1);
                i1 += 1;
            }
        }
        else {
            if i1 >= l1 && i2 >= l2 {
                break;
            }
            else if i1 >= l1 {
                res_arr.push(array2[i2]);
                i2 += 1;
            }
            else {
                res_arr.push(array1[i1]);
                i1 += 1;
            }
        }
    }
    res_arr
}

fn seq_merge_sort(array: Vec<i32>) -> Vec<i32> {
    if array.len() <= 1 {
        array
    } else {
        let mid: usize = array.len() / 2;
        let left: Vec<i32> = array.iter().cloned().take(mid).collect();
        let right: Vec<i32> = array.iter().cloned().skip(mid).collect();
        let arr1: Vec<i32> = seq_merge_sort(left);
        let arr2: Vec<i32> = seq_merge_sort(right);
        let res: Vec<i32> = merge(&arr1, &arr2);
        res
    }
}


fn par_merge_sort(array: Vec<i32>) -> Vec<i32> {
    if array.len() <= 1 {
        array
    } else {
        let mid: usize = array.len() / 2;
        let left: Vec<i32> = array.iter().cloned().take(mid).collect();
        let right: Vec<i32> = array.iter().cloned().skip(mid).collect();
        let (arr1, arr2) = rayon::join(
            || par_merge_sort(left),
            || par_merge_sort(right));
        let res= merge(&arr1, &arr2);
        res
    }
}

fn test(times: i32, len: i32, psnum: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng: ThreadRng = rand::thread_rng();
    let v: Vec<i32> = (0..len).map(|_| {
        rng.gen_range(1, 101)
    }).collect();
    let mut time_acc = Vec::<u128>::with_capacity( times as usize);
    for _ in 0..times {
        let start: Instant = Instant::now();
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("Error");
        seq_merge_sort(v.clone());
        let end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("Error");
        let duration: Duration = start.elapsed();
        let whole_time = end_time.as_millis() - start_time.as_millis();
        time_acc.push(whole_time);
    }
    let seq_res = time_acc.iter().fold(0, |x, &y| x + y) / times as u128;
    println!("{:?}", time_acc);
    println!("[Seq] Taken time: {} ms", seq_res);

    time_acc.clear();
    let pool = ThreadPoolBuilder::new().num_threads(psnum).build()?;
    for _ in 0..times {
        let start: Instant = Instant::now();
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("Error");
        pool.install(|| par_merge_sort(v.clone()));
        let end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("Error");
        let duration: Duration = start.elapsed();
        let whole_time = end_time.as_millis() - start_time.as_millis();
        time_acc.push(whole_time);
    }
    let par_res = time_acc.iter().fold(0, |x, &y| x + y) / times as u128;
    println!("{:?}", time_acc);
    println!("[Par] Taken time: {} ms", par_res);
    Ok(())
}

fn main() {
    let len: i32 = 1000000;
    let thread_number = 8;
    test(10, len, thread_number);
}
