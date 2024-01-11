use crate::{
    parser::ast::{Expression, Node},
    tokenizer::token::Token,
};

pub struct Variable<E: Expression> {
    token: Token,
    name: String,
    value: E,
}

impl<E: Expression> Node for Variable<E> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("{} = {}", self.name, self.value.string())
    }
}

impl<E: Expression> Expression for Variable<E> {}
