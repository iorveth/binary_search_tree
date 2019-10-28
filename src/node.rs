pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T: PartialOrd> {
    value: T,
    left: Link<T>,
    right: Link<T>,
    left_nodes_count: usize,
}

pub struct NodeIterator<'a, T: PartialOrd> {
    counter: usize,
    node: Option<&'a Node<T>>,
}

impl<T: PartialOrd> Node<T> {
    
    pub fn create(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
            left_nodes_count: 0,
        }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn insert(&mut self, new_value: T) {
        if self.find(&new_value) {
            return;
        } else {
            match self {
                Node { value, left, .. } if new_value < *value => {
                    self.left_nodes_count += 1;
                    if let Some(left) = left {
                        left.insert(new_value)
                    } else {
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
                _ => return,
            }
        }
    }

    pub fn get_nodes_count(&mut self, val: &T) -> Option<usize> {
        match self {
            Node {
                value,
                left: Some(left),
                ..
            } if *val < *value => left.get_nodes_count(val),
            Node {
                value,
                right: Some(right),
                ..
            } if *val > *value => right.get_nodes_count(val),
            Node { value, .. } if *val == *value => Some(self.into_iter().count()),
            _ => return None,
        }
    }

    pub fn delete_node(&mut self, nodes_count: usize, val: T) {
        match self {
            Node {
                value,
                left: Some(left),
                ..
            } if val < *value => {
                self.left_nodes_count -= nodes_count;
                if val == left.value {
                    self.left = None
                } else {
                    left.delete_node(nodes_count, val)
                }
            }
            Node {
                value,
                right: Some(right),
                ..
            } if val > *value => {
                if val == right.value {
                    self.right = None
                } else {
                    right.delete_node(nodes_count, val)
                }
            }
            _ => return,
        }
    }

    pub fn find(&self, val: &T) -> bool {
        match self {
            Node {
                value,
                left: Some(left),
                ..
            } if *val < *value => left.find(val),
            Node {
                value,
                right: Some(right),
                ..
            } if *val > *value => right.find(val),
            Node { value, .. } if *val == *value => true,
            _ => false,
        }
    }

    pub fn find_max(&self) -> &T {
        if let Some(ref right) = self.right {
            right.find_max()
        } else {
            &self.value
        }
    }

    pub fn find_n_max(&self, n: usize) -> Option<&T> {
        match n {
            n if n == self.left_nodes_count + 1 => Some(&self.value),
            n if n > self.left_nodes_count => {
                if let Some(ref right) = self.right {
                    right.find_n_max(n - self.left_nodes_count - 1)
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

impl<'a, T: PartialOrd> Iterator for NodeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node {
            Some(node) => {
                if let Some(next) = node.find_n_max(self.counter) {
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

impl<'a, T: PartialOrd> IntoIterator for &'a Node<T> {
    type Item = &'a T;
    type IntoIter = NodeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIterator {
            counter: 1,
            node: Some(self),
        }
    }
}