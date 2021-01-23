use std::sync::Arc;
use rayon::prelude::*;

const THRESHOLD: usize = 50;

fn process_by_chunks1(
    data: &[i32],
    f: Arc<dyn Fn(&i32) -> i32 + Send + Sync>
) -> Vec<i32> {
    if data.len() > THRESHOLD {
        data
            .par_iter()
            .map(move |a| {
                let f_cloned = Arc::clone(&f);
                f_cloned(a)
            })
            .collect()
    } else {
        data
            .iter()
            .cloned()
            .collect()
    }
}


fn main() {
    let f1: Arc<dyn Fn(&i32) -> i32 + Send + Sync> = Arc::new(move |a| {
        a * 100
    });
    let len1 = (THRESHOLD * 2) as i32;
    let data1 = (0..len1).collect::<Vec<i32>>();
    let res1 = process_by_chunks1(&data1, f1);
    println!("greater than the value of THRESHOLD (*100): \r\n{:?}", res1);

    println!("\r\n");

    let f2: Arc<dyn Fn(&i32) -> i32 + Send + Sync> = Arc::new(move |_| {
        println!("this won't be called");
        0
    });
    let len2 = (THRESHOLD - 5) as i32;
    let data2 = (0..len2).collect::<Vec<i32>>();
    let res2 = process_by_chunks1(&data2, f2);
    println!("less than the value of THRESHOLD: \r\n{:?}", res2);
}
