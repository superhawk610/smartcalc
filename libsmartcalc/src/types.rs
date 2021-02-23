use alloc::vec::Vec;
use core::result::Result;
use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::format;

use alloc::collections::btree_map::BTreeMap;
use chrono::{NaiveDateTime, NaiveTime};
use crate::executer::Storage;
use crate::token::ui_token::{UiTokenType};

use crate::tokinizer::{TokenLocation, TokenLocationStatus, Tokinizer};

pub type TokinizeResult     = Result<Vec<TokenLocation>, (&'static str, u16, u16)>;
pub type ExpressionFunc     = fn(fields: &BTreeMap<String, &TokenLocation>) -> core::result::Result<TokenType, String>;
pub type TokenParserResult  = Result<bool, (&'static str, u16)>;
pub type AstResult          = Result<BramaAstType, (&'static str, u16, u16)>;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct VariableInfo {
    pub index: usize,
    pub name: String,
    pub tokens: Vec<Token>,
    pub data: Rc<BramaAstType>
}

unsafe impl Send for VariableInfo {}
unsafe impl Sync for VariableInfo {}

impl VariableInfo {
    pub fn update_data(&mut self, data: Rc<BramaAstType>) {
        self.data = data;
    }
}

impl ToString for VariableInfo {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum FieldType {
    Text(String),
    Date(String),
    Time(String),
    Money(String),
    Percent(String),
    Number(String),
    Group(Vec<String>),
    NumberOrMoney(String)
}

unsafe impl Send for FieldType {}
unsafe impl Sync for FieldType {}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaNumberSystem {
    Binary      = 0,
    Octal       = 1,
    Decimal     = 2,
    Hexadecimal = 3
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Money {
    pub code: String,
    pub symbol: String,
    pub thousands_separator: String,
    pub decimal_separator: String,
    pub symbol_on_left: bool,
    pub space_between_amount_and_symbol: bool,
    pub decimal_digits: u8
}

#[derive(Debug, Clone)]
pub struct Token {
    pub start: u16,
    pub end: u16,
    pub token: TokenType
}

unsafe impl Send for Token {}
unsafe impl Sync for Token {}


#[derive(Debug, Clone)]
pub enum TokenType {
    Number(f64),
    Text(String),
    Time(NaiveTime),
    Date(NaiveDateTime),
    DateTime(NaiveDateTime),
    Operator(char),
    Field(Rc<FieldType>),
    Percent(f64),
    Money(f64, String),
    Variable(Rc<VariableInfo>)
}


impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (TokenType::Text(l_value),     TokenType::Text(r_value)) => *l_value == *r_value,
            (TokenType::Number(l_value),   TokenType::Number(r_value)) => l_value == r_value,
            (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
            (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
            (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
            (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
            (TokenType::Time(l_value),     TokenType::Time(r_value)) => l_value == r_value,
            (TokenType::Field(l_value),    TokenType::Field(r_value)) => {
                match (&**l_value, &**r_value) {
                    (FieldType::Percent(l), FieldType::Percent(r)) => r == l,
                    (FieldType::Number(l),  FieldType::Number(r)) => r == l,
                    (FieldType::Text(l),    FieldType::Text(r)) => r == l,
                    (FieldType::Date(l),    FieldType::Date(r)) => r == l,
                    (FieldType::Time(l),    FieldType::Time(r)) => r == l,
                    (FieldType::Money(l),   FieldType::Money(r)) => r == l,
                    (FieldType::Group(l),   FieldType::Group(r)) => r == l,
                    (FieldType::NumberOrMoney(l),   FieldType::NumberOrMoney(r)) => r == l,
                    (_, _) => false,
                }
            },
            (_, _)  => false
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match &self.token {
            TokenType::Number(number) => number.to_string(),
            TokenType::Text(text) => text.to_string(),
            TokenType::Time(time) => time.to_string(),
            TokenType::Date(date) => date.to_string(),
            TokenType::DateTime(datetime) => datetime.to_string(),
            TokenType::Operator(ch) => ch.to_string(),
            TokenType::Field(_) => "field".to_string(),
            TokenType::Percent(number) => format!("%{}", number),
            TokenType::Money(price, currency) => format!("{} {}", price, currency.to_string()),
            TokenType::Variable(var) => var.to_string()
        }
    }
}

impl Token {
    pub fn variable_compare(left: &TokenLocation, right: Rc<BramaAstType>) -> bool {
        match &left.token_type {
            Some(token) => match (&token, &*right) {
                (TokenType::Text(l_value), BramaAstType::Symbol(r_value)) => &**l_value == r_value,
                (TokenType::Number(l_value), BramaAstType::Number(r_value)) => *l_value == *r_value,
                (TokenType::Percent(l_value), BramaAstType::Percent(r_value)) => *l_value == *r_value,
                (TokenType::Time(l_value), BramaAstType::Time(r_value)) => *l_value == *r_value,
                (TokenType::Money(l_value, l_symbol), BramaAstType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol.to_string() == *r_symbol,
                (TokenType::Field(l_value), _) => {
                    match (&**l_value, &*right) {
                        (FieldType::Percent(_), BramaAstType::Percent(_)) => true,
                        (FieldType::Number(_), BramaAstType::Number(_)) => true,
                        (FieldType::Text(_), BramaAstType::Symbol(_)) => true,
                        (FieldType::Time(_), BramaAstType::Time(_)) => true,
                        (FieldType::Money(_),   BramaAstType::Money(_, _)) => true,
                        (FieldType::NumberOrMoney(_),   BramaAstType::Money(_, _)) => true,
                        (FieldType::NumberOrMoney(_),   BramaAstType::Number(_)) => true,
                        (_, _) => false,
                    }
                },
                (_, _) => false
            },
            _ => false
        }
    }

    pub fn get_field_name(token: &TokenLocation) -> Option<String> {
        match &token.token_type {
            Some(token_type) => match &token_type {
                TokenType::Field(field) => match &**field {
                    FieldType::Text(field_name)    => Some(field_name.to_string()),
                    FieldType::Date(field_name)    => Some(field_name.to_string()),
                    FieldType::Time(field_name)    => Some(field_name.to_string()),
                    FieldType::Money(field_name)   => Some(field_name.to_string()),
                    FieldType::Percent(field_name) => Some(field_name.to_string()),
                    FieldType::Number(field_name)  => Some(field_name.to_string()),
                    FieldType::NumberOrMoney(field_name)  => Some(field_name.to_string()),
                    FieldType::Group(_)  => None
                },
                _ => None
            },
            _ => None
        }
    }

    pub fn is_same(tokens: &Vec<Token>, rule_tokens: &Vec<Token>) -> Option<usize> {
        let total_rule_token       = rule_tokens.len();
        let mut rule_token_index   = 0;
        let mut target_token_index = 0;
        let mut start_token_index  = 0;

        loop {
            match tokens.get(target_token_index) {
                Some(token) => {
                    if token == &rule_tokens[rule_token_index] {
                        rule_token_index   += 1;
                        target_token_index += 1;
                    }
                    else {
                        rule_token_index    = 0;
                        target_token_index += 1;
                        start_token_index   = target_token_index;
                    }

                    if total_rule_token == rule_token_index { break; }
                },
                _=> break
            }
        }

        if total_rule_token == rule_token_index {
            return Some(start_token_index);
        }
        None
    }

    pub fn is_same_location(tokens: &Vec<TokenLocation>, rule_tokens: &Vec<Token>) -> Option<usize> {
        let total_rule_token       = rule_tokens.len();
        let mut rule_token_index   = 0;
        let mut target_token_index = 0;
        let mut start_token_index  = 0;

        loop {
            match tokens.get(target_token_index) {
                Some(token) => {
                    if token == &rule_tokens[rule_token_index] {
                        rule_token_index   += 1;
                        target_token_index += 1;
                    }
                    else {
                        rule_token_index    = 0;
                        target_token_index += 1;
                        start_token_index   = target_token_index;
                    }

                    if total_rule_token == rule_token_index { break; }
                },
                _=> break
            }
        }

        if total_rule_token == rule_token_index {
            return Some(start_token_index);
        }
        None
    }

    pub fn update_for_variable(tokenizer: &mut Tokinizer, storage: Rc<Storage>) {
        let mut token_start_index = 0;
        for (index, token) in tokenizer.token_locations.iter().enumerate() {
            match &token.token_type {
                Some(token) => match token {
                    TokenType::Operator('=') => {
                        token_start_index = index as usize + 1;

                        tokenizer.ui_tokens.update_tokens(0, tokenizer.token_locations[index - 1].end, UiTokenType::VariableDefination);                        
                        break;
                    },
                    _ => ()
                },
                _ => ()
            };
        }

       let mut update_tokens = true;

        while update_tokens {
            let mut found            = false;
            let mut closest_variable = usize::max_value();
            let mut variable_index   = 0;
            let mut variable_size    = 0;

            update_tokens            = false;

            for (index, variable) in storage.variables.borrow().iter().enumerate() {
                if let Some(start_index) = Token::is_same_location(&tokenizer.token_locations[token_start_index..].to_vec(), &variable.tokens) {
                    if start_index == closest_variable && variable_size < variable.tokens.len() {
                        closest_variable = start_index;
                        variable_index   = index;
                        variable_size    = variable.tokens.len();
                        found = true;
                    }
                    else if start_index < closest_variable {
                        closest_variable = start_index;
                        variable_index   = index;
                        variable_size    = variable.tokens.len();
                        found = true;
                    }
                }
            }

            if found {
                let remove_start_index  = token_start_index + closest_variable;
                let remove_end_index    = remove_start_index + variable_size;
                let text_start_position = tokenizer.token_locations[remove_start_index].start;
                let text_end_position   = tokenizer.token_locations[remove_end_index - 1].end;

                tokenizer.ui_tokens.update_tokens(text_start_position, text_end_position, UiTokenType::VariableUse);

                let buffer_length: usize = tokenizer.token_locations[remove_start_index..remove_end_index].iter().map(|s| s.original_text.len()).sum();
                let mut original_text = String::with_capacity(buffer_length);

                for token in tokenizer.token_locations[remove_start_index..remove_end_index].iter() {
                    original_text.push_str(&token.original_text.to_owned());
                }

                tokenizer.token_locations.drain(remove_start_index..remove_end_index);
                tokenizer.token_locations.insert(remove_start_index, TokenLocation {
                    start: text_start_position as usize,
                    end: text_end_position as usize,
                    token_type: Some(TokenType::Variable(storage.variables.borrow()[variable_index].clone())),
                    original_text: original_text.to_owned(),
                    status: TokenLocationStatus::Active
                });
                update_tokens = true;
            }
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (&self.token, &other.token) {
            (TokenType::Text(l_value),     TokenType::Text(r_value)) => l_value == r_value,
            (TokenType::Number(l_value),   TokenType::Number(r_value)) => l_value == r_value,
            (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
            (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
            (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
            (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
            (TokenType::Field(l_value), _) => {
                match (&**l_value, &other.token) {
                    (FieldType::Percent(_), TokenType::Percent(_)) => true,
                    (FieldType::Number(_),  TokenType::Number(_)) => true,
                    (FieldType::Text(_),    TokenType::Text(_)) => true,
                    (FieldType::Time(_),    TokenType::Time(_)) => true,
                    (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                    (FieldType::Group(items),    TokenType::Text(text)) => items.iter().find(|&item| item == text).is_some(),
                    (FieldType::NumberOrMoney(_),   TokenType::Money(_, _)) => true,
                    (FieldType::NumberOrMoney(_),   TokenType::Number(_)) => true,
                    (_, _) => false,
                }
            },
            (_, TokenType::Field(r_value)) => {
                match (&**r_value, &self.token) {
                    (FieldType::Percent(_), TokenType::Percent(_)) => true,
                    (FieldType::Number(_),  TokenType::Number(_)) => true,
                    (FieldType::Text(_),    TokenType::Text(_)) => true,
                    (FieldType::Time(_),    TokenType::Time(_)) => true,
                    (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                    (FieldType::Group(items),    TokenType::Text(text)) => items.iter().find(|&item| item == text).is_some(),
                    (FieldType::NumberOrMoney(_),   TokenType::Money(_, _)) => true,
                    (FieldType::NumberOrMoney(_),   TokenType::Number(_)) => true,
                    (_, _) => false
                }
            },
            (_, _)  => false
        }
    }
}

impl core::cmp::PartialEq<Token> for TokenLocation {
    fn eq(&self, other: &Token) -> bool {
        if self.token_type.is_none() {
            return false
        }

        match &self.token_type {
            Some(l_token) => match (&l_token, &other.token) {
                (TokenType::Text(l_value),     TokenType::Text(r_value)) => l_value == r_value,
                (TokenType::Number(l_value),   TokenType::Number(r_value)) => l_value == r_value,
                (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
                (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
                (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
                (TokenType::Field(l_value), _) => {
                    match (&**l_value, &other.token) {
                        (FieldType::Percent(_), TokenType::Percent(_)) => true,
                        (FieldType::Number(_),  TokenType::Number(_)) => true,
                        (FieldType::Text(_),    TokenType::Text(_)) => true,
                        (FieldType::Time(_),    TokenType::Time(_)) => true,
                        (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                        (FieldType::Group(items),    TokenType::Text(text)) => items.iter().find(|&item| item == text).is_some(),
                        (FieldType::NumberOrMoney(_),   TokenType::Money(_, _)) => true,
                        (FieldType::NumberOrMoney(_),   TokenType::Number(_)) => true,
                        (_, _) => false,
                    }
                },
                (_, TokenType::Field(r_value)) => {
                    match (&**r_value, &l_token) {
                        (FieldType::Percent(_), TokenType::Percent(_)) => true,
                        (FieldType::Number(_),  TokenType::Number(_)) => true,
                        (FieldType::Text(_),    TokenType::Text(_)) => true,
                        (FieldType::Time(_),    TokenType::Time(_)) => true,
                        (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                        (FieldType::Group(items),    TokenType::Text(text)) => items.iter().find(|&item| item == text).is_some(),
                        (FieldType::NumberOrMoney(_),   TokenType::Money(_, _)) => true,
                        (FieldType::NumberOrMoney(_),   TokenType::Number(_)) => true,
                        (_, _) => false
                    }
                },
                (_, _)  => false
            },
            _ => false
        }
    }
}

impl PartialEq for TokenLocation {
    fn eq(&self, other: &Self) -> bool {
        if self.token_type.is_none() || other.token_type.is_none() {
            return false
        }

        match (&self.token_type, &other.token_type) {
            (Some(l_token), Some(r_token)) => match (&l_token, &r_token) {
                (TokenType::Text(l_value),     TokenType::Text(r_value)) => l_value == r_value,
                (TokenType::Number(l_value),   TokenType::Number(r_value)) => l_value == r_value,
                (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
                (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
                (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
                (TokenType::Field(l_value), _) => {
                    match (&**l_value, &r_token) {
                        (FieldType::Percent(_), TokenType::Percent(_)) => true,
                        (FieldType::Number(_),  TokenType::Number(_)) => true,
                        (FieldType::Text(_),    TokenType::Text(_)) => true,
                        (FieldType::Time(_),    TokenType::Time(_)) => true,
                        (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                        (FieldType::Group(items),    TokenType::Text(text)) => items.iter().find(|&item| item == text).is_some(),
                        (FieldType::NumberOrMoney(_),   TokenType::Money(_, _)) => true,
                        (FieldType::NumberOrMoney(_),   TokenType::Number(_)) => true,
                        (_, _) => false,
                    }
                },
                (_, TokenType::Field(r_value)) => {
                    match (&**r_value, &l_token) {
                        (FieldType::Percent(_), TokenType::Percent(_)) => true,
                        (FieldType::Number(_),  TokenType::Number(_)) => true,
                        (FieldType::Text(_),    TokenType::Text(_)) => true,
                        (FieldType::Time(_),    TokenType::Time(_)) => true,
                        (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                        (FieldType::Group(items),    TokenType::Text(text)) => items.iter().find(|&item| item == text).is_some(),
                        (FieldType::NumberOrMoney(_),   TokenType::Money(_, _)) => true,
                        (FieldType::NumberOrMoney(_),   TokenType::Number(_)) => true,
                        (_, _) => false
                    }
                },
                (_, _)  => false
            },
            (_, _) => false
        }
    }
}

pub struct TokinizerBackup {
    pub index: u16,
    pub indexer: usize,
    pub column: u16
}

pub trait CharTraits {
    fn is_new_line(&self) -> bool;
    fn is_whitespace(&self) -> bool;
}

impl CharTraits for char {
    fn is_new_line(&self) -> bool {
        *self == '\n'
    }

    fn is_whitespace(&self) -> bool {
        matches!(*self, ' ' | '\r' | '\t')
    }
}


#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaAstType {
    None,
    Number(f64),
    Field(Rc<FieldType>),
    Percent(f64),
    Time(NaiveTime),
    Money(f64, String),
    Binary {
        left: Rc<BramaAstType>,
        operator: char,
        right: Rc<BramaAstType>
    },
    PrefixUnary(char, Rc<BramaAstType>),
    Assignment {
        index: usize,
        expression: Rc<BramaAstType>
    },
    Symbol(String),
    Variable(Rc<VariableInfo>)
}