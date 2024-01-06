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
    ADD(u32, String),
    DELETE(u32),
    CHANGE(u32, String),
}

struct Diff {
    operations: Vec<Operation>,
}

impl Diff {
    fn apply(&self, before: &str) -> String {
        let mut lines: Vec<Vec<&str>> = vec![];
        lines.push(vec![]);

        for line in before.lines() {
            lines.push(vec![line]);
        }

        for line in self.operations.into_iter_mut() {
            match line.op {
                Operation::ADD => todo!(),
                Operation::DELETE => todo!(),
                Operation::CHANGE => todo!(),
            }
        }

        let mut result = String::new();

        for arr in lines {
            for line in arr {
                result += line;
            }
        }

        result
    }
}

impl Mode for Diff {}

struct New {}

impl Mode for New {}

struct Delete {}

impl Mode for Delete {}
