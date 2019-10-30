mod node;
use node::*;
use std::ptr::write;

#[derive(Debug)]
struct BinaryTree<T: PartialOrd + Clone> {
    root: Link<T>,
}

struct BinaryTreeIterator<'a, T: PartialOrd + Clone> {
    counter: usize,
    root: &'a Link<T>,
}

impl<'a, T: PartialOrd + Clone + std::fmt::Debug> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = BinaryTreeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        BinaryTreeIterator {
            counter: 1,
            root: &self.root,
        }
    }
}

impl<'a, T: PartialOrd + Clone + std::fmt::Debug> Iterator for BinaryTreeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.root {
            Some(root) => {
                if let Some(NodeOptionValue::Value(next)) =
                    root.find_n_max(self.counter, NodeOption::Value)
                {
                    self.counter += 1;
                    Some(&next)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl<T: PartialOrd + Clone> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, new_value: T) {
        match self.root {
            Some(ref mut node) => node.insert(new_value),
            None => self.root = Some(Box::new(Node::create(new_value))),
        }
    }

    pub fn find(&self, value: T) -> bool {
        match &self.root {
            Some(node) => node.find(&value),
            None => false,
        }
    }

    pub fn find_min(&self) -> Option<&T> {
        match &self.root {
            Some(node) => Some(node.find_min()),
            None => None,
        }
    }

    pub fn find_max(&self) -> Option<&T> {
        match &self.root {
            Some(node) => Some(node.find_max()),
            None => None,
        }
    }

    pub fn find_n_max(&self, n: usize) -> Option<&T> {
        match &self.root {
            Some(node) if n != 0 => {
                if let Some(NodeOptionValue::Value(n_max)) = node.find_n_max(n, NodeOption::Value) {
                    Some(n_max)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn delete_node(&mut self, value: T) {
        match &mut self.root {
            Some(node) if node.value == value => self.root = None,
            Some(node) => {
                if let Some(nodes_count) = node.get_nodes_count(&value) {
                    node.delete_node(nodes_count, value)
                } else {
                    return;
                }
            }
            _ => return,
        }
    }

    pub fn delete(&mut self, val: T) {
        match &mut self.root {
            Some(node) if node.value == val => {
                if node.is_leaf() {
                    self.root = None
                } else {
                    node.delete_value()
                }
            }
            Some(node) => {
                if node.find(&val) {
                    node.delete(val)
                } else {
                    return;
                }
            }
            _ => return,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let mut tree = BinaryTree::new();
        tree.insert(5);
        tree.insert(7);
        tree.insert(4);
        tree.insert(10);
        assert!(tree.find(7))
    }

    #[test]
    #[should_panic]
    fn test_find() {
        let tree = BinaryTree::new();
        assert!(tree.find(5))
    }

    #[test]
    fn test_find_max() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.find_max(), None);
        tree.insert(5);
        tree.insert(14);
        tree.insert(8);
        tree.insert(44);
        tree.insert(25);
        assert_eq!(tree.find_max(), Some(&44))
    }

    #[test]
    fn test_find_min() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.find_min(), None);
        tree.insert(5);
        tree.insert(14);
        tree.insert(4);
        tree.insert(6);
        tree.insert(3);
        tree.insert(44);
        tree.insert(25);
        assert_eq!(tree.find_min(), Some(&3))
    }

    #[test]
    fn test_find_n_max() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.find_n_max(4), None);
        tree.insert(3);
        tree.insert(22);
        tree.insert(8);
        tree.insert(55);
        tree.insert(26);
        assert_eq!(tree.find_n_max(3), Some(&22))
    }

    #[test]
    fn test_iter() {
        let mut tree = BinaryTree::new();
        let mut counter = 0;
        tree.insert(3);
        tree.insert(5);
        tree.insert(22);
        tree.insert(7);
        tree.insert(7);
        tree.insert(7);
        tree.insert(22);
        tree.insert(5);
        tree.insert(21);
        tree.insert(53);
        tree.insert(21);
        let mut tree_iter = tree.into_iter();
        tree.into_iter().for_each(|elem| {
            assert_eq!(Some(elem), tree_iter.next());
            counter += 1
        });
        assert_eq!(counter, tree.into_iter().count())
    }

    #[test]
    fn test_delete_node() {
        let mut tree = BinaryTree::new();
        tree.insert(8);
        tree.insert(5);
        tree.insert(3);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(7);
        tree.insert(15);
        tree.insert(12);
        tree.insert(17);
        tree.insert(10);
        tree.insert(14);
        assert_eq!(12, tree.into_iter().count());
        tree.delete_node(3);
        tree.delete_node(12);
        assert_eq!(6, tree.into_iter().count());
    }

    #[test]
    fn test_delete() {
        let mut tree = BinaryTree::new();
        tree.insert(8);
        tree.insert(5);
        tree.insert(3);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(7);
        tree.insert(15);
        tree.insert(12);
        tree.insert(17);
        tree.insert(10);
        tree.insert(14);
        assert_eq!(12, tree.into_iter().count());
        tree.delete(8);
        assert_eq!(11, tree.into_iter().count());
        tree.delete(10);
        tree.delete(3);
        tree.delete(2);
        tree.delete(17);
        tree.delete(4);
        tree.delete(15);
        assert_eq!(5, tree.into_iter().count());
    }
}
