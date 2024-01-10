use ypres::tokenizer::{token::TokenType, Tokenizer};

fn main() {
    let input = r#"// example Ypres source code

    main: nil () {
        x := 5
        y := 3
        z := add(x, y)
    }

    add: i32 (x, y: i32) {
        x + y
    }
    "#;
    let mut t = Tokenizer::new(input);

    loop {
        let tok = t.next_token();

        println!("{tok:?}");

        if tok.token_type == TokenType::EOF {
            break;
        }
    }
}

// output:
// Token { token_type: Comment, literal: "// example Ypres source code", position: (1, 1) }
// Token { token_type: Ident, literal: "ain:", position: (3, 6) }
// Token { token_type: Colon, literal: ":", position: (3, 10) }
// Token { token_type: Ident, literal: "il ", position: (3, 12) }
// Token { token_type: LParen, literal: "(", position: (3, 16) }
// Token { token_type: RParen, literal: ")", position: (3, 17) }
// Token { token_type: LSquirly, literal: "{", position: (3, 19) }
// Token { token_type: Ident, literal: " ", position: (4, 10) }
// Token { token_type: Colon, literal: ":", position: (4, 12) }
// Token { token_type: Invalid, literal: "", position: (4, 13) }
// Token { token_type: Number, literal: "\n", position: (4, 15) }
// Token { token_type: Ident, literal: " ", position: (5, 10) }
// Token { token_type: Colon, literal: ":", position: (5, 12) }
// Token { token_type: Invalid, literal: "", position: (5, 13) }
// Token { token_type: Number, literal: "\n", position: (5, 15) }
// Token { token_type: Ident, literal: " ", position: (6, 10) }
// Token { token_type: Colon, literal: ":", position: (6, 12) }
// Token { token_type: Invalid, literal: "", position: (6, 13) }
// Token { token_type: Ident, literal: "dd(", position: (6, 15) }
// Token { token_type: LParen, literal: "(", position: (6, 18) }
// Token { token_type: Ident, literal: ",", position: (6, 19) }
// Token { token_type: Comma, literal: ",", position: (6, 20) }
// Token { token_type: Ident, literal: ")", position: (6, 22) }
// Token { token_type: RParen, literal: ")", position: (6, 23) }
// Token { token_type: RSquirly, literal: "}", position: (7, 6) }
// Token { token_type: Ident, literal: "dd:", position: (9, 6) }
// Token { token_type: Colon, literal: ":", position: (9, 9) }
// Token { token_type: Ident, literal: "32 ", position: (9, 11) }
// Token { token_type: LParen, literal: "(", position: (9, 15) }
// Token { token_type: Ident, literal: ",", position: (9, 16) }
// Token { token_type: Comma, literal: ",", position: (9, 17) }
// Token { token_type: Ident, literal: ":", position: (9, 19) }
// Token { token_type: Colon, literal: ":", position: (9, 20) }
// Token { token_type: Ident, literal: "32)", position: (9, 22) }
// Token { token_type: RParen, literal: ")", position: (9, 25) }
// Token { token_type: LSquirly, literal: "{", position: (9, 27) }
// Token { token_type: Ident, literal: " ", position: (10, 10) }
// Token { token_type: Plus, literal: "+", position: (10, 12) }
// Token { token_type: Ident, literal: "\n", position: (10, 14) }
// Token { token_type: RSquirly, literal: "}", position: (11, 6) }
// Token { token_type: EOF, literal: "", position: (12, 6) }
