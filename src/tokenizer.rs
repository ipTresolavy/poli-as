use regex::Regex;

use crate::{
    reader::Reader,
    token::{
        immediate::Immediate,
        instruction::Instruction,
        instruction_name::get_istr_regex,
        register::{Register, RegisterNumbers},
        Directive, Label, Number, Token,
    },
};

pub struct Tokenizer {
    reader: Reader,
}

impl Tokenizer {
    pub fn new(reader: Reader) -> Tokenizer {
        Tokenizer { reader }
    }

    pub fn is_eof(&self) -> bool {
        self.reader.is_eof()
    }

    pub fn consume_line(&mut self) -> Vec<Token> {
        let line = self.reader.consume_line();

        let literals = self.split_at_separators(&line);

        let mut line_tokens: Vec<Token> = vec![];

        for literal in literals {
            let token = self.create_token_from_literal(Some(literal));
            line_tokens.push(token);
        }

        line_tokens
    }

    pub fn reset(&mut self) {
        self.reader.reset();
    }

    fn split_at_separators(&self, line: &str) -> Vec<String> {
        // Another day another regex i want to kill myself over
        let re = Regex::new(r"(\d+)|(r\d+)|(\{|\})|(\[|\])|(-)|(!)|(=)|(\.[a-zA-Z]+)|(#0x\d+|#0b\d+|#0d\d+|#-?\d+)|([a-zA-Z\_\-]+:)|([a-zA-Z\_\-]+)").unwrap();
        let matches: Vec<String> = re
            .captures_iter(line)
            .filter_map(|caps| {
                for i in 1..caps.len() {
                    if let Some(m) = caps.get(i) {
                        return Some(m.as_str().to_string());
                    }
                }
                None
            })
            .collect();

        matches
    }

    fn create_token_from_literal(&mut self, literal: Option<String>) -> Token {
        if literal.is_none() {
            return Token::EOF;
        }

        let literal = literal.unwrap();

        if literal == "!" {
            return Token::BANG;
        }

        if literal == "[" {
            return Token::LPAREN;
        }

        if literal == "]" {
            return Token::RPAREN;
        }

        if literal == "{" {
            return Token::LBRACE;
        }

        if literal == "}" {
            return Token::RBRACE;
        }

        if literal == "-" {
            return Token::MINUS;
        }

        if literal == "=" {
            return Token::EQUAL;
        }

        if literal == "," {
            return Token::COMMA;
        }

        if let Ok(number) = literal.parse::<u8>() {
            return Token::NUMBER(Number::new(number));
        }

        if literal.starts_with('#') {
            let immediate = Immediate::new(literal.chars().skip(1).collect::<String>());

            if let Some(value) = immediate {
                return Token::IMMEDIATE(value);
            } else {
                return Token::ILLEGAL;
            }
        }

        if literal.starts_with('.') {
            let directory = Directive::new(literal);
            return Token::DIRECTIVE(directory);
        }

        if (literal.starts_with('r') && second_char_is_number(&literal)) || is_special_reg(&literal)
        {
            return reg_from_literal(&literal);
        }

        if literal.ends_with(':') {
            return Token::LABEL(Label::new(
                literal.chars().take(literal.len() - 1).collect::<String>(),
            ));
        }

        let istr_regex = get_istr_regex();

        if istr_regex.is_match(&literal) {
            let istr = istr_regex
                .captures(&literal)
                .expect("if match should be able to capture");

            let operation = istr
                .get(1)
                .expect("if captured should be able to get the first")
                .as_str();

            let condition = istr.get(2).map(|condition| condition.as_str());
            let save_register = istr.get(3).map(|save_reg| save_reg.as_str());

            let instruction_token = Instruction::new(operation, save_register, condition);

            if let Some(instruction) = instruction_token {
                return Token::INSTRUCTION(instruction);
            } else {
                return Token::ILLEGAL;
            }
        }

        if literal
            .chars()
            .all(|c| c.is_alphabetic() || c == '_' || c == '-')
        {
            return Token::LABELREF(literal);
        }

        Token::ILLEGAL
    }
}

fn reg_from_literal(literal: &str) -> Token {
    let reg_num = literal.chars().collect::<String>();

    let reg_num = if is_special_reg(&reg_num) {
        parse_special_reg(&reg_num)
    } else {
        let capture = parse_regex_number(&reg_num).unwrap();

        if capture > 15 {
            return Token::ILLEGAL;
        }

        RegisterNumbers::from_num(capture as u32)
    };

    match reg_num {
        Some(reg_num) => {
            let register = Register::new(reg_num);
            Token::REGISTER(register)
        }
        None => Token::ILLEGAL,
    }
}

fn parse_special_reg(str: &str) -> Option<RegisterNumbers> {
    match str {
        "sp" => Some(RegisterNumbers::THIRTEEN),
        "lr" => Some(RegisterNumbers::FOURTEEN),
        "pc" => Some(RegisterNumbers::FIFTEEN),
        _ => None,
    }
}

fn parse_regex_number(reg_num: &str) -> Option<u8> {
    let re = Regex::new(r"^r(\d{1,2})$").expect("regex should be valid");

    let capture = re.captures(reg_num).unwrap();
    let capture = capture.get(1).unwrap().as_str().parse::<u8>().unwrap();

    Some(capture)
}

fn second_char_is_number(str: &str) -> bool {
    if str.len() < 2 {
        return false;
    }
    str.chars().nth(1).unwrap_or('b').is_numeric()
}

fn is_special_reg(str: &str) -> bool {
    str == "sp" || str == "lr" || str == "pc"
}
