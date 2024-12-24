// concepto curioso: no uniformidad de guardado de datos en la lista

// Every enum has to store a tag to specify
// which variant of the enum its bits represent.

pub enum List0 {
    Empty,
    ElemThenEmpty(i32),
    ElemThenNotEmpty(i32, Box<List0>),
}

// We can have a special kind of enum that
// eliminates the space needed for the tag

// no compila por el containsanonnullptr
// enum Foo {
//     A,
//     B(ContainsANonNullPtr),
// }

// If the variant is A, the whole enum is set to all 0's. Otherwise, the variant is B.
// This works because B can never be all 0's, since it contains a non-zero pointer.

// este nullPointer optimization implica que
// &, &mut, Box, Rc, Arc, vec y otros no tienen overhead de almacenamiento
// when put in an Option

pub struct List {
    head: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}
struct Node {
    elem: i32,
    next: List,
}

// to associate actual code with a type

impl List {
    // Self referst to the type next to impl
    pub fn new() -> Self {
        return List { head: Link::Empty };
    }
}
