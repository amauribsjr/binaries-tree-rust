use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    key: i32,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32) -> Self {
        Self {
            key,
            height: 1,
            left: None,
            right: None,
        }
    }
}

pub struct AVLTree {
    root: Option<Box<Node>>,
}

impl AVLTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn search(&self, key: i32) -> bool {
        let mut current = self.root.as_deref();

        while let Some(node) = current {
            if key == node.key {
                return true;
            }

            if key > node.key {
                current = node.right.as_deref();
            } else {
                current = node.left.as_deref();
            }
        }

        false
    }

    pub fn insert(&mut self, key: i32) -> bool {
        let (new_root, inserted) = Self::insert_node(self.root.take(), key);
        self.root = Some(new_root);
        inserted
    }

    fn insert_node(node: Option<Box<Node>>, key: i32) -> (Box<Node>, bool) {
        let Some(mut node) = node else {
            return (Box::new(Node::new(key)), true);
        };

        let inserted;

        if key < node.key {
            let (new_left, did_insert) = Self::insert_node(node.left.take(), key);
            node.left = Some(new_left);
            inserted = did_insert;
        } else if key > node.key {
            let (new_right, did_insert) = Self::insert_node(node.right.take(), key);
            node.right = Some(new_right);
            inserted = did_insert;
        } else {
            return (node, false);
        }

        Self::update_height(&mut node);

        let balance = Self::get_balance_factor(Some(&node));

        if balance > 1 && key < node.left.as_ref().unwrap().key {
            return (Self::rotate_right(node), inserted);
        }
        if balance < -1 && key > node.right.as_ref().unwrap().key {
            return (Self::rotate_left(node), inserted);
        }
        if balance > 1 && key > node.left.as_ref().unwrap().key {
            node.left = Some(Self::rotate_left(node.left.take().unwrap()));
            return (Self::rotate_right(node), inserted);
        }
        if balance < -1 && key < node.right.as_ref().unwrap().key {
            node.right = Some(Self::rotate_right(node.right.take().unwrap()));
            return (Self::rotate_left(node), inserted);
        }

        (node, inserted)
    }

    pub fn remove(&mut self, value: i32) -> bool {
        let (new_root, removed) = Self::remove_node(self.root.take(), value);

        self.root = new_root;

        removed
    }

    fn remove_node(node: Option<Box<Node>>, value: i32) -> (Option<Box<Node>>, bool) {
        let Some(mut node) = node else {
            return (None, false);
        };

        let removed;

        if value < node.key {
            let (new_left, did_remove) = Self::remove_node(node.left.take(), value);

            node.left = new_left;
            removed = did_remove;
        } else if value > node.key {
            let (new_right, did_remove) = Self::remove_node(node.right.take(), value);

            node.right = new_right;
            removed = did_remove;
        } else {
            removed = true;

            if node.left.is_none() && node.right.is_none() {
                return (None, true);
            }

            if node.left.is_none() {
                return (node.right, true);
            }

            if node.right.is_none() {
                return (node.left, true);
            }

            let successor_key = Self::smallest_value(node.right.as_deref().unwrap());

            node.key = successor_key;

            let (new_right, _) = Self::remove_node(node.right.take(), successor_key);
            node.right = new_right;
        }

        Self::update_height(&mut node);

        let balance = Self::get_balance_factor(Some(&node));

        if balance > 1 && Self::get_balance_factor(node.left.as_deref()) >= 0 {
            return (Some(Self::rotate_right(node)), removed);
        }

        if balance > 1 && Self::get_balance_factor(node.left.as_deref()) < 0 {
            node.left = Some(Self::rotate_left(node.left.take().unwrap()));

            return (Some(Self::rotate_right(node)), removed);
        }

        if balance < -1 && Self::get_balance_factor(node.right.as_deref()) <= 0 {
            return (Some(Self::rotate_left(node)), removed);
        }

        if balance < -1 && Self::get_balance_factor(node.right.as_deref()) > 0 {
            node.right = Some(Self::rotate_right(node.right.take().unwrap()));

            return (Some(Self::rotate_left(node)), removed);
        }

        (Some(node), removed)
    }

    fn smallest_value(mut node: &Node) -> i32 {
        while let Some(left) = node.left.as_deref() {
            node = left;
        }

        node.key
    }

    pub fn calculate_height(&self) -> i32 {
        self.root.as_ref().map_or(0, |n| n.height)
    }

    fn height(node: Option<&Node>) -> i32 {
        match node {
            None => 0,
            Some(n) => n.height,
        }
    }

    fn update_height(node: &mut Node) {
        node.height = 1 + Self::height(node.left.as_deref())
            .max(Self::height(node.right.as_deref()));
    }

    fn get_balance_factor(node: Option<&Node>) -> i32 {
        match node {
            None => 0,
            Some(n) => Self::height(n.left.as_deref()) - Self::height(n.right.as_deref()),
        }
    }

    fn rotate_left(mut node: Box<Node>) -> Box<Node> {
        let mut temp = node
            .right
            .take()
            .expect("rotate_left requires a right child");

        let temp2 = temp.left.take();

        temp.left = Some(node);
        temp.left.as_mut().unwrap().right = temp2;

        Self::update_height(temp.left.as_mut().unwrap());
        Self::update_height(&mut temp);

        temp
    }

    fn rotate_right(mut node: Box<Node>) -> Box<Node> {
        let mut temp = node
            .left
            .take()
            .expect("rotate_right requires a left child");

        let temp2 = temp.right.take();

        temp.right = Some(node);
        temp.right.as_mut().unwrap().left = temp2;

        Self::update_height(temp.right.as_mut().unwrap());
        Self::update_height(&mut temp);

        temp
    }

    pub fn print_tree(&self) {
        Self::print_tree_node(self.root.as_deref());
    }

    fn print_tree_node(current: Option<&Node>) {
        if let Some(node) = current {
            Self::print_tree_node(node.left.as_deref());
            print!("{} ", node.key);
            Self::print_tree_node(node.right.as_deref());
        }
    }

    pub fn print_by_level(&self) {
        if self.root.is_none() {
            println!("Tree is empty.");
            return;
        }

        let height = self.calculate_height();
        let mut queue: VecDeque<Option<&Node>> = VecDeque::new();

        queue.push_back(self.root.as_deref());

        let max_width = 2_i32.pow(height as u32) - 1;

        for level in 0..height {
            let level_size = queue.len();
            let spaces = max_width / 2_i32.pow((level + 1) as u32);

            Self::print_spaces(spaces);

            for _ in 0..level_size {
                let current = queue.pop_front().unwrap();

                if let Some(node) = current {
                    print!("{}", node.key);
                    queue.push_back(node.left.as_deref());
                    queue.push_back(node.right.as_deref());
                } else {
                    print!("n");
                    queue.push_back(None);
                    queue.push_back(None);
                }

                Self::print_spaces(spaces * 2 + 1);
            }

            println!();
        }
    }

    fn print_spaces(count: i32) {
        for _ in 0..count {
            print!(" ");
        }
    }
}

impl Default for AVLTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_search() {
        let mut avl = AVLTree::new();

        assert!(avl.insert(10));
        assert!(avl.insert(5));
        assert!(avl.insert(15));

        assert!(avl.search(10));
        assert!(avl.search(5));
        assert!(avl.search(15));
        assert!(!avl.search(99));
    }

    #[test]
    fn insert_duplicate_returns_false() {
        let mut avl = AVLTree::new();

        assert!(avl.insert(10));
        assert!(!avl.insert(10));
    }

    #[test]
    fn remove_existing_value() {
        let mut avl = AVLTree::new();

        for value in [10, 5, 15] {
            avl.insert(value);
        }

        assert!(avl.remove(5));
        assert!(!avl.search(5));
        assert!(avl.search(10));
        assert!(avl.search(15));
    }

    #[test]
    fn remove_missing_value_returns_false() {
        let mut avl = AVLTree::new();

        avl.insert(10);

        assert!(!avl.remove(99));
        assert!(avl.search(10));
    }

    #[test]
    fn height_empty_tree_is_zero() {
        let avl = AVLTree::new();

        assert_eq!(avl.calculate_height(), 0);
    }

    #[test]
    fn ordered_insertions_remain_balanced() {
        let mut avl = AVLTree::new();

        for value in 1..=7 {
            avl.insert(value);
        }

        assert_eq!(avl.calculate_height(), 3);
    }
}
