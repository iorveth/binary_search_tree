pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
pub struct Node<T: PartialOrd + Clone> {
    pub value: T,
    pub left: Link<T>,
    pub right: Link<T>,
    pub left_nodes_count: usize,
}

pub enum NodeOption {
    Node,
    Value,
}

pub enum NodeOptionValue<'a, T: PartialOrd + Clone> {
    Node(&'a Node<T>),
    Value(&'a T),
}

pub struct NodeIterator<'a, T: PartialOrd + Clone> {
    counter: usize,
    node: Option<&'a Node<T>>,
}

pub struct NodeValuesIterator<'a, T: PartialOrd + Clone> {
    counter: usize,
    node: Option<&'a Node<T>>,
}

impl<T: PartialOrd + Clone> Node<T> {
    pub fn create(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
            left_nodes_count: 0,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.right.is_none() && self.left.is_none()
    }

    pub fn values(&self) -> NodeValuesIterator<T> {
        NodeValuesIterator {
            counter: 1,
            node: Some(self),
        }
    }

    pub fn nodes(&self) -> NodeIterator<T> {
        NodeIterator {
            counter: 1,
            node: Some(self),
        }
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
            Node { value, .. } if *val == *value => Some(self.values().count()),
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
    pub fn delete_value(&mut self) {
        match (&mut self.right, &mut self.left) {
            (Some(right), _) => {
                if right.is_leaf() {
                    self.value = right.value.clone();
                    self.right = None
                } else {
                    if let Some(mut successor) = right.remove_min_node() {
                        right.sub_left_nodes_count(successor.values().count());
                        let successor_node_to_insert = successor.get_max_node_mut();
                        successor_node_to_insert.right = self.right.take();
                        successor.left = self.left.take();
                        successor.left_nodes_count = self.left_nodes_count;
                        *self = *successor
                    } else {
                        right.left = self.left.take();
                        right.left_nodes_count = self.left_nodes_count;
                        *self = *self.right.take().unwrap();
                    }
                }
            }
            (_, Some(left)) => {
                if left.is_leaf() {
                    self.value = left.value.clone();
                    self.left_nodes_count = 0;
                    self.left = None
                } else {
                    if let Some(mut successor) = left.remove_max_node() {
                        successor.add_left_nodes_count(left.values().count());
                        let successor_node_to_insert = successor.get_min_node_mut();
                        successor_node_to_insert.left = self.left.take();
                        successor.right = self.right.take();
                        *self = *successor
                    } else {
                        left.right = self.right.take();
                        *self = *self.left.take().unwrap();
                    }
                }
            }
            _ => return,
        }
    }
    pub fn delete(&mut self, val: T) {
        match self {
            Node {
                value,
                left: Some(left),
                ..
            } if val < *value => {
                self.left_nodes_count -= 1;
                if val == left.value && left.is_leaf() {
                    self.left = None
                } else if val == left.value {
                    left.delete_value()
                } else {
                    left.delete(val)
                }
            }
            Node {
                value,
                right: Some(right),
                ..
            } if val > *value => {
                if val == right.value && right.is_leaf() {
                    self.right = None
                } else if val == right.value {
                    right.delete_value()
                } else {
                    right.delete(val)
                }
            }
            _ => return,
        }
    }

    pub fn remove_node(&mut self, nodes_count: usize, val: T) -> Link<T> {
        match self {
            Node {
                value,
                left: Some(left),
                ..
            } if val < *value => {
                self.left_nodes_count -= nodes_count;
                if val == left.value {
                    self.left.take()
                } else {
                    left.remove_node(nodes_count, val)
                }
            }
            Node {
                value,
                right: Some(right),
                ..
            } if val > *value => {
                if val == right.value {
                    self.right.take()
                } else {
                    right.remove_node(nodes_count, val)
                }
            }
            _ => return None,
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

    pub fn find_min(&self) -> &T {
        if let Some(ref left) = self.left {
            left.find_min()
        } else {
            &self.value
        }
    }

    pub fn remove_max_node(&mut self) -> Link<T> {
        if let Some(ref mut right) = self.right {
            if right.right.is_some() {
                right.remove_max_node()
            } else {
                self.right.take()
            }
        } else {
            None
        }
    }

    pub fn remove_min_node(&mut self) -> Link<T> {
        if let Some(ref mut left) = self.left {
            if left.left.is_some() {
                left.remove_min_node()
            } else {
                self.left.take()
            }
        } else {
            None
        }
    }

    pub fn get_max_node_mut(&mut self) -> &mut Node<T> {
        if let Some(ref mut right) = self.right {
            right.get_max_node_mut()
        } else {
            self
        }
    }

    pub fn get_min_node_mut(&mut self) -> &mut Node<T> {
        if let Some(ref mut left) = self.left {
            left.get_min_node_mut()
        } else {
            self
        }
    }

    pub fn add_left_nodes_count(&mut self, nodes_count: usize) {
        if let Some(ref mut left) = self.left {
            self.left_nodes_count += nodes_count;
            left.add_left_nodes_count(nodes_count)
        } else {
            self.left_nodes_count = nodes_count
        }
    }

    pub fn sub_left_nodes_count(&mut self, nodes_count: usize) {
        if let Some(ref mut left) = self.left {
            self.left_nodes_count -= nodes_count;
            left.sub_left_nodes_count(nodes_count)
        } else {
            self.left_nodes_count = 0
        }
    }

    pub fn find_n_max(&self, n: usize, node_option: NodeOption) -> Option<NodeOptionValue<T>> {
        match n {
            n if n == self.left_nodes_count + 1 => match node_option {
                NodeOption::Value => Some(NodeOptionValue::Value(&self.value)),
                NodeOption::Node => Some(NodeOptionValue::Node(self)),
            },
            n if n > self.left_nodes_count => {
                if let Some(ref right) = self.right {
                    right.find_n_max(n - self.left_nodes_count - 1, node_option)
                } else {
                    None
                }
            }
            _ => {
                if let Some(ref left) = self.left {
                    left.find_n_max(n, node_option)
                } else {
                    None
                }
            }
        }
    }
}

impl<'a, T: PartialOrd + Clone> Iterator for NodeValuesIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node {
            Some(node) => {
                if let Some(NodeOptionValue::Value(next)) =
                    node.find_n_max(self.counter, NodeOption::Value)
                {
                    self.counter += 1;
                    Some(next)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl<'a, T: PartialOrd + Clone> Iterator for NodeIterator<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node {
            Some(node) => {
                if let Some(NodeOptionValue::Node(next)) =
                    node.find_n_max(self.counter, NodeOption::Node)
                {
                    self.counter += 1;
                    Some(next)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
