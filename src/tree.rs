#[allow(non_snake_case,non_camel_case_types,dead_code)]

pub mod Tree{
use std::{
    cell::RefCell,
    collections::{HashMap,VecDeque},
    rc::{Rc, Weak},
};

type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub children: Vec<NodeRef>,
}

pub struct Tree {
    pub roots: Vec<NodeRef>,
    pub nodes: HashMap<String, NodeRef>,
}

pub struct Relation {
    parent: String,
    child: String,
}

impl Tree {
    pub fn build(edges: Vec<Relation>) -> Self {
        let mut nodes: HashMap<String, NodeRef> = HashMap::new();

        for edge in &edges {
            let parent = nodes
                .entry(edge.parent.clone())
                .or_insert_with(|| {
                    Rc::new(RefCell::new(Node {
                        name: edge.parent.clone(),
                        parent: None,
                        children: Vec::new(),
                    }))
                })
                .clone();

            let child = nodes
                .entry(edge.child.clone())
                .or_insert_with(|| {
                    Rc::new(RefCell::new(Node {
                        name: edge.child.clone(),
                        parent: None,
                        children: Vec::new(),
                    }))
                })
                .clone();

            child.borrow_mut().parent = Some(Rc::downgrade(&parent));

            parent.borrow_mut().children.push(child);
        }

        let roots = nodes
            .values()
            .filter(|n| n.borrow().parent.is_none())
            .cloned()
            .collect();

        Tree { roots, nodes }
    }

pub fn add_node(
    &mut self,
    parent: Option<&str>,
    child: &str,
) -> Result<(), String> {
    // Duplicate node?
    if self.nodes.contains_key(child) {
        return Err(format!("Node '{child}' already exists"));
    }

    // Create the new node
    let node = Rc::new(RefCell::new(Node {
        name: child.to_string(),
        parent: None,
        children: Vec::new(),
    }));

    match parent {
        Some(parent_name) => {
            let parent = self
                .nodes
                .get(parent_name)
                .ok_or_else(|| format!("Parent '{parent_name}' not found"))?
                .clone();

            node.borrow_mut().parent = Some(Rc::downgrade(&parent));

            parent.borrow_mut().children.push(node.clone());
        }

        None => {
            self.roots.push(node.clone());
        }
    }

    self.nodes.insert(child.to_string(), node);

    Ok(())
}

 pub fn bfs(&self) {

        let mut q = VecDeque::new();

        for root in &self.roots {
            q.push_back(root.clone());
        }

        while let Some(node) = q.pop_front() {

            println!("{}", node.borrow().name);

            for child in &node.borrow().children {
                q.push_back(child.clone());
            }
        }
    }


    pub fn dfs(&self) {

        for root in &self.roots {
            Self::dfs_node(root);
        }
    }

    fn dfs_node(node: &NodeRef) {

        println!("{}", node.borrow().name);

        for child in &node.borrow().children {
            Self::dfs_node(child);
        }
    }

    pub fn parent_of(
        &self,
        child: &str,
    ) -> Option<String> {

        let node = self.nodes.get(child)?;

        node.borrow()
            .parent
            .as_ref()?
            .upgrade()
            .map(|p| p.borrow().name.clone())
    }

    pub fn ancestors(
        &self,
        node: &str,
    ) -> Vec<String> {

        let mut result = Vec::new();

        let mut current = self.nodes.get(node).cloned();

        while let Some(node) = current {

            let parent = node.borrow().parent.clone();

            match parent {

                None => break,

                Some(p) => {

                    let p = p.upgrade().unwrap();

                    result.push(p.borrow().name.clone());

                    current = Some(p);
                }
            }
        }

        result
    }

    pub fn path_to_root(
        &self,
        node: &str,
    ) -> Vec<String> {

        let mut path = vec![node.to_string()];

        let mut current = self.nodes[node]
            .borrow()
            .parent
            .clone();

        while let Some(parent) = current {

            let p = parent.upgrade().unwrap();

            path.push(p.borrow().name.clone());

            current = p.borrow().parent.clone();
        }

        path
    }

}


}