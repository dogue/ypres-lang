use crate::{
    parser::ast::{Expression, Node},
    tokenizer::token::Token,
};

use super::Identifier;

pub struct Assignment<E: Expression> {
    token: Token,
    ident: Identifier,
    value: E,
}

impl<E: Expression> Node for Assignment<E> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!(
            "{} {} {}",
            self.ident.string(),
            self.token_literal(),
            self.value.string(),
        )
    }
}

impl<E: Expression> Expression for Assignment<E> {}
