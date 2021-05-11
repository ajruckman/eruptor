use crate::types::{Elem, Region, Renderable, Token};

mod types;

fn main() {
    println!("Hello, world!");

    let mut e = Elem::new(types::elem_types::body);

    let mut p = Elem::new("p");
    p.push_attr(types::Attr::String("asdf", "asdf"));
    e.push_child(p.into());

    let mut region = Region::new(vec![e.into()]);

    println!("{}", region.hash());
    for token in region.render() {
        match token {
            Token::HTML(v) => println!("{}", v),
            Token::String(v) => println!("{}", v),
        }
    }
    println!("{}", region.hash());

    region.push_child(Elem::new("p").into());
    println!("{}", region.hash());
}
