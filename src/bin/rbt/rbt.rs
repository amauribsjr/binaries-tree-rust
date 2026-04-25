use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    Red,
    Black,
}

type Link = Option<usize>;

#[derive(Clone, Debug)]
struct Node {
    key: i32,
    color: Color,
    left: Link,
    right: Link,
    parent: Link,
}

impl Node {
    fn new(key: i32) -> Self {
        Self {
            key,
            color: Color::Red,
            left: None,
            right: None,
            parent: None,
        }
    }
}

pub struct RBT {
    nodes: Vec<Node>,
    root: Link,
}

impl RBT {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
        }
    }

    fn color(&self, node: Link) -> Color {
        match node {
            Some(index) => self.nodes[index].color,
            None => Color::Black,
        }
    }

    fn set_color(&mut self, node: Link, color: Color) {
        if let Some(index) = node {
            self.nodes[index].color = color;
        }
    }

    fn parent(&self, node: usize) -> Link {
        self.nodes[node].parent
    }

    fn grandparent(&self, node: usize) -> Link {
        self.parent(node).and_then(|parent| self.nodes[parent].parent)
    }

    pub fn insert(&mut self, key: i32) -> bool {
        if self.find_node(key).is_some() {
            return false;
        }

        let z = self.nodes.len();
        self.nodes.push(Node::new(key));

        let mut y: Link = None;
        let mut x = self.root;

        while let Some(x_index) = x {
            y = x;
            if self.nodes[z].key < self.nodes[x_index].key {
                x = self.nodes[x_index].left;
            } else {
                x = self.nodes[x_index].right;
            }
        }

        self.nodes[z].parent = y;

        if y.is_none() {
            self.root = Some(z);
        } else if self.nodes[z].key < self.nodes[y.unwrap()].key {
            self.nodes[y.unwrap()].left = Some(z);
        } else {
            self.nodes[y.unwrap()].right = Some(z);
        }

        self.insert_fixup(z);
        true
    }

    fn insert_fixup(&mut self, mut z: usize) {
        while self.color(self.parent(z)) == Color::Red {
            let parent = self.parent(z).unwrap();
            let grandparent = self.grandparent(z).unwrap();

            if Some(parent) == self.nodes[grandparent].left {
                let uncle = self.nodes[grandparent].right;

                if self.color(uncle) == Color::Red {
                    self.set_color(Some(parent), Color::Black);
                    self.set_color(uncle, Color::Black);
                    self.set_color(Some(grandparent), Color::Red);

                    z = grandparent;
                } else {
                    if Some(z) == self.nodes[parent].right {
                        z = parent;
                        self.rotate_left(z);
                    }

                    let parent = self.parent(z).unwrap();
                    let grandparent = self.grandparent(z).unwrap();

                    self.set_color(Some(parent), Color::Black);
                    self.set_color(Some(grandparent), Color::Red);

                    self.rotate_right(grandparent);
                }
            } else {
                let uncle = self.nodes[grandparent].left;

                if self.color(uncle) == Color::Red {
                    self.set_color(Some(parent), Color::Black);
                    self.set_color(uncle, Color::Black);
                    self.set_color(Some(grandparent), Color::Red);

                    z = grandparent;
                } else {
                    if Some(z) == self.nodes[parent].left {
                        z = parent;
                        self.rotate_right(z);
                    }

                    let parent = self.parent(z).unwrap();
                    let grandparent = self.grandparent(z).unwrap();

                    self.set_color(Some(parent), Color::Black);
                    self.set_color(Some(grandparent), Color::Red);

                    self.rotate_left(grandparent);
                }
            }
        }

        self.set_color(self.root, Color::Black);
    }

    fn rotate_left(&mut self, x: usize) {
        let y = self.nodes[x]
            .right
            .expect("rotate_left requires a right child");

        let y_left = self.nodes[y].left;

        self.nodes[x].right = y_left;

        if let Some(y_left_index) = y_left {
            self.nodes[y_left_index].parent = Some(x);
        }

        let x_parent = self.nodes[x].parent;

        self.nodes[y].parent = x_parent;

        if x_parent.is_none() {
            self.root = Some(y);
        } else if self.nodes[x_parent.unwrap()].left == Some(x) {
            self.nodes[x_parent.unwrap()].left = Some(y);
        } else {
            self.nodes[x_parent.unwrap()].right = Some(y);
        }

        self.nodes[y].left = Some(x);
        self.nodes[x].parent = Some(y);
    }

    fn rotate_right(&mut self, x: usize) {
        let y = self.nodes[x]
            .left
            .expect("rotate_right requires a left child");

        let y_right = self.nodes[y].right;

        self.nodes[x].left = y_right;

        if let Some(y_right_index) = y_right {
            self.nodes[y_right_index].parent = Some(x);
        }

        let x_parent = self.nodes[x].parent;

        self.nodes[y].parent = x_parent;

        if x_parent.is_none() {
            self.root = Some(y);
        } else if self.nodes[x_parent.unwrap()].right == Some(x) {
            self.nodes[x_parent.unwrap()].right = Some(y);
        } else {
            self.nodes[x_parent.unwrap()].left = Some(y);
        }

        self.nodes[y].right = Some(x);
        self.nodes[x].parent = Some(y);
    }

    pub fn search(&self, key: i32) -> bool {
        self.find_node(key).is_some()
    }

    fn find_node(&self, key: i32) -> Link {
        let mut current = self.root;

        while let Some(index) = current {
            if key == self.nodes[index].key {
                return Some(index);
            } else if key < self.nodes[index].key {
                current = self.nodes[index].left;
            } else {
                current = self.nodes[index].right;
            }
        }

        None
    }

    pub fn calculate_height(&self) -> i32 {
        self.calculate_height_node(self.root)
    }

    fn calculate_height_node(&self, node: Link) -> i32 {
        match node {
            None => 0,
            Some(index) => {
                let left_height = self.calculate_height_node(self.nodes[index].left);
                let right_height = self.calculate_height_node(self.nodes[index].right);

                1 + left_height.max(right_height)
            }
        }
    }

    pub fn remove(&mut self, key: i32) -> bool {
        let z = self.find_node(key);

        if z.is_none() {
            return false;
        }

        self.delete(z.unwrap());
        true
    }

    fn transplant(&mut self, u: usize, v: Link) {
        let u_parent = self.nodes[u].parent;

        if u_parent.is_none() {
            self.root = v;
        } else if Some(u) == self.nodes[u_parent.unwrap()].left {
            self.nodes[u_parent.unwrap()].left = v;
        } else {
            self.nodes[u_parent.unwrap()].right = v;
        }

        if let Some(v_index) = v {
            self.nodes[v_index].parent = u_parent;
        }
    }

    fn delete(&mut self, z: usize) {
        let mut y = z;
        let x: Link;
        let x_parent: Link;
        let mut y_original_color = self.nodes[y].color;

        if self.nodes[z].left.is_none() {
            x = self.nodes[z].right;
            x_parent = self.nodes[z].parent;

            self.transplant(z, self.nodes[z].right);
        } else if self.nodes[z].right.is_none() {
            x = self.nodes[z].left;
            x_parent = self.nodes[z].parent;

            self.transplant(z, self.nodes[z].left);
        } else {
            y = self.smallest_node(self.nodes[z].right.unwrap());
            y_original_color = self.nodes[y].color;
            x = self.nodes[y].right;

            if self.nodes[y].parent == Some(z) {
                x_parent = Some(y);

                if let Some(x_index) = x {
                    self.nodes[x_index].parent = Some(y);
                }
            } else {
                x_parent = self.nodes[y].parent;

                self.transplant(y, self.nodes[y].right);

                self.nodes[y].right = self.nodes[z].right;

                if let Some(right) = self.nodes[y].right {
                    self.nodes[right].parent = Some(y);
                }
            }

            self.transplant(z, Some(y));

            self.nodes[y].left = self.nodes[z].left;

            if let Some(left) = self.nodes[y].left {
                self.nodes[left].parent = Some(y);
            }

            self.nodes[y].color = self.nodes[z].color;
        }

        if y_original_color == Color::Black {
            self.delete_fixup(x, x_parent);
        }
    }

    fn delete_fixup(&mut self, mut x: Link, mut x_parent: Link) {
        while x != self.root && self.color(x) == Color::Black {
            let Some(parent) = x_parent else {
                break;
            };

            if x == self.nodes[parent].left {
                let mut w = self.nodes[parent].right;

                if self.color(w) == Color::Red {
                    self.set_color(w, Color::Black);
                    self.set_color(Some(parent), Color::Red);
                    self.rotate_left(parent);

                    w = self.nodes[parent].right;
                }

                let w_left_black = self.color(w.and_then(|i| self.nodes[i].left)) == Color::Black;
                let w_right_black = self.color(w.and_then(|i| self.nodes[i].right)) == Color::Black;

                if w_left_black && w_right_black {
                    self.set_color(w, Color::Red);

                    x = Some(parent);
                    x_parent = self.nodes[parent].parent;
                } else {
                    let w_right_black =
                        self.color(w.and_then(|i| self.nodes[i].right)) == Color::Black;

                    if w_right_black {
                        if let Some(w_index) = w {
                            self.set_color(self.nodes[w_index].left, Color::Black);
                            self.set_color(w, Color::Red);
                            self.rotate_right(w_index);
                        }

                        w = self.nodes[parent].right;
                    }

                    if let Some(w_index) = w {
                        self.nodes[w_index].color = self.nodes[parent].color;
                        self.nodes[parent].color = Color::Black;
                        self.set_color(self.nodes[w_index].right, Color::Black);
                    }

                    self.rotate_left(parent);

                    x = self.root;
                    x_parent = None;
                }
            } else {
                let mut w = self.nodes[parent].left;

                if self.color(w) == Color::Red {
                    self.set_color(w, Color::Black);
                    self.set_color(Some(parent), Color::Red);
                    self.rotate_right(parent);

                    w = self.nodes[parent].left;
                }

                let w_right_black = self.color(w.and_then(|i| self.nodes[i].right)) == Color::Black;
                let w_left_black = self.color(w.and_then(|i| self.nodes[i].left)) == Color::Black;

                if w_right_black && w_left_black {
                    self.set_color(w, Color::Red);

                    x = Some(parent);
                    x_parent = self.nodes[parent].parent;
                } else {
                    let w_left_black =
                        self.color(w.and_then(|i| self.nodes[i].left)) == Color::Black;

                    if w_left_black {
                        if let Some(w_index) = w {
                            self.set_color(self.nodes[w_index].right, Color::Black);
                            self.set_color(w, Color::Red);
                            self.rotate_left(w_index);
                        }

                        w = self.nodes[parent].left;
                    }

                    if let Some(w_index) = w {
                        self.nodes[w_index].color = self.nodes[parent].color;
                        self.nodes[parent].color = Color::Black;
                        self.set_color(self.nodes[w_index].left, Color::Black);
                    }

                    self.rotate_right(parent);

                    x = self.root;
                    x_parent = None;
                }
            }
        }

        self.set_color(x, Color::Black);
    }

    fn smallest_node(&self, mut node: usize) -> usize {
        while let Some(left) = self.nodes[node].left {
            node = left;
        }

        node
    }

    pub fn print_tree(&self) {
        self.print_tree_node(self.root);
    }

    fn print_tree_node(&self, current: Link) {
        if let Some(index) = current {
            self.print_tree_node(self.nodes[index].left);

            print!(
                "{}{} ",
                self.nodes[index].key,
                if self.nodes[index].color == Color::Red {
                    "[R]"
                } else {
                    "[B]"
                }
            );

            self.print_tree_node(self.nodes[index].right);
        }
    }

    pub fn print_by_level(&self) {
        if self.root.is_none() {
            println!("Tree is empty.");
            return;
        }

        let height = self.calculate_height();
        let mut queue: VecDeque<Link> = VecDeque::new();

        queue.push_back(self.root);

        for level in 0..height {
            let level_size = queue.len();
            let spaces = (2_i32.pow((height - level - 1) as u32) - 1) as usize;

            Self::print_spaces(spaces * 4);

            for _ in 0..level_size {
                let current = queue.pop_front().unwrap();

                if let Some(index) = current {
                    let color = if self.nodes[index].color == Color::Red {
                        "R"
                    } else {
                        "B"
                    };

                    print!("{:>3}{}", self.nodes[index].key, color);

                    queue.push_back(self.nodes[index].left);
                    queue.push_back(self.nodes[index].right);
                } else {
                    print!(" --- ");

                    queue.push_back(None);
                    queue.push_back(None);
                }

                Self::print_spaces((spaces * 2 + 1) * 4);
            }

            println!("\n");
        }
    }

    fn print_spaces(count: usize) {
        print!("{}", " ".repeat(count));
    }
}

impl Default for RBT {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_search() {
        let mut rbt = RBT::new();
        assert!(rbt.insert(10));
        assert!(rbt.insert(5));
        assert!(rbt.insert(15));
        assert!(rbt.search(10));
        assert!(rbt.search(5));
        assert!(rbt.search(15));
        assert!(!rbt.search(99));
    }

    #[test]
    fn insert_duplicate_returns_false() {
        let mut rbt = RBT::new();
        assert!(rbt.insert(10));
        assert!(!rbt.insert(10));
    }

    #[test]
    fn remove_returns_false_when_not_found() {
        let mut rbt = RBT::new();
        rbt.insert(10);
        assert!(!rbt.remove(99));
    }

    #[test]
    fn remove_existing_key() {
        let mut rbt = RBT::new();
        for i in [10, 5, 15] {
            rbt.insert(i);
        }
        assert!(rbt.remove(5));
        assert!(!rbt.search(5));
        assert!(rbt.search(10));
    }

    #[test]
    fn root_is_always_black() {
        let mut rbt = RBT::new();
        for i in [10, 5, 15, 3, 7, 12, 17] {
            rbt.insert(i);
            let root_idx = rbt.root.unwrap();
            assert_eq!(rbt.nodes[root_idx].color, Color::Black);
        }
    }

    #[test]
    fn no_consecutive_red_nodes() {
        let mut rbt = RBT::new();
        for i in [10, 5, 15, 3, 7, 12, 17, 1, 4] {
            rbt.insert(i);
        }
        for (idx, node) in rbt.nodes.iter().enumerate() {
            if node.color == Color::Red {
                if let Some(left) = node.left {
                    assert_eq!(
                        rbt.nodes[left].color,
                        Color::Black,
                        "Node {} (Red) has Red left child",
                        idx
                    );
                }
                if let Some(right) = node.right {
                    assert_eq!(
                        rbt.nodes[right].color,
                        Color::Black,
                        "Node {} (Red) has Red right child",
                        idx
                    );
                }
            }
        }
    }

    #[test]
    fn height_empty_tree() {
        assert_eq!(RBT::new().calculate_height(), 0);
    }
}