use std::{cell::RefCell, rc::{Rc, Weak}, vec};
// TODO: RefCell IS NOT THREAD SAFE, because it only enforces borrowing rules at runtime, and breaking those rules in a multi-threaded environment will cause a panic
// This shouldn't matter, since to game dialogue would only need to be accessed and loaded by a single thread, at a single point in time, but its worth noting here regardless.


#[derive(Debug, Clone)]
pub struct DialogueTree {
    root: Rc<DialogueNode>,
}

#[derive(Debug, Clone)]
pub struct DialogueNode {
    value: i32,
    parent: RefCell<Weak<DialogueNode>>,
    children: RefCell<Vec<Rc<DialogueNode>>>,
}

impl DialogueNode {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Self {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    /// Recursive function that allows printing of the full tree
    fn print_self(&self) {
        println!("{}", self.value);
        let children = self.children.borrow();
        for child in children.iter() {
            child.print_self();
        }
    }

    fn add_child(self: &Rc<Self>, value: i32) -> Rc<DialogueNode> {
        let child = Rc::new(DialogueNode {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        *child.parent.borrow_mut() = Rc::downgrade(&self);
        self.children.borrow_mut().push(Rc::clone(&child));
        
        return child;
    }

    /// Stitches a subtree onto another, but adding its root node as a child
    fn _add_subtree(self: &Rc<Self>, node: &Rc<DialogueNode>) {
        *node.parent.borrow_mut() = Rc::downgrade(&self);
        self.children.borrow_mut().push(Rc::clone(&node))
    }
}

pub fn build_tree() {
    let tree = DialogueTree {
        root: DialogueNode::new(5),
    };
    let subtree = DialogueTree {
        root: DialogueNode::new(3)
    };
    
    subtree.root.add_child(5);
    tree.root._add_subtree(&subtree.root);

    tree.root.print_self();
}
