pub struct Builder {
    tree: NodeTree,
    idx_stack: Vec<usize>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            tree: NodeTree::new(),
            idx_stack: Vec::new(),
        }
    }

    fn current_idx(&self) -> usize {
        *self.idx_stack.last().unwrap_or(&usize::MAX)
    }

    fn push_idx(&mut self, idx: usize) {
        self.idx_stack.push(idx);
    }

    fn pop_idx(&mut self) {
        self.idx_stack.pop().unwrap();
    }

    pub fn open_elem(&mut self, t: &str) {
        let elem = NodeValue::Elem(Elem {
            name: t.to_string(),
            attrs: Vec::new(),
        });

        let current = self.current_idx();

        let node_ref = self.tree.push_node(current, elem);

        match current {
            usize::MAX => {}
            v => {
                let parent = self.tree.get_idx_mut(current);
                parent.children.push(node_ref.index);
            }
        }

        self.push_idx(node_ref.index);

        // let node = self.tree.build_node(self.current_idx(), elem);
        //
        // let new = node.index;
        //
        // self.tree.push_child(node);
        //
        // self.push_idx(new);

        // let node = Node::new(elem);
        //
        // self.tree.push_child(elem);
    }

    pub fn close_elem(&mut self) {
        self.pop_idx();
    }

    pub fn add_attr(&mut self, attr: Attr) {
        let current = self.tree.get_idx_mut(self.current_idx());

        match &mut current.value {
            NodeValue::Elem(e) => e.attrs.push(attr),
            NodeValue::Content(_) => panic!("current node is not an Elem")
        }
    }

    pub fn build(&mut self) -> NodeTree {
        if self.idx_stack.len() > 0 {
            panic!("attempted to build tree with unclosed nodes")
        }
        self.idx_stack.clear();
        std::mem::replace(&mut self.tree, NodeTree::new())
    }
}

pub struct NodeTree {
    children: Vec<Node>,
    idx_lock: Mutex<u8>,
}

impl NodeTree {
    pub fn new() -> NodeTree {
        NodeTree {
            children: Vec::new(),
            idx_lock: Mutex::new(0),
        }
    }

    pub fn push_child(&mut self, c: Node) -> usize {
        self.children.push(c);
        self.children.len() - 1
    }

    pub fn get_idx(&self, i: usize) -> &Node {
        self.children.get(i).unwrap()
    }

    pub fn get_idx_mut(&mut self, i: usize) -> &mut Node {
        // println!("{}", i);
        self.children.get_mut(i).unwrap()
    }

    pub fn push_node(&mut self, parent: usize, value: NodeValue) -> NodeRef {
        let _ = self.idx_lock.lock().unwrap();

        let next = self.children.len();

        let node = Node::new(next, parent, value);

        self.children.push(node);

        NodeRef {
            index: next,
        }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut result = Vec::new();

        for child in &self.children {
            if child.parent == usize::MAX {
                for token in child.tokenize(self) {
                    result.push(token);
                }
            }
        }

        return result;
    }
}

pub struct Node {
    index: usize,
    parent: usize,
    value: NodeValue,
    children: Vec<usize>,
}

impl Node {
    fn new(index: usize, parent: usize, value: NodeValue) -> Node {
        Node {
            index: index,
            parent: parent,
            value: value,
            children: Vec::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct NodeRef {
    index: usize,
}

impl NodeRef {}

pub enum NodeValue {
    Elem(Elem),
    Content(Content),
}

pub struct Elem {
    name: String,
    attrs: Vec<Attr>,
}

#[derive(Debug)]
pub enum Content {
    Plain(String),
    Raw(String),
}

impl Clone for Content {
    fn clone(&self) -> Self {
        match self {
            Content::Plain(p) => Content::Plain(p.clone()),
            Content::Raw(r) => Content::Raw(r.clone()),
        }
    }
}

pub enum Attr {
    String(String, String),
    Bool(String, bool),
}

impl Attr {
    pub fn name(&self) -> &str {
        match self {
            Attr::String(k, _) => k,
            Attr::Bool(k, _) => k,
        }
    }

    pub fn str(k: &str, v: &str) -> Attr {
        Attr::String(k.to_owned(), v.to_owned())
    }

    pub fn bool(k: &str, v: bool) -> Attr {
        Attr::Bool(k.to_owned(), v)
    }

    pub fn serialize(&self) -> String {
        match self {
            Attr::String(k, v) => format!("{}='{}'", k, v.replace("\"", "&quot;")).to_owned(),
            Attr::Bool(k, v) => {
                if !v {
                    "".to_owned()
                } else {
                    format!("{}={}", k, k).to_owned()
                }
            }
        }
    }
}
