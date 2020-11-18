// Libraries
#[macro_use] extern crate lazy_static;
extern crate string_template;
extern crate regex;
use regex::Regex;
use regex::RegexSet;
use string_template::Template;

use std::{env};
use std::fs;
use std::string::ToString;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Lexeme {
    _ID(String), _ASSIGN(char), _POINT(String),
    _LPAREN(char), _NUM(u32), _COMMA(char),
    _RPAREN(char), _SEMICOLON(char), _PERIOD(char),
}

#[derive(Debug)]
pub enum Token {
    ID, ASSIGN, POINT,
    LPAREN, NUM, COMMA,
    RPAREN, SEMICOLON, PERIOD,
}

#[derive(Debug)]
struct Node{
    name: Token,
    data: Lexeme,
}

impl Node {
    pub fn new(n: Token, d:Lexeme) -> Node{
        Node { name: n, data: d, }
    }
}

fn main() {

    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    // If -s or If -p
    if config.query == "-s"{
        println!("; processing input file {}", config.filename);
    } else if config.query == "-p"{
        println!("/* processing input file {}", config.filename);
    }

    // Read file to string
    let contents =
        fs::read_to_string(config.filename).expect("Err: File Invalid");

    // Lexical Analyzer
    let tokens = lex(&contents);
    let good_tok = tokens.unwrap();

    // Syntax Analyzer
    if parse(&good_tok){
        if config.query == "-s"{
            println!("; Lexical and Syntax analysis passed");
            print_scheme(&good_tok);
        } else if config.query == "-p"{
            println!("   Lexical and Syntax analysis passed */");
            print_prolog(&good_tok);
        }
    } else {
        panic!("Err: Analysis Failed - Try Again.");
    }
}

/* LEXER */
fn lex(input: &String) -> Result<Vec<Node>, String> {

    // initial regex qualifiers
    let set = RegexSet::new(&[
        r"[0-9]",   // 0. Digit  | NUM
        r"[a-z]",   // 1. Letter | ID | POINT
        r"=",       // 2. Assign
        r",",       // 3. Comma
        r"\(",      // 4. LParen
        r"\)",      // 5. RParen
        r"\.",      // 6. Period
        r";",       // 7. Semicolon
        r"\s",       // 8. Space
    ]).unwrap();

    let mut tokens = Vec::new();               // vector of tokens
    let mut it = input.chars().peekable(); // iterator

    /* Iterates by char through string and individually matches initial
    regex qualifier. Generates vector(s) of referencing regex expression(s)
    corresponding to individual character. Further qualifies char selected. */
    while let Some(&c) = it.peek() {

        // tmp vector holding matches from regex set
        let matches: Vec<_> = set.matches(&c.to_string()).into_iter().collect();

        // if matches != 0 - i.e at least one match
        if matches.len() != 0{
            match matches[0]{
                0 => {  // Digit | Num
                    let mut foo_num = 0;
                    while let Some(Ok(foo)) = it.peek().map(|c| c.to_string().parse::<u32>()){
                        foo_num = foo_num * 10 + foo;
                        it.next();
                    }
                    tokens.push(Node::new(Token::NUM, Lexeme::_NUM(foo_num)));
                }

                1 => {  // Letter | ID | POINT
                    let mut foo_string = String::new();
                    loop{
                        // gets all digits consecutive digits that follow
                        if digit_rx(it.peek().unwrap()) {
                            foo_string.push(*it.peek().unwrap());
                            it.next();
                        } else {
                            break;
                        }
                    }
                    if foo_string == "point" {  // if POINT token
                        tokens.push(Node::new(Token::POINT, Lexeme::_POINT(foo_string)));
                    } else {                    // if ID token
                        tokens.push(Node::new(Token::ID, Lexeme::_ID(foo_string)));
                    }
                }
                2 => {  // ASSIGN
                    tokens.push(Node::new(Token::ASSIGN, Lexeme::_ASSIGN(c)));
                    it.next();
                }
                3 => {  // COMMA
                    tokens.push(Node::new(Token::COMMA, Lexeme::_COMMA(c)));
                    it.next();
                }
                4 => {  // LPAREN
                    tokens.push(Node::new(Token::LPAREN, Lexeme::_LPAREN(c)));
                    it.next();
                }
                5 => {  // RPAREN
                    tokens.push(Node::new(Token::RPAREN, Lexeme::_RPAREN(c)));
                    it.next();
                }
                6 => {  // PERIOD
                    tokens.push(Node::new(Token::PERIOD, Lexeme::_PERIOD(c)));
                    it.next();
                }
                7 => {  // SEMICOLON
                    tokens.push(Node::new(Token::SEMICOLON, Lexeme::_SEMICOLON(c)));
                    it.next();
                }
                8 => {  // WHITESPACE
                    it.next();
                }
                _ => {panic!("Lexical Err: Input Type Unknown");
                }
            }
        } else { // if num of matches < 0 or > 0
            panic!("Lexical Err: -> {} <- Not Allowed", c);
        }
    }
    Ok(tokens)
}

/* LEXER Helper */
fn digit_rx(c: &char) -> bool {
    let text = c.to_string();
    lazy_static! {
        static ref RE: Regex = Regex::new("[a-z]").unwrap();
    }
    RE.is_match(&text)
}

/* Parser */
/*
    Grammar:

    START     --> POINT_DEF; POINT_DEF; POINT_DEF.
    POINT_DEF --> ID = point(NUM, NUM)
    ID        --> LETTER+
    NUM       --> DIGIT+
    LETTER    --> a | b | c | d | e | f | g | ... | z
    DIGIT     --> 0 | 1 | 2 | 3 | 4 | 5 | 6 | ... | 9
*/
fn parse(tokens: &Vec<Node>) -> bool {

    let syntax_ok = true;

    for (i, token) in (&tokens).iter().enumerate(){
        match (i, &token.name) {
            (0, Token::ID) | (9, Token::ID) | (18, Token::ID)                   => {}
            (1, Token::ASSIGN) | (10, Token::ASSIGN) | (19, Token::ASSIGN)      => {}
            (2, Token::POINT) | (11, Token::POINT) | (20, Token::POINT)         => {}
            (3, Token::LPAREN) | (12, Token::LPAREN) | (21, Token::LPAREN)      => {}
            (4, Token::NUM) | (13, Token::NUM) | (22, Token::NUM)               => {}
            (5, Token::COMMA) | (14, Token::COMMA) | (23, Token::COMMA)         => {}
            (6, Token::NUM) | (15, Token::NUM) | (24, Token::NUM)               => {}
            (7, Token::RPAREN) | (16, Token::RPAREN) | (25, Token::RPAREN)      => {}
            (8, Token::SEMICOLON) | (17, Token::SEMICOLON)                      => {}
            (26, Token::PERIOD)                                                 => {}

            /* If none of the grammar items above match at the specified position,
            a panic message is thrown */
            _ => {
                panic!("Syntax Err: Please Review Grammar")
            }
        }
    }
    return syntax_ok;
}


/* Scheme Output */
fn print_scheme(tokens: &Vec<Node>){
    let temp_scheme =
        Template::new("(make-point {{}} {{}}) (make-point {{}} {{}}) (make-point {{}} {{}})");

    let pt_a1 = get_num(&tokens[4]).clone().to_string();
    let pt_a2 = get_num(&tokens[6]).clone().to_string();
    let pt_b1 = get_num(&tokens[13]).clone().to_string();
    let pt_b2 = get_num(&tokens[15]).clone().to_string();
    let pt_c1 = get_num(&tokens[22]).clone().to_string();
    let pt_c2 = get_num(&tokens[24]).clone().to_string();

    let points = vec!{
        pt_a1.as_str(), pt_a2.as_str(),
        pt_b1.as_str(), pt_b2.as_str(),
        pt_c1.as_str(), pt_c2.as_str(),
    };

    let s = temp_scheme.render_positional(&points);

    println!("(calculate-triangle {})",s);
}

/* Prolog Output */
fn print_prolog(tokens: &Vec<Node>){
    let temp_rhs =
        Template::new("(point2d({{}},{{}}), point2d({{}},{{}}), point2d({{}}, {{}}))");

    let pt_a1 = get_num(&tokens[4]).clone().to_string();
    let pt_a2 = get_num(&tokens[6]).clone().to_string();
    let pt_b1 = get_num(&tokens[13]).clone().to_string();
    let pt_b2 = get_num(&tokens[15]).clone().to_string();
    let pt_c1 = get_num(&tokens[22]).clone().to_string();
    let pt_c2 = get_num(&tokens[24]).clone().to_string();

    let points = vec!{
        pt_a1.as_str(), pt_a2.as_str(),
        pt_b1.as_str(), pt_b2.as_str(),
        pt_c1.as_str(), pt_c2.as_str(),
    };

    let geometry = vec!{
        "line", "triangle", "vertical", "horizontal", "equilateral",
        "isosceles", "right", "scalene", "acute", "obtuse",
    };

    let rhs = temp_rhs.render_positional(&points);

    let temp_master = Template::new("{{lhs_obj}}{{rhs_point}}");

    for geo_obj in geometry {
        let mut rlhs = HashMap::new();
        rlhs.insert("lhs_obj", geo_obj);
        rlhs.insert("rhs_point", rhs.as_str());
        let s = temp_master.render(&rlhs);
        println!("query({})",s);
    }

    println!("writeln(T) :- write(T), nl.");
    println!("main:- forall(query(Q), Q-> (writeln(‘yes’)) ; (writeln(‘no’))),");
    println!("      halt.");

}

/* Scheme/Prolog Print Helper */
fn get_num(node: &Node) -> u32 {
    return match &node.data {
        Lexeme::_NUM(val) => {
            let p = val;
            p.clone()
        }
        _ => { panic!("something went wrong here");}
    }
}

/* File Open Struct */
struct Config {
    filename: String,
    query: String,
}

/* File Open */
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        } else if args.len() > 3 {
            panic!("too many arguments");
        }

        let filename = args[1].clone();
        let query = args[2].clone();

        let re_file = Regex::new(r".(cpl|txt)$").unwrap();
        if !re_file.is_match(&filename){
            panic!("Err: .txt or .cpl Files Only");
        }

        let re_query = Regex::new(r"-s|-p$").unwrap();
        if !re_query.is_match(&query){
            panic!("Err: Only -p | -s Queries Allowed");
        }
        // assert!(re_query.is_match(&query));

        Config { filename, query }
    }
}