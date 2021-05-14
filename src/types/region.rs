pub struct Region<'a> {
    children: Vec<Node<'a>>,
}

impl<'a> Region<'a> {
    pub fn new(children: Vec<Node<'a>>) -> Region<'a> {
        Region {
            children,
        }
    }

    pub fn push_child(&mut self, et: Node<'a>) {
        self.children.push(et);
    }

    fn hash(&self, h: &mut MetroHash) {
        for child in &self.children {
            child.hash(h);
        }
    }

    pub fn hash_code(&mut self) -> u64 {
        let mut h = metrohash::MetroHash::default();
        for child in &self.children {
            child.hash(&mut h);
        }
        h.finish()
    }

    pub fn print(&self) {
        let mut open_elems: Vec<String> = Vec::new();
        let mut unclosed_opener = false;

        for token in self.tokenize() {
            match token {
                Token::ElemAttr(_, _) => {}
                _ => {
                    if unclosed_opener {
                        unclosed_opener = false;
                        print!(">");
                    }
                }
            }

            match token {
                Token::ElemOpener(e) => {
                    print!("<{}", e);
                    unclosed_opener = true;
                    open_elems.push(e);
                }
                Token::ElemCloser => {
                    let e = open_elems.pop().unwrap();
                    print!("</{}>", e);
                }
                Token::ElemAttr(k, vo) => {
                    if !unclosed_opener { panic!("attribute token outside of element") }

                    match vo {
                        None => {}
                        Some(v) => print!(" {}='{}'", k, v)
                    }
                }
                Token::Content(v) => {
                    match v {
                        Content::Plain(s) => print!("{}", s),
                        Content::Markup(s) => print!("{}", s),
                    }
                }
            }
        }

        println!();
    }
}

impl<'a> Tokenizable for Region<'a> {
    fn tokenize(&self) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();

        for child in &self.children {
            for token in child.tokenize() {
                result.push(token);
            }
        }

        return result;
    }
}
