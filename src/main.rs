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
    }

    pub fn get_content(&self) -> &String {
        return &self.contents;
    }
}

enum TokenType {
    IntConstant,
    FloatConstant,
    Keyword,
    Operator,
    Identifier,
    Invalid
}

struct Token {
    text: String,
    token_type: TokenType,
    line_num: i32,
    char_pos: i32
}

impl Token {
    fn new(text: String, token_type: TokenType) -> Self {
        Self {
          text: text,
          token_type: token_type,
          line_num: 0,
          char_pos: 0
        }
      }
}

// struct Scanner {
//     fn new()
// }


fn main() {
    let mut f = Cstream::new(&"test.txt".to_string());
    println!("{:?}", f.get_content());
    println!("{:?}", f.get_next_char());
}
