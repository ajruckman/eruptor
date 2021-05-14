Design:
- `Region`: Set of `Elem`; divides groups of elements and can be ticked individually
- `Elem`: An HTML element with attributes and children like a `<div a='b'>...</div>`
- `Content`: A value inside an HTML element, and/or raw HTML to be inserted into the document without escaping
- `Token`: An instruction to serialize a part of the virtual DOM, for example to open or close a tag



References:
- https://github.com/chinedufn/percy/blob/master/crates/virtual-node/src/lib.rs
- https://github.com/fitzgen/dodrio/blob/master/src/node.rs
- https://chinedufn.github.io/percy/diff-patch/diff-patch-walkthrough/index.html
