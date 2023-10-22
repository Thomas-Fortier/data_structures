struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn add(&mut self, data: T) {
        let new_node = Node { data, next: None };
        let last_node: &mut Option<Box<Node<T>>> = self.get_last_node();

        if let Some(mut node) = last_node.as_mut() {
            node.next = Some(Box::new(new_node));
        } else {
            self.head = Some(Box::new(new_node));
        }
    }

    fn get_last_node(&mut self) -> &mut Option<Box<Node<T>>> {
        if self.head.is_none() {
            return &mut self.head;
        }

        let mut current_node: &mut Option<Box<Node<T>>> = &mut self.head;

        while !current_node.as_ref().unwrap().next.is_none() {
            current_node = &mut current_node.as_mut().unwrap().next;
        }

        current_node
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current_node = &self.head;
        let mut current_index = 0;

        while let Some(node) = current_node {
            if current_index == index {
                return Some(&node.data);
            }

            current_node = &node.next;
            current_index += 1;
        }

        None
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter(self.head)
    }
}

pub struct LinkedListIter<T>(Option<Box<Node<T>>>);

impl<T> Iterator for LinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.0.take() {
            Some(node) => {
                self.0 = node.next;
                Some(node.data)
            }
            None => None,
        }
    }
}