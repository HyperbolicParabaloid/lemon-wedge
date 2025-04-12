use lemon_wedge;

fn main() {
    match lemon_wedge::run() {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Trying initalizing Window for program: \n{}", err)
        }
    }

    // const CAPACITY: usize = 100_000;
    // let mut active_chars = Vec::from([100; 100]);
    // let new_active_slice = Vec::from([-100; CAPACITY]);

    // let now = std::time::Instant::now();
    // copy_from_slice_method(&mut active_chars, &new_active_slice, CAPACITY);
    // // resize_with_method(&mut active_chars, &new_active_slice, CAPACITY);
    // println!("[{}]", now.elapsed().as_micros());
}

// fn copy_from_slice_method(active_chars: &mut Vec<i32>, new_active_slice: &[i32], capacity: usize) {
//     active_chars.resize(capacity, 0);
//     active_chars.copy_from_slice(new_active_slice);
// }

// fn resize_with_method(active_chars: &mut Vec<i32>, new_active_slice: &[i32], capacity: usize) {
//     let mut new_index: usize = 0;
//     active_chars.resize_with(
//         capacity,
//         || {
//             let item = new_active_slice[new_index];
//             new_index += 1;
//             item
//     });
// }