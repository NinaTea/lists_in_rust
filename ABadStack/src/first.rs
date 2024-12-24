use std::mem;
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
    next: Link,
}

// to associate actual code with a type

impl List {
    // Self referst to the type next to impl
    pub fn new() -> Self {
        return List { head: Link::Empty };
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // pattern matching para devolver un None o un some
        // y usamos replace porque tenemos un mut self
        // y tenemos que devolver la lista modificada
        let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        };
        result
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

mod test {
    #[test]
    fn basics() {
        use super::List;
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
