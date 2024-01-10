use super::{keywords::Type, *};

struct TokenTest {
    expected_type: TokenType,
    expected_literal: String,
}

impl TokenTest {
    fn new(et: TokenType, el: &str) -> Self {
        Self {
            expected_type: et,
            expected_literal: el.to_owned(),
        }
    }
}

#[test]
fn test_next_token() {
    // let input = r#"// example Ypres source code

    // main: nil () {
    //     x := 5
    //     y := 3
    //     z := add(x, y)
    // }

    // add: i32 (x, y: i32) {
    //     x + y
    // }
    // "#;

    let input = std::fs::read_to_string("example.ypres").expect("failed to open source file");

    let tests = vec![
        TokenTest::new(TokenType::Comment, "// example Ypres source code"),
        TokenTest::new(TokenType::Keyword(Keyword::Main), "main"),
        TokenTest::new(TokenType::Colon, ":"),
        TokenTest::new(TokenType::Type(Type::Nil), "nil"),
        TokenTest::new(TokenType::LParen, "("),
        TokenTest::new(TokenType::RParen, ")"),
        TokenTest::new(TokenType::LSquirly, "{"),
        TokenTest::new(TokenType::Ident, "x"),
        TokenTest::new(TokenType::Colon, ":"),
        TokenTest::new(TokenType::Assign, "="),
        TokenTest::new(TokenType::Number, "5"),
        TokenTest::new(TokenType::Ident, "y"),
        TokenTest::new(TokenType::Colon, ":"),
        TokenTest::new(TokenType::Assign, "="),
        TokenTest::new(TokenType::Number, "3"),
        TokenTest::new(TokenType::Ident, "z"),
        TokenTest::new(TokenType::Colon, ":"),
        TokenTest::new(TokenType::Assign, "="),
        TokenTest::new(TokenType::Ident, "add"),
        TokenTest::new(TokenType::LParen, "("),
        TokenTest::new(TokenType::Ident, "x"),
        TokenTest::new(TokenType::Comma, ","),
        TokenTest::new(TokenType::Ident, "y"),
        TokenTest::new(TokenType::RParen, ")"),
        TokenTest::new(TokenType::RSquirly, "}"),
        TokenTest::new(
            TokenType::Comment,
            "/* this is\n\ta\n\tmultiline\n\tcomment\n*/",
        ),
        TokenTest::new(TokenType::Ident, "add"),
        TokenTest::new(TokenType::Colon, ":"),
        TokenTest::new(TokenType::Type(Type::I32), "i32"),
        TokenTest::new(TokenType::LParen, "("),
        TokenTest::new(TokenType::Ident, "x"),
        TokenTest::new(TokenType::Comma, ","),
        TokenTest::new(TokenType::Ident, "y"),
        TokenTest::new(TokenType::Colon, ":"),
        TokenTest::new(TokenType::Type(Type::I32), "i32"),
        TokenTest::new(TokenType::RParen, ")"),
        TokenTest::new(TokenType::LSquirly, "{"),
        TokenTest::new(TokenType::Keyword(Keyword::If), "if"),
        TokenTest::new(TokenType::Ident, "x"),
        TokenTest::new(TokenType::Greater, ">"),
        TokenTest::new(TokenType::Ident, "y"),
        TokenTest::new(TokenType::LSquirly, "{"),
        TokenTest::new(TokenType::Ident, "x"),
        TokenTest::new(TokenType::Plus, "+"),
        TokenTest::new(TokenType::Ident, "y"),
        TokenTest::new(TokenType::RSquirly, "}"),
        TokenTest::new(TokenType::RSquirly, "}"),
        TokenTest::new(TokenType::Ident, "same"),
        TokenTest::new(TokenType::Colon, ":"),
        TokenTest::new(TokenType::Type(Type::Bool), "bool"),
        TokenTest::new(TokenType::LParen, "("),
        TokenTest::new(TokenType::Ident, "x"),
        TokenTest::new(TokenType::Comma, ","),
        TokenTest::new(TokenType::Ident, "y"),
        TokenTest::new(TokenType::Colon, ":"),
        TokenTest::new(TokenType::Type(Type::I32), "i32"),
        TokenTest::new(TokenType::RParen, ")"),
        TokenTest::new(TokenType::LSquirly, "{"),
        TokenTest::new(TokenType::Keyword(Keyword::If), "if"),
        TokenTest::new(TokenType::Ident, "x"),
        TokenTest::new(TokenType::Eq, "=="),
        TokenTest::new(TokenType::Ident, "y"),
        TokenTest::new(TokenType::LSquirly, "{"),
        TokenTest::new(TokenType::Keyword(Keyword::Return), "return"),
        TokenTest::new(TokenType::Keyword(Keyword::True), "true"),
        TokenTest::new(TokenType::RSquirly, "}"),
        TokenTest::new(TokenType::Keyword(Keyword::Else), "else"),
        TokenTest::new(TokenType::LSquirly, "{"),
        TokenTest::new(TokenType::Keyword(Keyword::Return), "return"),
        TokenTest::new(TokenType::Keyword(Keyword::False), "false"),
        TokenTest::new(TokenType::RSquirly, "}"),
        TokenTest::new(TokenType::RSquirly, "}"),
        TokenTest::new(TokenType::EOF, ""),
    ];

    let mut tokenizer = Tokenizer::new(&input);

    for test in tests {
        let token = tokenizer.next_token();

        assert_eq!(test.expected_type, token.token_type, "Token: {token:?}");
        assert_eq!(test.expected_literal, token.literal, "Token: {token:?}");
    }
}
