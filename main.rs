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
          char_pos: 0,
          token_length = 0, // another feature
          pos_in_file = 0 //another feature
        }
      }
}

// struct Scanner {
//     fn new()
// }

fn check_token_type_char(text:char)->TokenType{
    if text =='('||text ==')'||text =='{'||text =='}'||text ==','||text =='}'||text =='='||text =='<'||text =='>'||text =='+'||text =='-'||text =='*'||text =='/'||text ==';'{
        return TokenType::Operator;
    }
    else{
        return Token::Invalid;
    }
}

fn check_token_type_string(text:String)->TokenType{
    let mut text = text.to_string();
    if text.chars().nth(0).is_numeric(){
        Bool number = True;
        Bool float = False;
        for c in text.chars(){
            if !c.is_numeric(){
                if c=='.' {
                    float = True; // case for float
                    if float==False{
                        number = False; // too many .
                    }
                }
                else{
                    number = False;
                }
            }
        }
        if number==True && float == True{
            return TokenType::FloatConstant;
        }
        else if number==True && float == False{
            return TokenType::IntConstant;
        }
        else{
            return TokenType::Invalid;
        }
    }
    else if text.chars().nth(0).is_alphabetic(){
        if text=="unsigned"||text=="char"||text=="short"||text=="int"||text=="long"||text=="float"||text=="double"||text=="while"||text=="if"||text=="return"||text=="void"||text=="main" {
            return TokenType::Keyword;
        }
        Bool letters = True;
        for c in text.chars(){
            if !c.is_alphabetic(){
                letters = False;
            }
        }
        if letters==True{
            return TokenType::Identifier;
        }
        else{
            return TokenType::Invalid;
        }
    }
    else if  text=="=="||text=="<="||text==">="||text=="!=" {
        return TokenType::Operator;
    }
    else{
        return TokenType::Invalid;
    }

}

fn Scanner(file: CStream)-> Vec<Token>{
    let mut length = 0;
    let f = file.get_content();
    let mut next_char = file.get_next_char().unwrap();
    let mut line = 0;
    while next_char == ' '||next_char =='\n' {
        if next_char=='\n' {
            line = line+1; //line ++
            length = 0;
        }
        else {
            length = length+1; //space ++
        }
        next_char = file.get_next_char().unwrap(); // line and space takes up one index in f
    }
    let pos_start = file.char_pos-1; //because the char_pos has +1 and the next_char is the char in the pos of char_pos -1 
    let mut token_length = 0;
    while next_char != ' '||next_char !='\n' {
        token_length = token_length+1;
        next_char = file.get_next_char().unwrap();
    }// continue adding up(the same token) if not space or new line
    if token_length == 1 {
        let token_type = check_token_type_char(f[pos_start,pos_start+1]);
    }
    else {
        let token_type = check_token_type_string(f[pos_start,pos_start+token_length]);
    }
    let mut current_token = Token::new(f[pos_start..pos_start+token_length],token_type);
    current_token.line_num = line;
    current_token.char_pos = length;
    current_token.token_length = token_length;
    current_token.pos_in_file = pos_start;

    let mut token_vector = Vec::new();
    while file.char_pos<=f.len(){
        new_token = get_next_token(file,current_token);
        token_vector.push(new_token);
    }
    return token_vector();
}

fn get_next_token(file: CStream,current_t:Token)->Token{
    let f = file.get_content();
    let mut next_char = file.get_next_char().unwrap();
    while next_char == ' '{
        next_char = file.get_next_char().unwrap();
    }
    if next_char =='\n' {
        let mut new_token = Token::new("\n",TokenType::Invalid);
        new_token.line_num = current_t.line_num+1;
        new_token.char_pos = 0;
        new_token.pos_in_file = file.char_pos-1;
        next_char = file.get_next_char().unwrap();
        get_next_token(file,new_token); 
        
    }
    else {
        let pos_start = file.char_pos-1;
        let mut token_length = 0;
        while next_char != ' ' || next_char !='\n'{
            token_length = token_length+1;
            next_char = file.get_next_char().unwrap();
        }
        let pos_end = pos_start + token_length;
        if token_length == 1 {
            let token_type = check_token_type_char(f[pos_start,pos_end]);
        }
        else {
            let token_type = check_token_type_string(f[pos_start,pos_end]);
        }
        let mut new_token = Token::new(f[pos_start..pos_end],token_type);
        new_token.char_pos = pos_start-current_t.pos_in_file;
        new_token.line_pos = current_t.line_num;
        new_token.token_length = token_length;
        new_token.pos_in_file = pos_start;
        return new_token;
    }
}


fn main() {
    let mut f = Cstream::new(&"./examples/example1.x".to_string());
    println!("{:?}", f.get_content());
    println!("{:?}", f.get_next_char());
}
