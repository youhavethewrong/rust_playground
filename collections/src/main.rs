fn main() {
    println!("Main screen turn on.");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = format!("{}-{}", s1, s2);

    println!("{}", s3);
}
