#![feature(raw_ref_op)]

use crate::dom2::*;

mod dom2;

fn main() {
    println!("-----");

    let mut builder = Builder::new();

    builder.open_elem("html");
    builder.open_elem("head");
    builder.open_elem("script");
    builder.close_elem();
    builder.close_elem();
    builder.open_elem("body");
    builder.open_elem("section");
    builder.add_attr(Attr::str("class", "eruptor-root"));
    builder.open_elem("div");
    builder.open_elem("section");
    builder.close_elem();
    builder.close_elem();
    builder.open_elem("div");
    builder.close_elem();
    builder.close_elem();
    builder.close_elem();
    builder.close_elem();

    let tree = builder.build();

    let tokens = tree.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }

    let x = tree.get_idx(0);

    //

    // let mut builder = DOMBuilder::new();
    //
    // builder.open_element(types::elem_types::body);
    // builder.close_element();
    //
    // builder.build();

    // builder.add_attribute_str("qwer", "asdf");
    // builder.add_attribute_bool("2", true);
    // builder.add_attribute_bool("3", false);
    //
    // builder.add_content(Content::Plain("hello world".to_owned()));
    //
    // // builder.open_element(types::elem_types::div);
    // // builder.add_content(Content::Plain("asdf".to_owned()));
    // // builder.close_element();
    //
    // builder.close_element();
    //
    // let r = builder.build();
    //
    // r.print();

    return;



    //

    // let mut e = Elem::new(types::elem_types::body);
    //
    // let mut p = Elem::new("p");
    // // p.push_attr(types::Attr::String("asdf", "asdf"));
    // p.push_child(types::NodeValue::Content(Content::Plain("Paragraph content".to_owned())));
    // e.push_child(p.into());
    //
    // let mut region = Region::new(vec![e.into()]);
    //
    // println!("{}", region.hash_code());
    //
    // region.print();
    //
    // println!("{}", region.hash_code());
    //
    // region.push_child(Elem::new("p").into());
    // println!("{}", region.hash_code());
}
