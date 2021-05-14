pub enum Token {
    ElemOpener(String),
    ElemCloser,
    ElemAttr(String, Option<String>),
    Content(Content),
}

pub trait Tokenizable {
    fn tokenize(&self) -> Vec<Token>;
}

pub enum Node<'a> {
    Elem(Elem<'a>),
    Content(Content),
}

impl<'a> Node<'a> {
    fn hash(&self, h: &mut MetroHash) {
        match self {
            Node::Elem(e) => e.hash(h),
            Node::Content(c) => {
                match c {
                    Content::Plain(v) => (*v).hash(h),
                    Content::Markup(v) => (*v).hash(h),
                }
            }
        }
    }

    fn elem_ref(&mut self) -> &mut Elem<'a> {
        match self {
            Node::Elem(e) => e,
            Node::Content(_) => panic!("not an elem"),
        }
    }
}

impl<'a> Tokenizable for Node<'a> {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            Node::Elem(e) => {
                let mut result: Vec<Token> = Vec::new();

                result.push(Token::ElemOpener(e.name.to_owned()));

                for attr in &e.attrs {
                    match attr {
                        Attr::String(k, v) => result.push(Token::ElemAttr(k.to_string(), Some(v.to_string()))),
                        Attr::Bool(k, v) => {
                            if !v {
                                result.push(Token::ElemAttr(k.to_string(), None));
                            } else {
                                result.push(Token::ElemAttr(k.to_string(), Some(k.to_string())));
                            }
                        }
                    }
                }

                for child in &e.children {
                    for token in child.tokenize() {
                        result.push(token);
                    }
                }

                result.push(Token::ElemCloser);

                result
            }
            Node::Content(v) => vec![Token::Content(v.clone())],
        }
    }
}
