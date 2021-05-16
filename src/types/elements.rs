pub struct Elem {
    name: String,
    attrs: Vec<Attr>,
    children: Vec<NodeValue>,
}

impl Elem {
    pub fn new(t: &str) -> Elem {
        Elem {
            name: t.to_owned(),
            attrs: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn attributes(&self) -> &Vec<Attr> {
        &self.attrs
    }

    pub fn push_attr(&mut self, val: Attr) {
        self.attrs.push(val);
    }

    pub fn push_child(&mut self, et: NodeValue) {
        self.children.push(et);
    }

    // fn attrs_string(&self) -> String {
    //     let mut result: Vec<String> = Vec::new();
    //
    //     for attr in &self.attrs {
    //         result.push(attr.serialize());
    //     }
    //
    //     return result.join(" ");
    // }

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

impl Into<NodeValue> for Elem {
    fn into(self) -> NodeValue {
        NodeValue::Elem(self)
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
