use std::{cell::RefCell, rc::Rc};

struct Log {
    changes: Vec<Box<dyn Mode>>,
}

struct Tree {
    refs: Vec<TreeRef>,
}

impl Tree {
    pub fn generate_string(&self) -> String {
        String::new()
    }

    pub fn new() -> Self {
        Tree { refs: vec![] }
    }
}

type TreeRef = Rc<RefCell<Tree>>;

trait Mode {}

enum Operation {
    ADD,
    DELETE,
    CHANGE,
}

struct LineOp {
    op: Operation,
    line: u32,
    string: String,
}

impl LineOp {
    fn new(op: Operation, line: u32, string: String) -> Self {
        LineOp {
            op: op,
            line: line,
            string: string,
        }
    }
}

struct Diff {
    operations: Vec<LineOp>,
}

impl Diff {
    fn apply(before: &String) -> String {}
}

impl Mode for Diff {}

struct New {}

impl Mode for New {}

struct Delete {}

impl Mode for Delete {}
