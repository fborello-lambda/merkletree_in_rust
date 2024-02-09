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
        match &mut self.root {
            None => self.root = Node::new(value).into(),
            Some(node) => Tree::insert_recursive(node, value),
        }
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
        writeln!(f, "{} {}- {}", indent, dir, node.item)?;
        if let Some(left_node) = &node.left {
            Tree::print_node(f, left_node.as_ref(), level + 1, Dir::Left)?;
        } else {
            writeln!(f, "{}left- None", "    ".repeat(level + 1))?;
        }
        if let Some(right_node) = &node.right {
            Tree::print_node(f, right_node.as_ref(), level + 1, Dir::Right)?;
        } else {
            writeln!(f, "{}right- None", "    ".repeat(level + 1))?;
        }
        Ok(())
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
        println!("{tree}");
    }
}
