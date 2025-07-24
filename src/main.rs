fn main() {
<<<<<<< HEAD
    let tup = (12,13.1,true);
    println!("value is {}",tup.2);
=======
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("length is  {len}");
>>>>>>> 881be10356de2794edeceac1c11557a8ee414830
}


fn calculate_length(s: &String) -> usize {
    s.len()
}