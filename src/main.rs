use crate::types::{Elem, Region, Content, DOMBuilder};

mod types;

fn main() {
    println!("Hello, world!");

    //

    let mut builder = DOMBuilder::default();

    builder.open_element(types::elem_types::body);
    builder.add_attribute_str("qwer", "asdf");
    builder.add_attribute_bool("2", true);
    builder.add_attribute_bool("3", false);

    builder.add_content(Content::Plain("hello world".to_owned()));

    // builder.open_element(types::elem_types::div);
    // builder.add_content(Content::Plain("asdf".to_owned()));
    // builder.close_element();

    builder.close_element();

    let r = builder.build();

    r.print();

    return;



    //

    let mut e = Elem::new(types::elem_types::body);

    let mut p = Elem::new("p");
    p.push_attr(types::Attr::String("asdf", "asdf"));
    p.push_child(types::Node::Content(Content::Plain("Paragraph content".to_owned())));
    e.push_child(p.into());

    let mut region = Region::new(vec![e.into()]);

    println!("{}", region.hash_code());

    region.print();

    println!("{}", region.hash_code());

    region.push_child(Elem::new("p").into());
    println!("{}", region.hash_code());
}
