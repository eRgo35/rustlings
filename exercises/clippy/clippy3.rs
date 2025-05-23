// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = Default::default();
    if my_option.is_none() {
        my_option.unwrap_or_default();
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = vec![];
    vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let value_b = 66;
    // Let's swap these two!
    let _ = std::mem::replace(&mut value_a, value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
