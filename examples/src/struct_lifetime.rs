#![allow(dead_code)]
use ouroboros::self_referencing;

pub struct Token<'text> {
    x: &'text str,
}

pub struct Node<'text, 'ast> {
    token: &'ast Token<'text>,
}

pub struct Nodes<'text, 'ast> {
    delim: &'ast Token<'text>,
    nodes: Vec<Node<'text, 'ast>>,
}

#[self_referencing]
pub struct Ast<'a> {
    pub(crate) tokens: Box<[Token<'a>]>,
    #[borrows(tokens)]
    #[covariant] // FIXME
    pub(crate) nodes: Nodes<'a, 'this>,
}

type Error = Box<dyn std::error::Error>;
pub fn parse<'text>(tokens: Vec<Token<'text>>) -> Result<Ast<'text>, Error> {
    Ok(AstTryBuilder {
        tokens: tokens.into_boxed_slice(),
        nodes_builder: |tokens| -> Result<_, Error> {
            if tokens.len() == 1 {
                return Err("no tokens".into());
            } else {
                Ok(Nodes {
                    delim: &tokens[0],
                    nodes: tokens[1..].iter().map(|token| Node { token }).collect(),
                })
            }
        },
    }
    .try_build()?)
}
