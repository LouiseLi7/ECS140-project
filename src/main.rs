struct Cstream {
    filename: String,
    char_pos: usize,
    contents: String,
}

impl Cstream {
    pub fn new(filename: &String) -> Self {
      Self {
        filename: filename.to_string(),
        char_pos: 0,
        contents: std::fs::read_to_string(filename).expect("no such file"),
      }
    }

    pub fn get_next_char(&mut self) -> Option<char> {
        self.char_pos += 1;
        return self.contents.chars().nth(self.char_pos - 1);
        // let lines: Vec<&str> = self.contents.split("\n").collect();
        // if self.char_pos + 1 < lines[self.line_num + 1].len() {
        //   self.char_pos += 1;
        //   return lines[self.line_num + 1].chars().nth(self.char_pos);
        // } else {// When the next char is at next line
        //   self.line_num += 1;
        //   self.char_pos = 0;
        //   return lines[self.line_num + 1].chars().nth(self.char_pos);
        // }
    }

    pub fn get_content(&self) -> &String {
        return &self.contents;
    }
}

fn main() {
    let mut f = Cstream::new(&"test.txt".to_string());
    // println!("{:?}", f.get_content());
    println!("{:?}", f.get_next_char());
}
