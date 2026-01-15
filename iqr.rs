fn quartiles(arr: &Vec<f64>) -> (f64, f64, f64) 
{
    let mut v = arr.clone();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = v.len();

    let q2 = if n % 2 == 0 {
        (v[n/2 - 1] + v[n/2]) / 2.0
    } else {
        v[n/2]
    };

    let lower = &v[..n/2];
    let upper = if n % 2 == 0 {
        &v[n/2..]
    } else {
        &v[n/2 + 1..]
    };

    let q1 = median(lower);
    let q3 = median(upper);

    (q1, q2, q3)
}

fn median(slice: &[f64]) -> f64 {
    let n = slice.len();
    if n % 2 == 0 {
        (slice[n/2 - 1] + slice[n/2]) / 2.0
    } else {
        slice[n/2]
    }
}

fn iqr(arr: &Vec<f64>) -> f64 {
    let (q1, _, q3) = quartiles(arr);
    q3 - q1
}

fn main() {
    let data: Vec<f64> = vec![5.0, 7.0, 10.0, 12.0, 15.0, 18.0, 20.0, 25.0];

    let result = iqr(&data);

    println!("Interquartile Range (IQR) = {}", result);
}
