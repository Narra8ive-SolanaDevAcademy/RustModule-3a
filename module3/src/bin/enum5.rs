// Since there is no null in Rust, we have to use enum Option<T> to deal with the cases when the value is absent.


// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let __ = six {
        println!("{}", n);

        println!("Success!");
    } 
        
    panic!("NEVER LET THIS RUN！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        __ => None,
        __ => Some(i + 1),
    }
}
