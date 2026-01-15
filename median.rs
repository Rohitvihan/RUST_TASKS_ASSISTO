fn median(arr: &Vec<f64>) -> f64 {
    let mut v = arr.clone();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = v.len();
    if n % 2 == 0 {
        (v[n/2 - 1] + v[n/2]) / 2.0
    } else {
        v[n/2]
    }
}

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    println!("Median = {}", median(&data));
}

