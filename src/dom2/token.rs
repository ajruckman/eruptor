#[derive(Debug)]
pub enum Token {
    ElemOpener(String),
    ElemCloser,
    ElemAttr(String, Option<String>),
    Content(Content),
}

pub trait Tokenizable {
    fn tokenize(&self, tree: &NodeTree) -> Vec<Token>;
}

impl Tokenizable for Node {
    fn tokenize(&self, tree: &NodeTree) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();

        for idx in &self.children {
            let node = tree.get_idx(*idx);

            match &node.value {
                NodeValue::Elem(e) => {
                    println!("Parent: {}", idx);

                    result.push(Token::ElemOpener(e.name.clone()));

                    for attr in &e.attrs {
                        for token in attr.tokenize(tree) {
                            result.push(token);
                        }
                    }

                    for child in &node.children {
                        let sub_node = tree.get_idx(*child);
                        println!("-> {}", child);

                        for token in sub_node.tokenize(tree) {
                            result.push(token);
                        }
                    }

                    result.push(Token::ElemCloser);
                }
                NodeValue::Content(c) => {
                    result.push(Token::Content(c.clone()));
                }
            }
        }

        return result;
    }
}

// impl Tokenizable for NodeValue {
//     fn tokenize(&self, tree: &NodeTree) -> Vec<Token> {
//         let mut result: Vec<Token> = Vec::new();
//
//         match self {
//             NodeValue::Elem(e) => {
//
//                 result.push(Token::ElemOpener(e.name.clone()));
//
//                 for attr in &e.attrs {
//                     for token in attr.tokenize(tree) {
//                         result.push(token);
//                     }
//                 }
//
//                 result.push(Token::ElemCloser);
//             }
//             NodeValue::Content(c) => {
//                 result.push(Token::Content(c.clone()));
//             }
//         }
//
//         return result;
//     }
// }

// impl Tokenizable for Elem {
//     fn tokenize(&self, tree: &NodeTree) -> Vec<Token> {
//         let mut result: Vec<Token> = Vec::new();
//
//         result.push(Token::ElemOpener(self.name.clone()));
//
//         for attr in &self.attrs {
//             for token in attr.tokenize(tree) {
//                 result.push(token);
//             }
//         }
//
//         result.push(Token::ElemCloser);
//
//         return result;
//     }
// }

impl Tokenizable for Attr {
    fn tokenize(&self, tree: &NodeTree) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();

        match self {
            Attr::String(k, v) => {
                result.push(Token::ElemAttr(k.clone(), Some(v.clone())));
            }
            Attr::Bool(k, v) => {
                if *v == false {
                    result.push(Token::ElemAttr(k.clone(), None));
                } else {
                    // Truthy values like: required='required'
                    result.push(Token::ElemAttr(k.clone(), Some(k.clone())));
                }
            }
        }

        return result;
    }
}
