pub struct DOMBuilder<'a> {
    children: Vec<Node<'a>>,

    open_elems: Vec<usize>,
}

struct DOMLevel<'a> {
    value: Node<'a>,
    parent: Option<Node<'a>>,
}

impl<'a> DOMLevel<'a> {
    fn new(n: Node<'a>) -> DOMLevel<'a> {
        DOMLevel {
            value: n,
            parent: None,
        }
    }
}

impl<'a> DOMBuilder<'a> {
    fn get_top_node(&mut self) -> &mut Node<'a> {
        let mut last = *self.open_elems.last().unwrap();

        self.children.get_mut(last).unwrap()
    }

    fn get_top_node_elem(&mut self) -> &mut Elem<'a> {
        match self.get_top_node() {
            Node::Elem(e) => e,
            _ => panic!("unclosed element at top of stack was not a Elem"),
        }
    }

    pub fn open_element(&mut self, t: &'a str) {
        let elem = Elem {
            name: t,
            attrs: Vec::new(),
            children: Vec::new(),
        };
        let mut node = Node::Elem(elem);

        let mut last = self.open_elems.last();

        match last {
            None => {
                println!("pushing first child");
                self.open_elems.push(0);
                self.children.push(node);
            }
            Some(i) => {
                println!("pushing {} child", i);
                let cur = self.children.get_mut(*i);

                match cur {
                    None => panic!("current node index does not exist"),
                    Some(n) => {
                        match n {
                            Node::Elem(e) => {
                                e.push_child(node);
                            }
                            _ => panic!("current node was not an Elem")
                        }
                    }
                }
            }
        }
    }

    pub fn add_attribute_str(&mut self, k: &'a str, v: &'a str) {
        self.get_top_node_elem().push_attr(Attr::String(k, v));
    }

    pub fn add_attribute_bool(&mut self, k: &'a str, v: bool) {
        self.get_top_node_elem().push_attr(Attr::Bool(k, v));
    }

    pub fn add_content(&mut self, c: Content) {
        self.get_top_node_elem().push_child(Node::Content(c));
    }

    pub fn close_element(&mut self) {
        match self.open_elems.pop() {
            None => panic!("attempted to close element, but there are no unopened elements on the stack"),
            Some(_) => {}
        }
    }

    pub fn build(&mut self) -> Region<'a> {
        if self.open_elems.len() != 0 {
            panic!("attempted to build with unclosed elements");
        }

        let mut n: Vec<Node<'a>> = Vec::new();

        let mut result = Region::new(Vec::new());

        for child in self.children.drain(..) {
            result.push_child(child);
        }

        return result;
    }
}

impl<'a> Default for DOMBuilder<'a> {
    fn default() -> DOMBuilder<'a> {
        return DOMBuilder {
            children: Vec::new(),
            open_elems: Vec::new(),
        };
    }
}
