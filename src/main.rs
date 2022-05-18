#[derive(Clone)]
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
        return self.get_content().chars().nth(self.char_pos - 1);
    }

    pub fn get_content(&self) -> &String {
        return &self.contents;
    }

    // pub fn clone(&self)->Cstream {
    //     let mut file = Cstream::new(&self.filename);
    //     file.char_pos = self.char_pos;
    //     file.contents = self.get_content().to_string();
    //     return file;
    // }
}

#[derive(Copy, Clone)]
enum TokenType {
    IntConstant,
    FloatConstant,
    Keyword,
    Operator,
    Identifier,
    Invalid
}

impl TokenType {
    fn as_str(&self) -> &'static str {
        match &self {
            TokenType::IntConstant => "IntConstant",
            TokenType::FloatConstant => "FloatConstant",
            TokenType::Keyword => "Keyword",
            TokenType::Operator => "Operator",
            TokenType::Identifier => "Identifier",
            TokenType::Invalid => "Invalid",
        }
    }
}// transform the content of TokenType as string 

#[derive(Clone)]
struct Token {
    text: String,
    token_type: TokenType,
    line_num: i32,
    char_pos: i32,
    token_length: i32,
    pos_in_file:i32
}

impl Token {
    fn new(text: String, token_type: TokenType) -> Self {
        Self {
          text: text,
          token_type: token_type,
          line_num: 0,
          char_pos: 0,
          token_length: 0, // another feature
          pos_in_file: 0 //another feature
        }
    }

    fn return_token_type(&self)->String{
        return self.token_type.as_str().to_string()
    }// return the token type value as string 
}

// struct Scanner {
//     fn new()
// }

fn check_token_type_char(text:char)->TokenType{
    if text =='('||text ==')'||text =='{'||text =='}'||text ==','||text =='}'||text =='='||text =='<'||text =='>'||text =='+'||text =='-'||text =='*'||text =='/'||text ==';'{
        return TokenType::Operator;
    }
    else if text.is_numeric(){
        return TokenType::IntConstant;
    }
    else if text.is_alphabetic(){
        return TokenType::Identifier;
    }
    else{
        return TokenType::Invalid;
    }
}

fn check_token_type_string(text:String)->TokenType{
    let text = text.to_string();
    if text.chars().nth(0).unwrap().is_numeric(){
        let mut number = true;
        let mut float = false;
        for c in text.chars(){
            if !c.is_numeric(){
                if c=='.' {
                    float = true; // case for float
                    if float==false{
                        number = false; // too many .
                    }
                }
                else{
                    number = false;
                }
            }
        }
        if number==true && float == true{
            return TokenType::FloatConstant;
        }
        else if number==true && float == false{
            return TokenType::IntConstant;
        }
        else{
            return TokenType::Invalid;
        }
    }
    else if text.chars().nth(0).unwrap().is_alphabetic(){
        if text=="unsigned"||text=="char"||text=="short"||text=="int"||text=="long"||text=="float"||text=="double"||text=="while"||text=="if"||text=="return"||text=="void"||text=="main" {
            return TokenType::Keyword;
        }
        let mut letters = true;
        for c in text.chars(){
            if !c.is_alphabetic(){
                letters = false;
            }
        }
        if letters==true{
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

fn Scanner(file: &mut Cstream)-> Vec<Token>{
    let mut length = 0;
    let file_clone = file.clone();
    let f = file_clone.contents;
    println!("{:?}",f);
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
    println!("{},{}",next_char,file.char_pos);
    if next_char =='(' && next_char ==')'&& next_char =='{'&& next_char =='}'&& next_char ==';'{
        token_length = 1;
    }
    else{
        while next_char != ' ' && next_char !='\n' && next_char !='('&& next_char !=')'&& next_char !='{'&& next_char !='}'&& next_char !=';'{
            token_length = token_length+1;
            next_char = file.get_next_char().unwrap();
        }
    }// continue adding up(the same token) if not space or new line or Operator without spaces 
    let token_type;
    if token_length == 1 {
        token_type = check_token_type_char(f.chars().nth(pos_start).unwrap());
    }
    else {
        token_type = check_token_type_string(f[pos_start..pos_start+token_length].to_string());
    }
    let mut current_token = Token::new(f[pos_start..pos_start+token_length].to_string(),token_type);
    current_token.line_num = line;
    current_token.char_pos = length;
    current_token.token_length = token_length as i32;
    current_token.pos_in_file = pos_start as i32;

    let mut token_vector = Vec::new();
    token_vector.push(current_token.clone());
    while file.char_pos<f.len(){
        current_token = get_next_token(file,current_token);
        println!("{:?},{:?}",current_token.text,current_token.return_token_type());
        token_vector.push(current_token.clone());
    }
    return token_vector;
}

fn get_next_token(file: &mut Cstream,current_t:Token)->Token{
    let file_clone = file.clone();
    let f = file_clone.get_content();
    let mut next_char = f.chars().nth(current_t.pos_in_file as usize +current_t.token_length as usize).unwrap();
    while next_char == ' '{
        next_char = file.get_next_char().unwrap();
    }
    if next_char =='\n' {
        let mut new_token = Token::new('\n'.to_string(),TokenType::Invalid);
        new_token.line_num = current_t.line_num+1;
        new_token.char_pos = 0;
        new_token.pos_in_file = file.char_pos as i32;
        next_char = file.get_next_char().unwrap();
        let new_token = get_next_token(file,new_token); 
        return new_token;
    }
    else {
        let pos_start = file.char_pos-1;
        let mut token_length = 0;
        if next_char =='('|| next_char ==')'|| next_char =='{'|| next_char =='}' || next_char ==';'{
            token_length = 1;
            if file.char_pos < f.len(){
                next_char = file.get_next_char().unwrap();
            }
        }
        else{
            while next_char != ' ' && next_char !='\n'&&next_char !='('&& next_char !=')'&& next_char !='{'&& next_char !='}'&& next_char !=';'{
                token_length = token_length+1;
                next_char = file.get_next_char().unwrap();
            }
        }
        let pos_end = pos_start + token_length;
        let token_type;
        if token_length == 1 {
            token_type = check_token_type_char(f.chars().nth(pos_start).unwrap());
        }
        else {
            token_type = check_token_type_string(f[pos_start..pos_end].to_string());
        }

        let mut new_token = Token::new(f[pos_start..pos_end].to_string(),token_type);
        new_token.char_pos = pos_start as i32 -current_t.pos_in_file;
        new_token.line_num = current_t.line_num;
        new_token.token_length = token_length as i32;
        new_token.pos_in_file = pos_start as i32;
        return new_token;
    }
}


fn main() {
    let mut f = Cstream::new(&"./examples/example1.x".to_string());
    let mut content = f.get_content();
    let vector = Scanner(&mut f);
    //println!("{:?}", f.get_content());
    //println!("{:?}", vector.first().unwrap().return_token_type());
}
