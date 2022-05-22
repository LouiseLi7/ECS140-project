use custom_error::custom_error;
use std::env;

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

#[derive(Copy, Clone, Debug)]
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

#[derive(Clone, Debug)]
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

    fn get_next_token(&mut self) -> Option<Token> {
        if self.curr_token_index < self.all_tokens.len() {
            self.curr_token_index += 1;
            Some(self.all_tokens[self.curr_token_index - 1].clone())
        } else {
            None
        }
    }

    fn get_curr_token(&self) -> Token {
        return self.all_tokens[self.curr_token_index - 1].clone();
    }

    fn peek_next_token(&self) -> Option<Token> {
        if self.curr_token_index < self.all_tokens.len() + 1 {
            Some(self.all_tokens[self.curr_token_index].clone())
        } else {
            None
        }
    }

    fn peek_nextnext_token(&self) -> Option<Token> {
        if self.curr_token_index < self.all_tokens.len() + 2 {
            Some(self.all_tokens[self.curr_token_index + 1].clone())
        } else {
            None
        }
    }
    
    fn fun_Program(&mut self) -> Result<(), MyError> {
        let syntax = String::from("Program := {Declaration} MainDeclaration
        {FunctionDefinition}");

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: 0, char_pos: 0, syntax}),
            Some(x) => {
                let mut curr_lexeme = x;
                let first_set_Declaration = vec_of_strings!["unsigned", "char", "short",
                 "int", "long", "float", "double"];
                
                while first_set_Declaration.contains(&curr_lexeme.text) {
                    match self.fun_Declaration() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }
                    curr_lexeme = self.get_curr_token();

                    match self.get_next_token() {
                        None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => curr_lexeme = x
                    }
                }

                if curr_lexeme.text == "void" {
                    match self.fun_MainDeclaration() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }
                } else {
                    return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                }

                match self.get_next_token() {
                    None => {
                        println!("Input program is syntactacilly correct.");
                        Ok(())
                    }
                    Some(x) => {
                        curr_lexeme = x;
                        while first_set_Declaration.contains(&curr_lexeme.text) {
                            match self.fun_FunctionDefinition() {
                                Ok(()) => (),
                                Err(e) => println!("{}", e)
                            }
                            match self.get_next_token() {
                                None => {
                                    println!("Input program is syntactacilly correct.");
                                    return Ok(())
                                },
                                Some(x) => curr_lexeme = x
                            }
                        }
                        Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                    }
                }
                
            }

        }
    }

    

    
    

    fn fun_Declaration(&mut self) -> Result<(), MyError> {
        let syntax = String::from("Declaration := DeclarationType (VariableDeclaration
             | FunctionDeclaration)");
        match self.fun_DeclarationType() {
            Ok(()) => (),
            Err(e) => println!("{}", e)
        }

        match self.get_next_token() {
            None => Ok(()),
            Some(x) => {
                let curr_lexeme = x;

                if curr_lexeme.text == "=" {
                    match self.fun_VariableDeclaration() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }
                    Ok(())
                } else if curr_lexeme.text == "(" {
                    match self.fun_FunctionDeclaration() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }
                    Ok(())
                } else {
                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                }
            }
        }
    }

    fn fun_MainDeclaration(&mut self) -> Result<(), MyError> {
        let syntax = String::from("MainDeclaration := void main ( ) Block");
        let functional_token = self.get_curr_token();

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: functional_token.line_num,
                 char_pos: functional_token.char_pos, syntax}),
            Some(x) => {
                let mut curr_lexeme = x;

                if curr_lexeme.text == "main" {
                    match self.get_next_token() {
                        None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => {
                            curr_lexeme = x;

                            if curr_lexeme.text == "(" {
                                match self.get_next_token() {
                                    None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                                    Some(x) => {
                                        curr_lexeme = x;

                                        if curr_lexeme.text == ")" {
                                            match self.get_next_token() {
                                                None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                                                Some(x) => {
                                                    curr_lexeme = x;

                                                    if curr_lexeme.text == "{" {
                                                        match self.fun_Block() {
                                                            Ok(()) => (),
                                                            Err(e) => println!("{}", e)
                                                        }
                                                        Ok(())
                                                    } else {
                                                        Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                                                    }
                                                }
                                            }
                                        } else {
                                            Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                                        }
                                    }
                                }
                            } else {
                                Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                            }
                        }
                    }
                } else {
                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                }
            }
        }
    }

    fn fun_FunctionDefinition(&mut self) -> Result<(), MyError> {
        let syntax = String::from("FunctionDefinition := DeclarationType ParameterBlock Block");
        match self.fun_DeclarationType() {
            Ok(()) => (),
            Err(e) => println!("{}", e)
        }
        let mut curr_lexeme = self.get_curr_token();

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num,
                 char_pos: curr_lexeme.char_pos, syntax}),
            Some(x) => {
                curr_lexeme = x;

                if curr_lexeme.text == "(" {
                    match self.fun_ParameterBlock() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }
                } else {
                    return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                }
                curr_lexeme = self.get_curr_token();
                match self.get_next_token() {
                    None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num,
                        char_pos: curr_lexeme.char_pos, syntax}),
                    Some(x) => {
                        let curr_lexeme = x;
                        
                        if curr_lexeme.text == "{" {
                            match self.fun_Block() {
                                Ok(()) => (),
                                Err(e) => println!("{}", e)
                            }
                            Ok(())
                        } else {
                            Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                        }
                    }
                }
            }
        }


    }

    fn fun_DeclarationType(&mut self) -> Result<(), MyError> {
        let syntax = String::from("DeclarationType := DataType Identifier");
        match self.fun_DataType() {
            Ok(()) => (),
            Err(e) => println!("{}", e)
        }
        let functional_token = self.get_curr_token();

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: functional_token.line_num,
                char_pos: functional_token.char_pos, syntax}),
            Some(x) => {
                let curr_lexeme = x;

                if matches!(curr_lexeme.token_type, TokenType::Identifier) {
                    Ok(())
                } else {
                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                }
            }
        }
    }

    fn fun_VariableDeclaration(&mut self) -> Result<(), MyError> {
        let syntax = String::from("VariableDeclaration := [= Constant] ;");
        let functional_token = self.get_curr_token();

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: functional_token.line_num,
                char_pos: functional_token.char_pos, syntax}),
            Some(x) => {
                
                match self.fun_Constant() {
                    Ok(()) => (),
                    Err(e) => println!("{}", e)
                }
                let mut curr_lexeme = self.get_curr_token();

                match self.get_next_token() {
                    None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                    Some(x) => {
                        curr_lexeme = x;

                        if curr_lexeme.text == ";" {
                            Ok(())
                        } else {
                            Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                        }
                    }
                }
            }
            
        }
    }

    fn fun_FunctionDeclaration(&mut self) -> Result<(), MyError> {
        let syntax = String::from("FunctionDeclaration := ParameterBlock ;");
        match self.fun_ParameterBlock() {
            Ok(()) => (),
            Err(e) => println!("{}", e)
        }
        let functional_token = self.get_curr_token();

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: functional_token.line_num,
                char_pos: functional_token.char_pos, syntax}),
            Some(x) => {
                let curr_lexeme = x;

                if curr_lexeme.text == ";" {
                    Ok(())
                } else {
                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                }
            }
        }
    }

    fn fun_Block(&mut self) -> Result<(), MyError> {
        let syntax = String::from("Block := { (Declaration) (Statement) (FunctionDefinition) }");
        let functional_token = self.get_curr_token();

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: functional_token.line_num,
                char_pos: functional_token.char_pos, syntax}),
            Some(x) => {
                let mut curr_lexeme = x;
                let nextnext_token;
                let first_set_Declaration = vec_of_strings!["unsigned", "char", "short",
                 "int", "long", "float", "double"];
                let first_set_Statement = vec_of_strings!["while", "if", "return",
                 "("];

                if curr_lexeme.text == "}" {
                    return Ok(())
                }

                match self.peek_nextnext_token() {
                    None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                    Some(x) => {
                        nextnext_token = x;
                    }
                }

                while first_set_Declaration.contains(&curr_lexeme.text) && nextnext_token.text != "{" {
                    match self.fun_Declaration() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }
                    curr_lexeme = self.get_curr_token();
                    match self.get_next_token() {
                        None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => curr_lexeme = x
                    }
                }

                if curr_lexeme.text == "}" {
                    return Ok(())
                }

                while matches!(curr_lexeme.token_type, TokenType::Identifier) || first_set_Statement.contains(&curr_lexeme.text) {
                    match self.fun_Statement() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }
                    curr_lexeme = self.get_curr_token();
                    match self.get_next_token() {
                        None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => curr_lexeme = x
                    }
                }

                if curr_lexeme.text == "}" {
                    return Ok(())
                }

                while first_set_Declaration.contains(&curr_lexeme.text) {
                    match self.fun_FunctionDefinition() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }
                    curr_lexeme = self.get_curr_token();
                    match self.get_next_token() {
                        None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => curr_lexeme = x
                    }
                }

                if curr_lexeme.text == "}" {
                    return Ok(())
                }

                Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
            }
        }
    }

    fn fun_ParameterBlock(&mut self) -> Result<(), MyError> {
        let syntax = String::from("ParameterBlock := ( [Parameter {, Parameter}] )");
        let functional_token = self.get_curr_token();

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: functional_token.line_num,
                char_pos: functional_token.char_pos, syntax}),
            Some(x) => {
                let mut curr_lexeme = x;
                let first_set_Declaration = vec_of_strings!["unsigned", "char", "short",
                 "int", "long", "float", "double"];

                if curr_lexeme.text == ")" {
                    return Ok(())
                }

                if first_set_Declaration.contains(&curr_lexeme.text) {
                    match self.fun_Parameter() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }
                    curr_lexeme = self.get_curr_token();

                    match self.get_next_token() {
                        None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => {
                            curr_lexeme = x;
                            while curr_lexeme.text == "," {
                                match self.get_next_token() {
                                    None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                                    Some(x) => {
                                        curr_lexeme = x;
                                        if first_set_Declaration.contains(&curr_lexeme.text) {
                                            match self.fun_Parameter() {
                                                Ok(()) => (),
                                                Err(e) => println!("{}", e)
                                            }
                                        } else {
                                            return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                                        }
                                    }
                                }
                                match self.get_next_token() {
                                    None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                                    Some(x) => curr_lexeme = x
                                }
                            }
                            match self.get_next_token() {
                                None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                                Some(x) => {
                                    curr_lexeme = x;
                                    if curr_lexeme.text == ")" {
                                        return Ok(())
                                    } else {
                                        return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                                    }
                                }
                            } 
                        }
                    }
                } else {
                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                }
                
            }
        }
        
    }

    fn fun_DataType(&mut self) -> Result<(), MyError> {
        let syntax = String::from("DataType := IntegerType | FloatType");
        let curr_lexeme = self.get_curr_token();
        let first_set_IntergerType = vec_of_strings!["unsigned", "char", "short",
        "int", "long"];
        let first_set_FloatType = vec_of_strings!["float", "double"];

        if first_set_IntergerType.contains(&curr_lexeme.text) {
            match self.fun_InterType() {
                Ok(()) => (),
                Err(e) => println!("{}", e)
            }
            Ok(())
        } else if first_set_FloatType.contains(&curr_lexeme.text) {
            match self.fun_FloatType() {
                Ok(()) => (),
                Err(e) => println!("{}", e)
            }
            Ok(())
        } else {
            Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
        }
    }

    fn fun_Constant(&self) -> Result<(), MyError> {
        let syntax = String::from("Constant := IntConstant | FloatConstant");
        let curr_lexeme = self.get_curr_token();

        if matches!(curr_lexeme.token_type, TokenType::IntConstant) ||
        matches!(curr_lexeme.token_type, TokenType::FloatConstant) {
            Ok(())
        } else {
            Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
        }
    }

    fn fun_Statement(&mut self) -> Result<(), MyError> {
        let syntax = String::from("Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)");
        let mut curr_lexeme = self.get_curr_token();
        let mut next_token;
        match self.peek_next_token() {
            None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num,
                char_pos: curr_lexeme.char_pos, syntax}),
            Some(x) => next_token = x
        }

        if matches!(curr_lexeme.token_type, TokenType::Identifier) && next_token.text == "=" {
            match self.fun_Assignment() {
                Ok(()) => (),
                Err(e) => println!("{}", e)
            }
            Ok(())
        } else if curr_lexeme.text == "while" {
            match self.fun_WhileLoop() {
                Ok(()) => (),
                Err(e) => println!("{}", e)
            }
            Ok(())
        } else if curr_lexeme.text == "if" {
            match self.fun_WhileLoop() {
                Ok(()) => (),
                Err(e) => println!("{}", e)
            }
            Ok(())
        } else if curr_lexeme.text == "return" {
            match self.fun_IfStatement() {
                Ok(()) => (),
                Err(e) => println!("{}", e)
            }
            Ok(())
        } else if curr_lexeme.text == "(" || matches!(curr_lexeme.token_type, TokenType::IntConstant) ||
        matches!(curr_lexeme.token_type, TokenType::FloatConstant) || 
        matches!(curr_lexeme.token_type, TokenType::Identifier){
            match self.fun_Expression() {
                Ok(()) => (),
                Err(e) => println!("{}", e)
            }
            curr_lexeme = self.get_curr_token();

            match self.get_next_token() {
                None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                Some(x) => {
                    curr_lexeme = x;
                    if curr_lexeme.text == ";" {
                        match self.get_next_token() {
                            None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                            Some(x) => {
                                curr_lexeme = x;
                                if curr_lexeme.text == ")" {
                                    Ok(())
                                } else {
                                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                                }
                            }
                        }
                    } else {
                        return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                    }
                }
            }
        } else {
            Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
        }
    }

    fn fun_Parameter(&mut self) -> Result<(), MyError> {
        let syntax = String::from("Parameter := DataType Identifier");
        match self.fun_DataType() {
            Ok(()) => (),
            Err(e) => println!("{}", e)
        }
        let functional_token = self.get_curr_token();

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: functional_token.line_num,
                char_pos: functional_token.char_pos, syntax}),
            Some(x) => {
                let curr_lexeme = x;

                if matches!(curr_lexeme.token_type, TokenType::Identifier) {
                    Ok(())
                } else {
                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                }
            }
        }
    }

    fn fun_InterType(&mut self) -> Result<(), MyError> {
        let syntax = String::from("IntegerType := [unsigned] ( char | short | int | long )");
        let mut curr_lexeme = self.get_curr_token();
        let first_set_IntergerType = vec_of_strings!["char", "short",
        "int", "long"];

        if curr_lexeme.text == "unsigned" {
            match self.get_next_token() {
                None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                Some(x) => {
                    curr_lexeme = x;
                    
                    if first_set_IntergerType.contains(&curr_lexeme.text) {
                        Ok(())
                    } else {
                        Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                    }
                }
            }
        } else {
            Ok(())
        }
    }

    fn fun_FloatType(&self) -> Result<(), MyError> {
        let syntax = String::from("FloatType := float | double");
        let curr_lexeme = self.get_curr_token();

        if curr_lexeme.text == "float" || curr_lexeme.text == "double" {
            Ok(())
        } else {
            Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
        }
    }

    fn fun_Assignment(&mut self) -> Result<(), MyError> {
        let syntax = String::from("Assignment := Identifier = {Identifier =} Expression ;");
        let functional_token = self.get_curr_token();

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: functional_token.line_num,
                char_pos: functional_token.char_pos, syntax}),
            Some(x) => {
                let mut curr_lexeme = x;

                if curr_lexeme.text == "=" {
                    let mut nextnext_token;
                    match self.peek_nextnext_token() {
                        None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => {
                            nextnext_token = x;
                        }
                    }
                    match self.get_next_token() {
                        None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => {
                            curr_lexeme = x;
                        }
                    }

                    while matches!(curr_lexeme.token_type, TokenType::Identifier) && nextnext_token.text == "=" {
                        match self.peek_nextnext_token() {
                            None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                            Some(x) => {
                                nextnext_token = x;
                            }
                        }
                        match self.get_next_token() {
                            None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                            Some(x) => {
                                curr_lexeme = x;
                            }
                        }
                    }

                    if curr_lexeme.text == "(" || matches!(curr_lexeme.token_type, TokenType::IntConstant) ||
                    matches!(curr_lexeme.token_type, TokenType::FloatConstant) || 
                    matches!(curr_lexeme.token_type, TokenType::Identifier) {
                        match self.fun_Expression() {
                            Ok(()) => (),
                            Err(e) => println!("{}", e)
                        }
                        curr_lexeme = self.get_curr_token();
                        match self.get_next_token() {
                            None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                            Some(x) => {
                                curr_lexeme = x;

                                if curr_lexeme.text == ";" {
                                    Ok(())
                                } else {
                                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                                }
                            }
                        }
                    } else {
                        Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                    }
                } else {
                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                }
            }
        }
    }

    fn fun_WhileLoop(&mut self) -> Result<(), MyError> {
        let syntax = String::from("WhileLoop := while ( Expression ) Block");
        let functional_token = self.get_curr_token();

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: functional_token.line_num,
                char_pos: functional_token.char_pos, syntax}),
            Some(x) => {
                let mut curr_lexeme = x;
                if curr_lexeme.text == "(" {

                    match self.get_next_token() {
                        None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => {
                            
                            match self.fun_Expression() {
                                Ok(()) => (),
                                Err(e) => println!("{}", e)
                            }
                            curr_lexeme = self.get_curr_token();

                            match self.get_next_token() {
                                None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                                Some(x) => {
                                    curr_lexeme = x;
                                    if curr_lexeme.text == ")" {

                                        match self.get_next_token() {
                                            None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                                            Some(x) => {
                                                curr_lexeme = x;
                                                if curr_lexeme.text == "{" {
                                                    match self.fun_Block() {
                                                        Ok(()) => (),
                                                        Err(e) => println!("{}", e)
                                                    }
                                                    Ok(())
                                                } else {
                                                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                                                }
                                            }
                                        }
                                    } else {
                                        Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                                    }
                                }
                            }
                        }
                    }
                } else {
                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                }
            }
        }
    }

    fn fun_IfStatement(&mut self) -> Result<(), MyError> {
        let syntax = String::from("IfStatement := if ( Expression ) Block");
        let functional_token = self.get_curr_token();

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: functional_token.line_num,
                char_pos: functional_token.char_pos, syntax}),
            Some(x) => {
                let mut curr_lexeme = x;
                if curr_lexeme.text == "(" {

                    match self.get_next_token() {
                        None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => {
                            
                            match self.fun_Expression() {
                                Ok(()) => (),
                                Err(e) => println!("{}", e)
                            }
                            curr_lexeme = self.get_curr_token();

                            match self.get_next_token() {
                                None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                                Some(x) => {
                                    curr_lexeme = x;
                                    if curr_lexeme.text == ")" {

                                        match self.get_next_token() {
                                            None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                                            Some(x) => {
                                                curr_lexeme = x;
                                                if curr_lexeme.text == "{" {
                                                    match self.fun_Block() {
                                                        Ok(()) => (),
                                                        Err(e) => println!("{}", e)
                                                    }
                                                    Ok(())
                                                } else {
                                                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                                                }
                                            }
                                        }
                                    } else {
                                        Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                                    }
                                }
                            }
                        }
                    }
                } else {
                    Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                }
            }
        }
    }

    fn fun_ReturnStatement(&mut self) -> Result<(), MyError> {
        let syntax = String::from("ReturnStatement := return Expression ;");
        let functional_token = self.get_curr_token();

        match self.get_next_token() {
            None => Err(MyError::SyntaxError{line_num: functional_token.line_num,
                char_pos: functional_token.char_pos, syntax}),
            Some(x) => {

                match self.fun_Expression() {
                    Ok(()) => (),
                    Err(e) => println!("{}", e)
                }
                let mut curr_lexeme = self.get_curr_token();
                
                match self.get_next_token() {
                    None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                    Some(x) => {
                        curr_lexeme = x;
                        if curr_lexeme.text == ";" {
                            Ok(())
                        } else {
                            Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                        }
                    }
                }
            }
        }
    }

    fn fun_Expression(&mut self) -> Result<(), MyError> {
        let syntax = String::from("Expression := SimpleExpression [ RelationOperator SimpleExpression ]");
        match self.fun_SimpleExpression() {
            Ok(()) => (),
            Err(e) => println!("{}", e)
        }

        match self.get_next_token() {
            None => Ok(()),
            Some(x) => {
                match self.fun_RelationOperator() {
                    Ok(()) => (),
                    Err(e) => println!("{}", e)
                }
                let curr_lexeme = self.get_curr_token();

                match self.get_next_token() {
                    None => Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                    Some(x) => {
                        match self.fun_SimpleExpression() {
                            Ok(()) => (),
                            Err(e) => println!("{}", e)
                        }
                        Ok(())
                    }
                }
            }
        }
    }

    fn fun_SimpleExpression(&mut self) -> Result<(), MyError> {
        let syntax = String::from("SimpleExpression := Term { AddOperator Term }");
        match self.fun_Term() {
            Ok(()) => (),
            Err(e) => println!("{}", e)
        }

        match self.get_next_token() {
            None => Ok(()),
            Some(x) => {
                let mut curr_lexeme = x;
                while curr_lexeme.text == "+" || curr_lexeme.text == "-" {

                    match self.get_next_token() {
                        None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => {
                            curr_lexeme = x;
                            match self.fun_Term() {
                                Ok(()) => (),
                                Err(e) => println!("{}", e)
                            }

                            match self.get_next_token() {
                                None => return Ok(()),
                                Some(x) => curr_lexeme = x
                            }
                        }
                    }
                }
                Ok(())
            }
        }
    }

    fn fun_Term(&mut self) -> Result<(), MyError> {
        let syntax = String::from("Term := Factor { MultapgrAtgr Factor }");
        match self.fun_Factor() {
            Ok(()) => (),
            Err(e) => println!("{}", e)
        }

        match self.get_next_token() {
            None => Ok(()),
            Some(x) => {
                let mut curr_lexeme = x;
                while curr_lexeme.text == "*" || curr_lexeme.text == "/" {

                    match self.get_next_token() {
                        None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => {
                            match self.fun_Factor() {
                                Ok(()) => (),
                                Err(e) => println!("{}", e)
                            }

                            match self.get_next_token() {
                                None => return Ok(()),
                                Some(x) => curr_lexeme = x
                            }
                        }
                    }
                }
                Ok(())
            }
        }
    }

    fn fun_Factor(&mut self) -> Result<(), MyError> {
        let syntax = String::from("Term := Factor { MultapgrAtgr Factor }");
        let mut curr_lexeme = self.get_curr_token();

        if curr_lexeme.text == "(" {

            match self.get_next_token() {
                None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                Some(x) => {
                    match self.fun_Expression() {
                        Ok(()) => (),
                        Err(e) => println!("{}", e)
                    }

                    curr_lexeme = self.get_curr_token();
                    match self.get_next_token() {
                        None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                        Some(x) => {
                            curr_lexeme = x;
                            if curr_lexeme.text == ")" {
                                Ok(())
                            } else {
                                Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                            }
                        }
                    }
                }
            }
        } else if matches!(curr_lexeme.token_type, TokenType::FloatConstant) ||
        matches!(curr_lexeme.token_type, TokenType::IntConstant) {
            Ok(())
        } else if matches!(curr_lexeme.token_type, TokenType::Identifier) {

            match self.get_next_token() {
                None => return Ok(()),
                Some(x) => {
                    curr_lexeme = x;
                    if curr_lexeme.text == "(" {

                        match self.get_next_token() {
                            None => return Ok(()),
                            Some(x) => {
                                match self.fun_Expression() {
                                    Ok(()) => (),
                                    Err(e) => println!("{}", e)
                                }
                                curr_lexeme = self.get_curr_token();

                                match self.get_next_token() {
                                    None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                                    Some(x) => {
                                        curr_lexeme = x;

                                        while curr_lexeme.text == "," {

                                            match self.get_next_token() {
                                                None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                                                Some(x) => {
                                                    match self.fun_Expression() {
                                                        Ok(()) => (),
                                                        Err(e) => println!("{}", e)
                                                    }
                                                    curr_lexeme = self.get_curr_token();

                                                    match self.get_next_token() {
                                                        None => return Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax}),
                                                        Some(x) => curr_lexeme = x
                                                    }
                                                }
                                            }
                                        }

                                        if curr_lexeme.text == ")" {
                                            Ok(())
                                        } else {
                                            Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
                    }
                }
            }
            
        } else {
            Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
        }
    }

    fn fun_RelationOperator(&mut self) -> Result<(), MyError> {
        let syntax = String::from("RelationOperator := ( == ) | < | > | ( <= ) | ( >= ) | ( != )");
        let curr_lexeme = self.get_curr_token();

        if curr_lexeme.text == "==" {
            Ok(())
        } else if curr_lexeme.text == "<" {
            Ok(())
        } else if curr_lexeme.text == ">" {
            Ok(())
        } else if curr_lexeme.text == "<=" {
            Ok(())
        } else if curr_lexeme.text == ">=" {
            Ok(())
        } else if curr_lexeme.text == "!=" {
            Ok(())
        } else {
            Err(MyError::SyntaxError{line_num: curr_lexeme.line_num, char_pos: curr_lexeme.char_pos, syntax})
        }
    }

}

fn main() {
    // Run program with "cargo run examples/example1.x"
    let args: Vec<String> = env::args().collect(); 
    let filename = args[1].clone();
    let mut my_cstream = Cstream::new(&filename);
    println!("{:?}", my_cstream.get_content());
    
    let all_tokens: Vec<Token> = Scanner(&mut my_cstream);
    println!("{:?}", &all_tokens);
    let mut my_parser = Parser::new(all_tokens);
    match my_parser.fun_Program() {
        Ok(()) => (),
        Err(e) => println!("{}", e),
      }
}
