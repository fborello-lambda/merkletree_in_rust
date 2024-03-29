// Tutorial -> https://www.youtube.com/watch?v=yHi3q2Iiepc
use std::fmt;
type Leaf = Option<Box<Node>>;

struct Tree {
    root: Leaf,
}

struct Node {
    item: i32,
    left: Leaf,
    right: Leaf,
}

impl Node {
    fn new(item: i32) -> Self {
        Node {
            item,
            left: None,
            right: None,
        }
    }
}

enum Dir {
    Left,
    Right,
    Root,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(root) = &self.root {
            writeln!(f, "Tree:")?;
            Tree::print_node(f, root, 0, Dir::Root)?;
        } else {
            write!(f, "Empty Tree")?;
        }
        Ok(())
    }
}

impl From<Node> for Leaf {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: i32) {
        /* for insert_recursive
        match &mut self.root {
            None => self.root = Node::new(value).into(),
            Some(node) => Tree::insert_recursive(node, value),
        }
        */
        self.insert_iterative(value)
    }

    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        match value.cmp(&node.item) {
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => match &mut node.right {
                None => node.right = Node::new(value).into(),
                Some(n) => Tree::insert_recursive(n, value),
            },
            std::cmp::Ordering::Less => match &mut node.left {
                None => node.left = Node::new(value).into(),
                Some(n) => Tree::insert_recursive(n, value),
            },
            // if the value is equal
        }
    }

    fn insert_iterative(&mut self, value: i32) {
        if self.root.is_none() {
            self.root = Node::new(value).into();
            return;
        }

        let mut q: Vec<&mut Box<Node>> = Vec::new();
        let root = self.root.as_mut().unwrap();
        q.push(root);

        while let Some(node) = q.pop() {
            match value.cmp(&node.item) {
                std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => match node.right {
                    ref mut right @ None => *right = Node::new(value).into(),
                    Some(ref mut n) => q.push(n),
                },
                std::cmp::Ordering::Less => match node.left {
                    ref mut left @ None => *left = Node::new(value).into(),
                    Some(ref mut n) => q.push(n),
                },
                // if the value is equal
            }
        }
    }

    fn print_node(
        f: &mut fmt::Formatter<'_>,
        node: &Node,
        level: usize,
        left_right: Dir,
    ) -> fmt::Result {
        let indent = "    ".repeat(level);

        let dir = match left_right {
            Dir::Left => "left",
            Dir::Right => "right",
            Dir::Root => "root",
        };
        writeln!(
            f,
            "{}{}{}-{}",
            indent,
            Tree::get_arrow(level),
            dir,
            node.item
        )?;
        if let Some(left_node) = &node.left {
            Tree::print_node(f, left_node.as_ref(), level + 1, Dir::Left)?;
        } else {
            writeln!(
                f,
                "{}{}left-None",
                "    ".repeat(level + 1),
                Tree::get_arrow(level),
            )?;
        }
        if let Some(right_node) = &node.right {
            Tree::print_node(f, right_node.as_ref(), level + 1, Dir::Right)?;
        } else {
            writeln!(
                f,
                "{}{}right-None",
                "    ".repeat(level + 1),
                Tree::get_arrow(level),
            )?;
        }
        Ok(())
    }
    fn get_arrow(level: usize) -> String {
        if level > 0 {
            String::from("└──>")
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binary_tree() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        println!("{tree}");
    }
}
