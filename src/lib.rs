type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T: PartialOrd> {
    value: T,
    left: Link<T>,
    right: Link<T>,
    left_count: usize,
}

impl<T: PartialOrd> Node<T> {
    fn create(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
            left_count: 0,
        }
    }

    fn insert(&mut self, new_value: T) {
        match self {
            Node { value, left, .. } if new_value < *value => {
                if let Some(left) = left {
                    self.left_count += 1;
                    left.insert(new_value)
                } else {
                    self.left_count += 1;
                    self.left = Some(Box::new(Node::create(new_value)))
                }
            }
            Node { value, right, .. } if new_value > *value => {
                if let Some(right) = right {
                    right.insert(new_value)
                } else {
                    self.right = Some(Box::new(Node::create(new_value)))
                }
            }
            Node { value, .. } if new_value == *value => return,
            _ => return,
        }
    }

    fn find(&self, val: T) -> bool {
        match self {
            Node { value, left, .. } if val < *value => {
                if let Some(left) = left {
                    left.find(val)
                } else {
                    false
                }
            }
            Node { value, right, .. } if val > *value => {
                if let Some(right) = right {
                    right.find(val)
                } else {
                    false
                }
            }
            Node { value, .. } if val == *value => true,
            _ => false,
        }
    }

    fn find_max(&self) -> &T {
        if let Some(ref right) = self.right {
            right.find_max()
        } else {
            &self.value
        }
    }

    fn find_n_max(&self, n: usize) -> Option<&T> {
        match n {
            n if n == self.left_count + 1 => Some(&self.value),
            n if n > self.left_count => {
                if let Some(ref right) = self.right {
                    right.find_n_max(n - self.left_count - 1)
                } else {
                    None
                }
            }
            _ => {
                if let Some(ref left) = self.left {
                    left.find_n_max(n)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
struct BinaryTree<T: PartialOrd> {
    root: Link<T>,
}

struct NodeIterator<'a, T: PartialOrd> {
    counter: usize,
    root: &'a Link<T>,
}

impl<'a, T: PartialOrd> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = NodeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIterator {
            counter: 1,
            root: &self.root,
        }
    }
}

impl<'a, T: PartialOrd> Iterator for NodeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.root {
            Some(root) => {
                if let Some(next) = root.find_n_max(self.counter) {
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

impl<T: PartialOrd> BinaryTree<T> {
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
            Some(node) => node.find(value),
            None => false,
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
            Some(node) if n != 0 => node.find_n_max(n),
            _ => None,
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
}
