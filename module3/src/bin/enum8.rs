#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
  
 
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = vec![
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }


  let r:i128 = 8 ; // 
  let w:u8 = 3 ; // 0 to 2^8  
  
  
  
}
