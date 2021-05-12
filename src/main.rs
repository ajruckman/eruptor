use std::time::Instant;

use crate::types::{Elem, Region, Renderable, Token, Content};

mod types;

fn main() {
    println!("Hello, world!");

    let mut e = Elem::new(types::elem_types::body);

    let mut p = Elem::new("p");
    p.push_attr(types::Attr::String("asdf", "asdf"));
    p.push_child(types::Node::Content(Content::Plain("Paragraph content".to_owned())));
    e.push_child(p.into());

    let mut region = Region::new(vec![e.into()]);

    println!("{}", region.hash());

    region.print();

    println!("{}", region.hash());

    region.push_child(Elem::new("p").into());
    println!("{}", region.hash());
}
