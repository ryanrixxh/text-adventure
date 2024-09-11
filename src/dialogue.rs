use std::{cell::RefCell, rc::Rc, vec};

#[derive(Debug, Clone)]
pub struct DialogueTree {
    root: Rc<DialogueNode>,
}

#[derive(Debug, Clone)]
pub struct DialogueNode {
    value: i32,
    children: RefCell<Vec<Rc<DialogueNode>>>,
}

impl DialogueNode {
    /// Recursive function that allows printing of the full tree
    fn print_self(&self) {
        let children = self.children.borrow();
        for child in children.iter() {
            println!("Child value: {}", child.value);
            child.print_self();
        }
    }

    fn add_child(&self, value: i32) {
        let child = Rc::new(DialogueNode {
            value,
            children: RefCell::new(vec![]),
        });

        self.children.borrow_mut().push(child);
    }

    // TODO: This theoretically should be able to stitch subtrees together, but this needs testing
    fn _add_subtree(&self, node: &Rc<DialogueNode>) {
        self.children.borrow_mut().push(Rc::clone(&node))
    }
}

pub fn build_tree() {
    let tree = DialogueTree {
        root: Rc::new(DialogueNode {
            value: 0,
            children: RefCell::new(vec![]),
        }),
    };

    tree.root.add_child(1);
    tree.root.add_child(2);
    tree.root.add_child(3);

    tree.root.print_self();
}
