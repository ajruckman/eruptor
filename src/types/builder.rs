pub trait TDOMBuilder {
    fn open_element(&mut self, t: &str);
    fn close_element(&mut self);
    fn add_content(&mut self, c: Content);
    fn add_attribute_str(&mut self, k: &str, v: &str);
    fn add_attribute_bool(&mut self, k: &str, v: bool);

    fn build(&mut self);
}

// pub enum DOMBuilderFrame {
//     OpenElement(String),
//     CloseElement,
//     AddContent(Content),
//     AddAttributeStr(String, String),
//     AddAttributeBool(String, bool),
// }

pub struct DOMBuilder {
    root: Arc<Mutex<NodeTree>>,

    current: usize,
}

impl<'a> DOMBuilder {
    pub fn new() -> DOMBuilder {
        let tree = NodeTree::new();
        // let root = tree.lock().unwrap().root();

        DOMBuilder {
            root: tree,
            current: 0,
        }
    }

    fn get(&self) -> MutexGuard<NodeTree> {
        self.root.lock().unwrap()
    }
}

impl TDOMBuilder for DOMBuilder {
    fn open_element(&mut self, t: &str) {
        let elem = NodeValue::Elem(Elem::new(t));
        let idx = self.get().get_idx(self.current).push_new_child(elem);
    }

    fn close_element(&mut self) {
        todo!();
    }

    fn add_content(&mut self, c: Content) {
        todo!();
    }

    fn add_attribute_str(&mut self, k: &str, v: &str) {
        todo!();
    }

    fn add_attribute_bool(&mut self, k: &str, v: bool) {
        todo!();
    }

    fn build(&mut self) {
        // let mut result: Vec<Rc<NodeValue>> = Vec::new();
        // let mut unclosed: Vec<Weak<NodeValue>> = Vec::new();
        // let mut current: Option<Weak<NodeValue>> = None;
        //
        // for frame in &self.frames {
        //     match frame {
        //         DOMBuilderFrame::OpenElement(t) => {
        //             let elem = Elem {
        //                 name: t,
        //                 attrs: Vec::new(),
        //                 children: Vec::new(),
        //             };
        //             let mut node = NodeValue::Elem(elem);
        //             let cell = Rc::new(node);
        //
        //             match current {
        //                 None => {
        //                     current = Some(Rc::downgrade(&cell));
        //                     result.push(cell);
        //                 }
        //                 Some(ref n) => {
        //                     n.upgrade().unwrap().elem_ref().push_child(node);
        //                 }
        //             }
        //
        //             // let mut cell = Rc::new(node);
        //             //
        //             // unclosed.push(cell.clone());
        //             //
        //             // match current.deref() {
        //             //     Node::Elem(e) => e.push_child(cell.clone()),
        //             //     Node::Content(_) => {}
        //             // }
        //         }
        //         DOMBuilderFrame::CloseElement => {}
        //         DOMBuilderFrame::AddContent(c) => {}
        //         DOMBuilderFrame::AddAttributeStr(k, v) => {}
        //         DOMBuilderFrame::AddAttributeBool(k, v) => {}
        //     }
        // }
    }
}

// pub struct DOMBuilder {
//     children: Vec<Rc<RefCell<Node>>>,
//     current: Option<Rc<RefCell<Node>>>,
//
//     // open_elems: Vec<usize>,
// }
//
// struct DOMLevel {
//     value: Node,
//     parent: Option<Node>,
// }
//
// impl DOMLevel {
//     fn new(n: Node) -> DOMLevel {
//         DOMLevel {
//             value: n,
//             parent: None,
//         }
//     }
// }
//
// impl DOMBuilder {
// fn get_top_node(&mut self) -> &mut Node {
//     let mut last = *self.open_elems.last().unwrap();
//
//     self.children.get_mut(last).unwrap()
// }
//
// fn get_top_node_elem(&mut self) -> &mut Elem {
//     match self.get_top_node() {
//         Node::Elem(e) => e,
//         _ => panic!("unclosed element at top of stack was not a Elem"),
//     }
// }
//
// pub fn open_element(&mut self, t: &'a str) {
//     let elem = Elem {
//         name: t,
//         attrs: Vec::new(),
//         children: Vec::new(),
//     };
//     let mut node = Node::Elem(elem);
//     let mut cell = RefCell::new(node);
//     let mut rc = Rc::new(cell);
//
//     if self.current.is_none() {
//         self.children.push(rc.clone());
//         self.current = Some(rc.clone());
//     } else {
//         match self.current.unwrap().get_mut() {
//             Node::Elem(e) => e.push_child(node),
//             Node::Content(_) => {}
//         }
//     }
// }

// pub fn open_element(&mut self, t: &'a str) {
//     let elem = Elem {
//         name: t,
//         attrs: Vec::new(),
//         children: Vec::new(),
//     };
//     let mut node = Node::Elem(elem);
//
//     let mut last = self.open_elems.last();
//
//     match last {
//         None => {
//             println!("pushing first child");
//             self.open_elems.push(0);
//             self.children.push(node);
//         }
//         Some(i) => {
//             println!("pushing {} child", i);
//             let cur = self.children.get_mut(*i);
//
//             match cur {
//                 None => panic!("current node index does not exist"),
//                 Some(n) => {
//                     match n {
//                         Node::Elem(e) => {
//                             e.push_child(node);
//                         }
//                         _ => panic!("current node was not an Elem")
//                     }
//                 }
//             }
//         }
//     }
// }

// pub fn add_attribute_str(&mut self, k: &'a str, v: &'a str) {
//     self.get_top_node_elem().push_attr(Attr::String(k, v));
// }
//
// pub fn add_attribute_bool(&mut self, k: &'a str, v: bool) {
//     self.get_top_node_elem().push_attr(Attr::Bool(k, v));
// }
//
// pub fn add_content(&mut self, c: Content) {
//     self.get_top_node_elem().push_child(Node::Content(c));
// }
//
// pub fn close_element(&mut self) {
//     match self.open_elems.pop() {
//         None => panic!("attempted to close element, but there are no unopened elements on the stack"),
//         Some(_) => {}
//     }
// }
//
// pub fn build(&mut self) -> Region {
//     if self.open_elems.len() != 0 {
//         panic!("attempted to build with unclosed elements");
//     }
//
//     let mut n: Vec<Node> = Vec::new();
//
//     let mut result = Region::new(Vec::new());
//
//     for child in self.children.drain(..) {
//         result.push_child(child);
//     }
//
//     return result;
// }
// }
//
// impl Default for DOMBuilder {
//     fn default() -> DOMBuilder {
//         return DOMBuilder {
//             children: Vec::new(),
//             current: None,
//         };
//     }
// }
