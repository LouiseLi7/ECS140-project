use custom_error::custom_error;
use std::env;

struct CStream {
    filename: String,
    char_pos: usize,
    contents: String,
}

impl CStream {
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
        return TokenType::Invalid;
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
    if next_char =='('|| next_char ==')'|| next_char =='{'|| next_char =='}'|| next_char ==';'{
        token_length = 1;
    }
    else{
        while next_char != ' ' || next_char !='\n'||next_char =='('|| next_char ==')'|| next_char =='{'|| next_char =='}'|| next_char ==';'{
            token_length = token_length+1;
            next_char = file.get_next_char().unwrap();
        }
    }// continue adding up(the same token) if not space or new line or Operator without spaces 
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
        if next_char =='('|| next_char ==')'|| next_char =='{'|| next_char =='}'|| next_char ==';'{
            token_length = 1;
        }
        else{
            while next_char != ' ' || next_char !='\n'||next_char =='('|| next_char ==')'|| next_char =='{'|| next_char =='}'|| next_char ==';'{
                token_length = token_length+1;
                next_char = file.get_next_char().unwrap();
            }
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




/* Stage 3 */
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

custom_error!{MyError
    SyntaxError{line_num: i32, char_pos: i32, syntax: String} = 
    "Error at Line {line_num} Character {char_pos}. The syntax should be: {syntax}."
}

struct Parser {
    all_tokens: Vec<Token>,
    curr_token_index: usize
}

impl Parser {
    pub fn new(all_tokens: Vec<Token>) -> Self {
        Self {
            all_tokens: all_tokens,
            curr_token_index: 0
        }
        
    }

    fn get_next_token(&mut self) -> Option<&Token> {
        self.curr_token_index += 1;
        return self.all_tokens.get(self.curr_token_index - 1);
    }

    
    fn fun_Program(&mut self) -> Result<(), MyError> {
        let syntax = String::from("Program := {Declaration} MainDeclaration
        {FunctionDefinition}");
        match self.get_next_token() {
            None => {
                Err(MyError::SyntaxError{line_num: 0, char_pos: 0, syntax});
            },
            Some(x) => {
                let mut curr_lexeme = x;
                let line_num = curr_lexeme.line_num;
                let char_pos = curr_lexeme.char_pos;
        
                let declaration_first_set= vec_of_strings!["unsigned", "char", "short", "int", "long",
                 "float", "double"];
                while declaration_first_set.contains(&curr_lexeme.text) {
                    match self.fun_Declaration() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }
                    curr_lexeme = self.get_next_token().unwrap();
                }
                if curr_lexeme.text == "void" {
                    match self.fun_MainDeclaration() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }
                } else {
                    Err(MyError::SyntaxError{line_num, char_pos, syntax});
                }
                match self.get_next_token() {
                    None => println!("Input program is syntactacilly correct."),
                    Some(x) => curr_lexeme = x
                }
                while declaration_first_set.contains(&curr_lexeme.text) {
                    match self.fun_FunctionDefinition() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }
                    match self.get_next_token() {
                        None => println!("Input program is syntactacilly correct."),
                        Some(x) => curr_lexeme = x
                    }
                }
            }
        }
        Ok(())

    }

    // fn fun_Declaration(&mut self) -> Result<(), MyError> {

    // }

}

fn main() {
    // Run program with "cargo run examples/example1.x"
    let args: Vec<String> = env::args().collect(); 
    let filename = args[1].clone();
    let my_cstream = CStream::new(&filename);
    println!("{:?}", my_cstream.get_content());
    
    let my_scanner = Scanner::new(my_cstream);
    //let all_tokens: Vec<Token> = my_scanner.


}
