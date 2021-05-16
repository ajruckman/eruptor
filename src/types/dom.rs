pub enum Token {
    ElemOpener(String),
    ElemCloser,
    ElemAttr(String, Option<String>),
    Content(Content),
}

pub trait Tokenizable {
    fn tokenize(&self) -> Vec<Token>;
}

pub struct NodeTree {
    children: Vec<Arc<Mutex<Node>>>,
}

impl NodeTree {
    fn new() -> Arc<Mutex<NodeTree>> {
        let mut tree = NodeTree {
            children: Vec::new(),
        };

        let arc = Arc::new(Mutex::new(tree));

        let root = Arc::new(Mutex::new(Node {
            tree: arc.clone(),
            value: NodeValue::Root,
            index: 0,
            parent: None,
            children: Vec::new(),
        }));

        arc.lock().unwrap().children.push(root);

        arc.clone()
    }

    pub fn root(&self) -> Arc<Mutex<Node>> {
        self.children[0].clone()
    }

    pub fn push_child(&mut self, c: Node) -> usize {
        self.children.push(Arc::new(Mutex::new(c)));
        return self.children.len() - 1;
    }

    // pub fn push_new_child(&mut self, v: NodeValue) -> usize {
    //     let mut n = Node {
    //         tree: Arc::new(Mutex::new(self)),
    //         value: v,
    //         index: 0,
    //         parent: Some(self.index),
    //         children: Vec::new(),
    //     };
    //     let i = self.push_child(n);
    //     // n.index = i;
    //     return i;
    // }

    pub fn get_idx(&mut self, i: usize) -> MutexGuard<Node> {
        self.children.get_mut(i).unwrap().lock().unwrap()
    }
}

pub struct Node {
    tree: Arc<Mutex<NodeTree>>,
    value: NodeValue,
    index: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    // pub fn new(&self, v: NodeValue) -> Node {
    //     Node {
    //         tree: self.tree.clone(),
    //         value: v,
    //         index: 0,
    //         parent: None,
    //         children: vec![]
    //     }
    // }
    // fn get_children(&self) -> &Vec<Node> {
    //     &self.tree.lock().unwrap().children
    // }

    pub fn children(&mut self) -> Vec<Arc<Mutex<Node>>> {
        let mut result = Vec::new();
        let tree = &self.tree.lock().unwrap();

        for child in &self.children {
            result.push(tree.children[*child].clone());
        }

        return result;

        // let tree = self.tree.clone().as_ref().borrow();
        //
        // for child in &self.children {
        //     result.push(tree.get_idx(*child));
        //     // result.push(self.tree.children.get(*child).unwrap());
        // }
        //
        // return result;
    }

    pub fn parent(&mut self) -> Option<Arc<Mutex<Node>>> {
        match self.parent {
            None => None,
            Some(i) => {
                let tree = self.tree.lock().unwrap();
                return Some(tree.children.get(i).unwrap().clone());
            }
        }
    }

    pub fn push_child(&mut self, mut c: Node) -> usize {
        let mut tree = self.tree.lock().unwrap();

        let i =tree.push_child(c);
        tree.children.get_mut(i).unwrap().lock().unwrap().index = i;

        self.children.push(i);
        return i;
    }

    pub fn push_new_child(&mut self, v: NodeValue) -> usize {
        let mut n = Node {
            tree: self.tree.clone(),
            value: v,
            index: 0,
            parent: Some(self.index),
            children: Vec::new(),
        };
        let i = self.push_child(n);
        return i;
    }
}

pub enum NodeValue {
    Root,
    Elem(Elem),
    Content(Content),
}

impl NodeValue {
    fn hash(&self, h: &mut MetroHash) {
        match self {
            NodeValue::Elem(e) => e.hash(h),
            NodeValue::Content(c) => {
                match c {
                    Content::Plain(v) => (*v).hash(h),
                    Content::Markup(v) => (*v).hash(h),
                }
            }
            _ => {}
        }
    }

    fn elem_ref(&mut self) -> &mut Elem {
        match self {
            NodeValue::Elem(e) => e,
            _ => panic!("not an elem"),
        }
    }
}

impl<'a> Tokenizable for NodeValue {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            NodeValue::Elem(e) => {
                let mut result: Vec<Token> = Vec::new();

                result.push(Token::ElemOpener(e.name.to_owned()));

                for attr in &e.attrs {
                    match attr.borrow() {
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
            NodeValue::Content(v) => vec![Token::Content(v.clone())],
            root => Vec::new(),
        }
    }
}
