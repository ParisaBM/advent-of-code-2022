use std::fs;
fn main() {
    let data = fs::read("input").unwrap();
    for (part, window_size) in [(1, 4), (2, 14)] {
        'sliding_window: for i in 0..data.len()-window_size+1 {
            for j in 0..window_size-1 {
                for k in j+1..window_size {
                    if data[i+j] == data[i+k] {
                        continue 'sliding_window;
                    }
                }
            }
            println!("Part {}: {}", part, i+window_size);
            break;
        }
    }
}
