pub enum Token {
    ElemOpener(String),
    ElemCloser(String),
    ElemAttr(String, Option<String>),
    Content(Content),
}

pub trait Renderable {
    fn render(&self) -> Vec<Token>;
}

impl<'a> Renderable for Region<'a> {
    fn render(&self) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();

        for child in &self.children {
            for token in child.render() {
                result.push(token);
            }
        }

        return result;
    }
}

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

    pub fn hash(&mut self) -> u64 {
        let mut h = metrohash::MetroHash::default();
        for child in &self.children {
            child.hash(&mut h);
        }
        h.finish()
    }

    pub fn print(&self) {
        let mut in_opener = false;

        for token in self.render() {
            match token {
                Token::ElemAttr(_, _) => {}
                _ => {
                    if in_opener {
                        in_opener = false;
                        print!(">");
                    }
                }
            }

            match token {
                Token::ElemOpener(e) => {
                    print!("<{}", e);
                    in_opener = true;
                }
                Token::ElemCloser(e) => {
                    print!("</{}>", e);
                }
                Token::ElemAttr(k, vo) => {
                    if !in_opener { panic!("attribute token outside of element") }

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

    // pub fn iter(&self) -> impl Iterator<Item = &Token> {
    //     let mut result: Vec<Token> = Vec::new();
    //     for child in &self.children {
    //         for token in child.render() {
    //             result.push(token);
    //         }
    //     }
    //     return result.iter();
    // }
}

// impl<'a> Iterator for Region<'a> {
//     type Item = ();
//
//     fn next(&mut self) -> Option<Self::Item> {
//
//     }
// }

pub enum Content {
    Plain(String),
    Markup(String),
}

impl Clone for Content {
    fn clone(&self) -> Self {
        match self {
            Content::Plain(v) => Content::Plain(v.clone()),
            Content::Markup(v) => Content::Markup(v.clone()),
        }
    }
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
}

impl<'a> Renderable for Node<'a> {
    fn render(&self) -> Vec<Token> {
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
                    for token in child.render() {
                        result.push(token);
                    }
                }

                result.push(Token::ElemCloser(e.name.to_owned()));

                result
            }
            Node::Content(v) => vec![Token::Content(v.clone())],
        }
    }
}

pub struct Elem<'a> {
    name: &'a str,
    attrs: Vec<Attr<'a>>,
    children: Vec<Node<'a>>,
}

impl<'a> Elem<'a> {
    pub fn new(t: &'a str) -> Elem<'a> {
        Elem {
            name: t,
            attrs: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn attributes(&self) -> &Vec<Attr> {
        &self.attrs
    }

    pub fn push_attr(&mut self, val: Attr<'a>) {
        self.attrs.push(val);
    }

    pub fn push_child(&mut self, et: Node<'a>) {
        self.children.push(et);
    }

    fn attrs_string(&self) -> String {
        let mut result: Vec<String> = Vec::new();

        for attr in &self.attrs {
            result.push(attr.serialize());
        }

        return result.join(" ");
    }

    fn hash(&self, h: &mut MetroHash) {
        self.name.hash(h);
        for attr in &self.attrs {
            attr.hash(h);
        }

        for child in &self.children {
            child.hash(h);
        }
    }
}

impl<'a> Into<Node<'a>> for Elem<'a> {
    fn into(self) -> Node<'a> {
        Node::Elem(self)
    }
}

pub enum Attr<'a> {
    String(&'a str, &'a str),
    Bool(&'a str, bool),
}

impl<'a> Attr<'a> {
    pub fn name(&self) -> &str {
        match self {
            Attr::String(k, _) => k,
            Attr::Bool(k, _) => k,
        }
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

    pub fn hash(&self, h: &mut MetroHash) {
        match self {
            Attr::String(k, v) => {
                (**k).hash(h);
                (**v).hash(h);
            }
            Attr::Bool(k, v) => {
                (**k).hash(h);
                (*v).hash(h);
            }
        }
    }
}

#[allow(non_upper_case_globals, unused)]
pub mod elem_types {
    pub const html: &str = "html";
    pub const base: &str = "base";
    pub const head: &str = "head";
    pub const body: &str = "body";
    pub const style: &str = "style";
    pub const title: &str = "title";
    pub const address: &str = "address";
    pub const article: &str = "article";
    pub const footer: &str = "footer";
    pub const header: &str = "header";
    pub const h1: &str = "h1";
    pub const h2: &str = "h2";
    pub const h3: &str = "h3";
    pub const h4: &str = "h4";
    pub const h5: &str = "h5";
    pub const h6: &str = "h6";
    pub const hgroup: &str = "hgroup";
    pub const nav: &str = "nav";
    pub const section: &str = "section";
    pub const dd: &str = "dd";
    pub const div: &str = "div";
    pub const dl: &str = "dl";
    pub const dt: &str = "dt";
    pub const figcaption: &str = "figcaption";
    pub const figure: &str = "figure";
    pub const hr: &str = "hr";
    pub const li: &str = "li";
    pub const main: &str = "main";
    pub const ol: &str = "ol";
    pub const p: &str = "p";
    pub const pre: &str = "pre";
    pub const ul: &str = "ul";
    pub const abbr: &str = "abbr";
    pub const b: &str = "b";
    pub const bdi: &str = "bdi";
    pub const bdo: &str = "bdo";
    pub const br: &str = "br";
    pub const cite: &str = "cite";
    pub const code: &str = "code";
    pub const data: &str = "data";
    pub const dfn: &str = "dfn";
    pub const em: &str = "em";
    pub const i: &str = "i";
    pub const kbd: &str = "kbd";
    pub const mark: &str = "mark";
    pub const q: &str = "q";
    pub const rp: &str = "rp";
    pub const rt: &str = "rt";
    pub const rtc: &str = "rtc";
    pub const ruby: &str = "ruby";
    pub const s: &str = "s";
    pub const samp: &str = "samp";
    pub const small: &str = "small";
    pub const span: &str = "span";
    pub const strong: &str = "strong";
    pub const sub: &str = "sub";
    pub const sup: &str = "sup";
    pub const time: &str = "time";
    pub const u: &str = "u";
    pub const var: &str = "var";
    pub const wbr: &str = "wbr";
    pub const area: &str = "area";
    pub const audio: &str = "audio";
    pub const map: &str = "map";
    pub const track: &str = "track";
    pub const video: &str = "video";
    pub const embed: &str = "embed";
    pub const object: &str = "object";
    pub const param: &str = "param";
    pub const source: &str = "source";
    pub const canvas: &str = "canvas";
    pub const noscript: &str = "noscript";
    pub const script: &str = "script";
    pub const del: &str = "del";
    pub const ins: &str = "ins";
    pub const caption: &str = "caption";
    pub const col: &str = "col";
    pub const colgroup: &str = "colgroup";
    pub const table: &str = "table";
    pub const tbody: &str = "tbody";
    pub const td: &str = "td";
    pub const tfoot: &str = "tfoot";
    pub const th: &str = "th";
    pub const thead: &str = "thead";
    pub const tr: &str = "tr";
    pub const button: &str = "button";
    pub const datalist: &str = "datalist";
    pub const fieldset: &str = "fieldset";
    pub const form: &str = "form";
    pub const input: &str = "input";
    pub const keygen: &str = "keygen";
    pub const label: &str = "label";
    pub const legend: &str = "legend";
    pub const meter: &str = "meter";
    pub const optgroup: &str = "optgroup";
    pub const option: &str = "option";
    pub const output: &str = "output";
    pub const progress: &str = "progress";
    pub const select: &str = "select";
    pub const details: &str = "details";
    pub const dialog: &str = "dialog";
    pub const menu: &str = "menu";
    pub const menuitem: &str = "menuitem";
    pub const summary: &str = "summary";
    pub const content: &str = "content";
    pub const element: &str = "element";
    pub const shadow: &str = "shadow";
    pub const template: &str = "template";
    pub const acronym: &str = "acronym";
    pub const applet: &str = "applet";
    pub const basefont: &str = "basefont";
    pub const big: &str = "big";
    pub const blink: &str = "blink";
    pub const center: &str = "center";
    pub const dir: &str = "dir";
    pub const frame: &str = "frame";
    pub const frameset: &str = "frameset";
    pub const isindex: &str = "isindex";
    pub const listing: &str = "listing";
    pub const noembed: &str = "noembed";
    pub const plaintext: &str = "plaintext";
    pub const spacer: &str = "spacer";
    pub const strike: &str = "strike";
    pub const tt: &str = "tt";
    pub const xmp: &str = "xmp";
}
