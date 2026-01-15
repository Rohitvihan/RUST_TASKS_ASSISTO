fn mode(arr: &Vec<f64>) -> f64 {
    let mut max_count = 0;
    let mut result = arr[0];

    for i in 0..arr.len() {
        let mut count = 0;
        for j in 0..arr.len() {
            if arr[i] == arr[j] {
                count += 1;
            }
        }

        if count > max_count {
            max_count = count;
            result = arr[i];
        }
    }
    result
}

fn main() {
    let data: Vec<f64> = vec![1.0, 2.0, 2.0, 3.0, 4.0, 2.0, 5.0];

    let result = mode(&data);

    println!("Mode = {}", result);
}
