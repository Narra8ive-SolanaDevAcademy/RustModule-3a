enum Message {
    ChangeColor(i32,i32,i32),
    Echo(String),
    Move(Point),
    Quit
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true; // quit false 
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
       match message {
    //ChangeColor(i32,i32,i32),
    // Echo(String),
    // Move(Point),
    // Quit
         
         Message::ChangeColor(i1, i2,i3 ) => {

           //
           let color =(i1,i2,i3);

           Self::change_color(&mut self, color)
         },
         Message::Echo(s) =>{
           //
         },
         Message::Move(p) => {
           
         },
         Message::Quit =>{
           
         }
         
       }
      
    }
}


fn main() {
  /// call the process function 

  // user input 
  // "change color
  // "echo" -> ""
  // "move" 
} 
