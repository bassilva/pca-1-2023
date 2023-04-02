fn main() {
    let mut a = [7, 6, 5, 4, 3, 2, 1, 0];

    // Start from one because arrays are 0-indexed in Rust
    for j in 1..a.len() {
        // key is a number that we want to sort
        let key = a[j];
        let mut i = j - 1;
        while a[i] > key {
            a[i + 1] = a[i];
            if i == 0 {
                break;
            }
            i = i - 1;
        }
        a[i] = key;
    }

    println!("a: {:?}", a);
}
