fn main() {

    println!("Start code !!");

    let data = String::from("hello");

    let data_length = calculate_length(&data);

    for_loop_ex(data_length);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn for_loop_ex(i: usize){
    println!("input data : {}", i);

    for index in 0..i {
        println!("{}", index);
    }
}