// pub struct ElemRoot<'a> {
//     children: Vec<Elem<'a>>,
// }
//
// impl<'a> ElemRoot<'a> {
//     pub fn new() -> ElemRoot {
//         ElemRoot {
//             children: Vec::new(),
//         }
//     }
//     pub fn append(&mut self, et: Elem) {
//         self.children.push(et);
//     }
// }

pub enum Token {
    HTML(String),
    String(String),
}

pub trait Renderable {
    fn render(&self) -> Vec<Token>;
    // fn hash(&self, h: &mut metrohash::MetroHash);
}

// pub struct Document {
//     values: Vec<String>,
// }
//
// impl Write for Document {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
//         todo!()
//     }
//
//     fn flush(&mut self) -> Result<(), std::io::Error> {
//         todo!()
//     }
// }

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

    hash: Option<u64>,
}

impl<'a> Region<'a> {
    pub fn new(children: Vec<Node<'a>>) -> Region<'a> {
        Region {
            children,
            hash: None,
        }
    }

    pub fn push_child(&mut self, et: Node<'a>) {
        self.children.push(et);
        // self.invalidate();
    }

    pub fn hash(&mut self) -> u64 {
        let mut h = metrohash::MetroHash::default();
        for child in &self.children {
            child.hash(&mut h);
        }
        h.finish()
        // match self.hash {
        //     None => {
        //
        //     }
        //     Some(v) => v
        // }
    }

    // pub fn invalidate(&mut self) {
    //     self.hash = None;
    // }
}

pub enum Content {
    Plain(String),
    Markup(String),
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

                if e.attrs.len() == 0 {
                    result.push(Token::HTML(format_args!("<{}>", e.name).to_string()));
                } else {
                    result.push(Token::HTML(format_args!("<{} {}>", e.name, e.attrs_string()).to_string()));
                }

                for child in &e.children {
                    for token in child.render() {
                        result.push(token);
                    }
                }

                result.push(Token::HTML(format_args!("</{}>", e.name).to_string()));

                result
            }
            Node::Content(v) => {
                match v {
                    Content::Plain(c) => vec!(Token::String(c.clone())),
                    Content::Markup(c) => vec!(Token::String(c.clone())),
                }
            }
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

// pub trait Attr<'a> {
//     fn name(&self) -> &'a str;
//     fn serialize(&self) -> String;
// }
//
// pub struct StringAttr<'a> {
//     name: &'a str,
//     value: &'a str,
// }
//
// impl<'a> StringAttr<'a> {
//     pub fn new(name: &'a str, value: &'a str) -> StringAttr<'a> {
//         StringAttr {
//             name,
//             value,
//         }
//     }
// }
//
// impl<'a> Attr<'a> for StringAttr<'a> {
//     fn name(&self) -> &'a str {
//         self.name
//     }
//
//     fn serialize(&self) -> String {
//         format!("\"{}\"", self.value.replace("\"", "&quot;")).to_string()
//     }
// }
//
// impl<'a> Into<Box<dyn Attr<'a>>> for (&'a str, &'a str) {
//     fn into(self) -> Box<dyn Attr<'a>> {
//         Box::new(StringAttr::new(self.0, self.1))
//     }
// }

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

// #[allow(non_camel_case_types)]
// #[derive(Display)]
// pub enum ElemType {
//     html,
//     base,
//     head,
//     body,
//     style,
//     title,
//     address,
//     article,
//     footer,
//     header,
//     h1,
//     h2,
//     h3,
//     h4,
//     h5,
//     h6,
//     hgroup,
//     nav,
//     section,
//     dd,
//     div,
//     dl,
//     dt,
//     figcaption,
//     figure,
//     hr,
//     li,
//     main,
//     ol,
//     p,
//     pre,
//     ul,
//     abbr,
//     b,
//     bdi,
//     bdo,
//     br,
//     cite,
//     code,
//     data,
//     dfn,
//     em,
//     i,
//     kbd,
//     mark,
//     q,
//     rp,
//     rt,
//     rtc,
//     ruby,
//     s,
//     samp,
//     small,
//     span,
//     strong,
//     sub,
//     sup,
//     time,
//     u,
//     var,
//     wbr,
//     area,
//     audio,
//     map,
//     track,
//     video,
//     embed,
//     object,
//     param,
//     source,
//     canvas,
//     noscript,
//     script,
//     del,
//     ins,
//     caption,
//     col,
//     colgroup,
//     table,
//     tbody,
//     td,
//     tfoot,
//     th,
//     thead,
//     tr,
//     button,
//     datalist,
//     fieldset,
//     form,
//     input,
//     keygen,
//     label,
//     legend,
//     meter,
//     optgroup,
//     option,
//     output,
//     progress,
//     select,
//     details,
//     dialog,
//     menu,
//     menuitem,
//     summary,
//     content,
//     element,
//     shadow,
//     template,
//     acronym,
//     applet,
//     basefont,
//     big,
//     blink,
//     center,
//     dir,
//     frame,
//     frameset,
//     isindex,
//     listing,
//     noembed,
//     plaintext,
//     spacer,
//     strike,
//     tt,
//     xmp,
// }
//
// // impl<'a> Elem<'a> {
// //     pub fn data(&self) -> &mut Elem<'a> {
// //         let r: &&mut Elem = match self {
// //             Elem::ROOT(d) => d,
// //             Elem::html(d) => d,
// //             Elem::base(d) => d,
// //             Elem::head(d) => d,
// //             Elem::body(d) => d,
// //             Elem::style(d) => d,
// //             Elem::title(d) => d,
// //             Elem::address(d) => d,
// //             Elem::article(d) => d,
// //             Elem::footer(d) => d,
// //             Elem::header(d) => d,
// //             Elem::h1(d) => d,
// //             Elem::h2(d) => d,
// //             Elem::h3(d) => d,
// //             Elem::h4(d) => d,
// //             Elem::h5(d) => d,
// //             Elem::h6(d) => d,
// //             Elem::hgroup(d) => d,
// //             Elem::nav(d) => d,
// //             Elem::section(d) => d,
// //             Elem::dd(d) => d,
// //             Elem::div(d) => d,
// //             Elem::dl(d) => d,
// //             Elem::dt(d) => d,
// //             Elem::figcaption(d) => d,
// //             Elem::figure(d) => d,
// //             Elem::hr(d) => d,
// //             Elem::li(d) => d,
// //             Elem::main(d) => d,
// //             Elem::ol(d) => d,
// //             Elem::p(d) => d,
// //             Elem::pre(d) => d,
// //             Elem::ul(d) => d,
// //             Elem::abbr(d) => d,
// //             Elem::b(d) => d,
// //             Elem::bdi(d) => d,
// //             Elem::bdo(d) => d,
// //             Elem::br(d) => d,
// //             Elem::cite(d) => d,
// //             Elem::code(d) => d,
// //             Elem::data(d) => d,
// //             Elem::dfn(d) => d,
// //             Elem::em(d) => d,
// //             Elem::i(d) => d,
// //             Elem::kbd(d) => d,
// //             Elem::mark(d) => d,
// //             Elem::q(d) => d,
// //             Elem::rp(d) => d,
// //             Elem::rt(d) => d,
// //             Elem::rtc(d) => d,
// //             Elem::ruby(d) => d,
// //             Elem::s(d) => d,
// //             Elem::samp(d) => d,
// //             Elem::small(d) => d,
// //             Elem::span(d) => d,
// //             Elem::strong(d) => d,
// //             Elem::sub(d) => d,
// //             Elem::sup(d) => d,
// //             Elem::time(d) => d,
// //             Elem::u(d) => d,
// //             Elem::var(d) => d,
// //             Elem::wbr(d) => d,
// //             Elem::area(d) => d,
// //             Elem::audio(d) => d,
// //             Elem::map(d) => d,
// //             Elem::track(d) => d,
// //             Elem::video(d) => d,
// //             Elem::embed(d) => d,
// //             Elem::object(d) => d,
// //             Elem::param(d) => d,
// //             Elem::source(d) => d,
// //             Elem::canvas(d) => d,
// //             Elem::noscript(d) => d,
// //             Elem::script(d) => d,
// //             Elem::del(d) => d,
// //             Elem::ins(d) => d,
// //             Elem::caption(d) => d,
// //             Elem::col(d) => d,
// //             Elem::colgroup(d) => d,
// //             Elem::table(d) => d,
// //             Elem::tbody(d) => d,
// //             Elem::td(d) => d,
// //             Elem::tfoot(d) => d,
// //             Elem::th(d) => d,
// //             Elem::thead(d) => d,
// //             Elem::tr(d) => d,
// //             Elem::button(d) => d,
// //             Elem::datalist(d) => d,
// //             Elem::fieldset(d) => d,
// //             Elem::form(d) => d,
// //             Elem::input(d) => d,
// //             Elem::keygen(d) => d,
// //             Elem::label(d) => d,
// //             Elem::legend(d) => d,
// //             Elem::meter(d) => d,
// //             Elem::optgroup(d) => d,
// //             Elem::option(d) => d,
// //             Elem::output(d) => d,
// //             Elem::progress(d) => d,
// //             Elem::select(d) => d,
// //             Elem::details(d) => d,
// //             Elem::dialog(d) => d,
// //             Elem::menu(d) => d,
// //             Elem::menuitem(d) => d,
// //             Elem::summary(d) => d,
// //             Elem::content(d) => d,
// //             Elem::element(d) => d,
// //             Elem::shadow(d) => d,
// //             Elem::template(d) => d,
// //             Elem::acronym(d) => d,
// //             Elem::applet(d) => d,
// //             Elem::basefont(d) => d,
// //             Elem::big(d) => d,
// //             Elem::blink(d) => d,
// //             Elem::center(d) => d,
// //             Elem::dir(d) => d,
// //             Elem::frame(d) => d,
// //             Elem::frameset(d) => d,
// //             Elem::isindex(d) => d,
// //             Elem::listing(d) => d,
// //             Elem::noembed(d) => d,
// //             Elem::plaintext(d) => d,
// //             Elem::spacer(d) => d,
// //             Elem::strike(d) => d,
// //             Elem::tt(d) => d,
// //             Elem::xmp(d) => d,
// //         };
// //     }
// // }
//
// // pub enum Node<'a> {
// //     Elem(Elem<'a>),
// //     // Other(String, ElemData<'a>),
// //     Content(String),
// // }
// //
// // // #[allow(non_camel_case_types)]
// // // pub enum Elem<'a> {
// // //     ROOT(&'a mut ElemData<'a>),
// // //     html(&'a mut ElemData<'a>),
// // //     base(&'a mut ElemData<'a>),
// // //     head(&'a mut ElemData<'a>),
// // //     body(&'a mut ElemData<'a>),
// // //     style(&'a mut ElemData<'a>),
// // //     title(&'a mut ElemData<'a>),
// // //     address(&'a mut ElemData<'a>),
// // //     article(&'a mut ElemData<'a>),
// // //     footer(&'a mut ElemData<'a>),
// // //     header(&'a mut ElemData<'a>),
// // //     h1(&'a mut ElemData<'a>),
// // //     h2(&'a mut ElemData<'a>),
// // //     h3(&'a mut ElemData<'a>),
// // //     h4(&'a mut ElemData<'a>),
// // //     h5(&'a mut ElemData<'a>),
// // //     h6(&'a mut ElemData<'a>),
// // //     hgroup(&'a mut ElemData<'a>),
// // //     nav(&'a mut ElemData<'a>),
// // //     section(&'a mut ElemData<'a>),
// // //     dd(&'a mut ElemData<'a>),
// // //     div(&'a mut ElemData<'a>),
// // //     dl(&'a mut ElemData<'a>),
// // //     dt(&'a mut ElemData<'a>),
// // //     figcaption(&'a mut ElemData<'a>),
// // //     figure(&'a mut ElemData<'a>),
// // //     hr(&'a mut ElemData<'a>),
// // //     li(&'a mut ElemData<'a>),
// // //     main(&'a mut ElemData<'a>),
// // //     ol(&'a mut ElemData<'a>),
// // //     p(&'a mut ElemData<'a>),
// // //     pre(&'a mut ElemData<'a>),
// // //     ul(&'a mut ElemData<'a>),
// // //     abbr(&'a mut ElemData<'a>),
// // //     b(&'a mut ElemData<'a>),
// // //     bdi(&'a mut ElemData<'a>),
// // //     bdo(&'a mut ElemData<'a>),
// // //     br(&'a mut ElemData<'a>),
// // //     cite(&'a mut ElemData<'a>),
// // //     code(&'a mut ElemData<'a>),
// // //     data(&'a mut ElemData<'a>),
// // //     dfn(&'a mut ElemData<'a>),
// // //     em(&'a mut ElemData<'a>),
// // //     i(&'a mut ElemData<'a>),
// // //     kbd(&'a mut ElemData<'a>),
// // //     mark(&'a mut ElemData<'a>),
// // //     q(&'a mut ElemData<'a>),
// // //     rp(&'a mut ElemData<'a>),
// // //     rt(&'a mut ElemData<'a>),
// // //     rtc(&'a mut ElemData<'a>),
// // //     ruby(&'a mut ElemData<'a>),
// // //     s(&'a mut ElemData<'a>),
// // //     samp(&'a mut ElemData<'a>),
// // //     small(&'a mut ElemData<'a>),
// // //     span(&'a mut ElemData<'a>),
// // //     strong(&'a mut ElemData<'a>),
// // //     sub(&'a mut ElemData<'a>),
// // //     sup(&'a mut ElemData<'a>),
// // //     time(&'a mut ElemData<'a>),
// // //     u(&'a mut ElemData<'a>),
// // //     var(&'a mut ElemData<'a>),
// // //     wbr(&'a mut ElemData<'a>),
// // //     area(&'a mut ElemData<'a>),
// // //     audio(&'a mut ElemData<'a>),
// // //     map(&'a mut ElemData<'a>),
// // //     track(&'a mut ElemData<'a>),
// // //     video(&'a mut ElemData<'a>),
// // //     embed(&'a mut ElemData<'a>),
// // //     object(&'a mut ElemData<'a>),
// // //     param(&'a mut ElemData<'a>),
// // //     source(&'a mut ElemData<'a>),
// // //     canvas(&'a mut ElemData<'a>),
// // //     noscript(&'a mut ElemData<'a>),
// // //     script(&'a mut ElemData<'a>),
// // //     del(&'a mut ElemData<'a>),
// // //     ins(&'a mut ElemData<'a>),
// // //     caption(&'a mut ElemData<'a>),
// // //     col(&'a mut ElemData<'a>),
// // //     colgroup(&'a mut ElemData<'a>),
// // //     table(&'a mut ElemData<'a>),
// // //     tbody(&'a mut ElemData<'a>),
// // //     td(&'a mut ElemData<'a>),
// // //     tfoot(&'a mut ElemData<'a>),
// // //     th(&'a mut ElemData<'a>),
// // //     thead(&'a mut ElemData<'a>),
// // //     tr(&'a mut ElemData<'a>),
// // //     button(&'a mut ElemData<'a>),
// // //     datalist(&'a mut ElemData<'a>),
// // //     fieldset(&'a mut ElemData<'a>),
// // //     form(&'a mut ElemData<'a>),
// // //     input(&'a mut ElemData<'a>),
// // //     keygen(&'a mut ElemData<'a>),
// // //     label(&'a mut ElemData<'a>),
// // //     legend(&'a mut ElemData<'a>),
// // //     meter(&'a mut ElemData<'a>),
// // //     optgroup(&'a mut ElemData<'a>),
// // //     option(&'a mut ElemData<'a>),
// // //     output(&'a mut ElemData<'a>),
// // //     progress(&'a mut ElemData<'a>),
// // //     select(&'a mut ElemData<'a>),
// // //     details(&'a mut ElemData<'a>),
// // //     dialog(&'a mut ElemData<'a>),
// // //     menu(&'a mut ElemData<'a>),
// // //     menuitem(&'a mut ElemData<'a>),
// // //     summary(&'a mut ElemData<'a>),
// // //     content(&'a mut ElemData<'a>),
// // //     element(&'a mut ElemData<'a>),
// // //     shadow(&'a mut ElemData<'a>),
// // //     template(&'a mut ElemData<'a>),
// // //     acronym(&'a mut ElemData<'a>),
// // //     applet(&'a mut ElemData<'a>),
// // //     basefont(&'a mut ElemData<'a>),
// // //     big(&'a mut ElemData<'a>),
// // //     blink(&'a mut ElemData<'a>),
// // //     center(&'a mut ElemData<'a>),
// // //     dir(&'a mut ElemData<'a>),
// // //     frame(&'a mut ElemData<'a>),
// // //     frameset(&'a mut ElemData<'a>),
// // //     isindex(&'a mut ElemData<'a>),
// // //     listing(&'a mut ElemData<'a>),
// // //     noembed(&'a mut ElemData<'a>),
// // //     plaintext(&'a mut ElemData<'a>),
// // //     spacer(&'a mut ElemData<'a>),
// // //     strike(&'a mut ElemData<'a>),
// // //     tt(&'a mut ElemData<'a>),
// // //     xmp(&'a mut ElemData<'a>),
// // // }
