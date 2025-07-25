struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let s1 = String::from("hel lo");
    let size = first_word(&s1);
    println!("{size}");

    let s = "world";


    let user = User {
        active: true,
        username: String::from("小王"),
        email: String::from("value"),
        sign_in_count: 1,
    };
    println!("{0}", user.username);

}


fn first_word(s: &String)-> &str {
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
    }
    &s[..]
}