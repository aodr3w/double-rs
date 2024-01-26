use core::fmt;
use std::{cell::RefCell, fmt::Debug, rc::{Rc, Weak}};
//implementation of doubly DoublyLinkedList in rust

pub struct Node<T>{
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    previous: Option<Weak<RefCell<Node<T>>>>
}


impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prev_value = if self.previous.is_some() {"Some"} else {"None"};
        let next_value = if self.next.is_some() {"Some"} else {"None"};

        f.debug_struct("Node")
        .field("value", &self.value)
        .field("next", &next_value)
        .field("previous", &prev_value)
        .finish()
    }
}

impl <T: Debug + PartialEq>Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
       Rc::new(RefCell::new(Node { value:value, next: None,previous: None }))
    }

}

pub struct DoublyLinkedList<T>{
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: i32
}

impl <T: Debug + PartialEq>DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList { head: None, tail: None, size: 0 }
    }

    pub fn append(&mut self, v: T) {
        let new_node = Node::new(v);
        if self.head.is_none(){
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            if let Some(old_tail) = self.tail.take(){
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().previous = Some(Rc::downgrade(&old_tail));
                self.tail = Some(new_node);
            }
        }
        self.size += 1
 
    }

    pub fn remove_values(&mut self, value: T){
        let nodes_to_remove = self.find_all(value);
        for node in nodes_to_remove{
            self.remove_node(node)
        }
    }

    pub fn remove_value(&mut self, value: T) {
        let node_to_remove = self.find_first(value);
        if let Some(node) = node_to_remove {
            self.remove_node(node)
        }
    }

    fn remove_node(&mut self, node_to_remove: Rc<RefCell<Node<T>>>){
        let next_node = node_to_remove.borrow().next.clone();
        let previous_node = node_to_remove.borrow().previous.clone();

        if previous_node.is_none() && next_node.is_none() {
            self.head = None;
            self.tail = None;
            self.size -= 1;
            return
        }

        if previous_node.is_none() {
            if let Some(node) = next_node.as_ref(){
                node.borrow_mut().previous = None;
                self.head = next_node.clone();
            }
        } else {
            if let Some(node) = previous_node.as_ref().and_then(|weak| weak.upgrade()) {
                node.borrow_mut().next = next_node.clone()
            }
        }

        if next_node.is_none() {
            if let Some(node) =  previous_node.as_ref().and_then(|weak| weak.upgrade()){
                node.borrow_mut().next = None;
                self.tail = Some(node);
            }
        } else{
            if let Some(node) = next_node.as_ref() {
                node.borrow_mut().previous = previous_node
            }
        }
        node_to_remove.borrow_mut().previous = None;
        node_to_remove.borrow_mut().next = None;
        self.size -= 1;

    }

    pub fn find_first(&self, value: T) -> Option<Rc<RefCell<Node<T>>>>{
        let mut current  = self.head.clone();
        while let Some(node) = current {
            if node.borrow().value == value {
                return Some(node.clone())
            }
            current  = node.borrow().next.clone();
        }
        return None
    }

    pub fn find_all(&self, value: T) -> Vec<Rc<RefCell<Node<T>>>>{
        let mut current = self.head.clone();
        let mut nodes: Vec<Rc<RefCell<Node<T>>>> = Vec::new();
        while let Some(node) = current {
            if node.borrow().value == value {
                nodes.push(node.clone());
            }
            current = node.borrow().next.clone();
        }

        nodes
    }

    pub fn print(&self) {
        if self.head.is_some(){
            let mut current = self.head.clone();
            while let Some(node) = current {
                if node.borrow().next.is_some(){
                    print!("{:?} <--> ", node.borrow().value);
                } else {
                    print!("{:?} --> None ", node.borrow().value);
                }
                current = node.borrow().next.clone();
                
            }

        }
        println!()
    }

    pub fn remove_head(&mut self){
       if let Some(head) = self.head.clone() {
          self.remove_node(head)
       }
    }

    pub fn remove_tail(&mut self){
        if let Some(tail) = self.tail.clone(){
            self.remove_node(tail)
        }
    }

    pub fn remove_all(&mut self){
        self.head = None;
        self.tail = None;
        self.size = 0;
    }


}



#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_empty_list() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn test_single_item_list(){
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.append(1);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 1);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, list.tail.as_ref().unwrap().borrow().value);
        list.remove_head();
        assert!(list.head.is_none());
        assert!(list.tail.is_none())
    }

    #[test]
    fn test_two_item_list(){
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.append(1);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 1);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 1);
        list.append(2);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 2);
        list.remove_tail();
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 1);
    }

    #[test]
    fn test_append() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 1);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 1);
        list.append(2);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 1);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 2);
    }

    #[test]
    fn test_remove_head() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.head.clone().unwrap().borrow().value, 1);
        list.remove_head();
        assert_eq!(list.head.clone().unwrap().borrow().value, 2);
        list.remove_head();
        assert_eq!(list.head.clone().unwrap().borrow().value, 3);
    }

    #[test]
    fn test_remove_middle() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.remove_value(2);
        assert!(list.find_first(2).is_none());
        assert_eq!(list.find_first(3).as_ref().unwrap().borrow()
        .previous.as_ref().unwrap()
        .upgrade().unwrap().borrow().value, 1);
        assert_eq!(list.find_first(1).as_ref().unwrap().borrow()
        .next.as_ref().unwrap().borrow().value, 3)
        
    }

    #[test]
    fn test_remove_tail() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.remove_tail();
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 2);
        assert_eq!(list.tail.as_ref().unwrap()
        .borrow().previous.as_ref().unwrap().upgrade().unwrap().borrow().value, 1);
        assert_eq!(list.head.as_ref().unwrap()
        .borrow().next.as_ref().unwrap().borrow().value, 2);
    }

    #[test]
    fn test_remove_node() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        let node_to_remove = list.find_first(2).unwrap();
        list.remove_node(node_to_remove);
        assert!(list.find_first(2).is_none());
    }

    #[test]
    fn test_find_first() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        assert!(list.find_first(1).is_some());
        assert!(list.find_first(2).is_some());
        assert!(list.find_first(3).is_none());
    }

    #[test]
    fn test_find_all() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(2);
        list.append(3);
        let found = list.find_all(2);
        assert_eq!(found.len(), 2);
        for node in found {
            assert_eq!(node.borrow().value, 2);
        }
        assert!(list.find_all(4).is_empty());
    }

    #[test]
    fn test_remove_all(){
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 1);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 4);
        list.remove_all();
        assert!(list.head.as_ref().is_none());
        assert!(list.tail.as_ref().is_none());
        assert_eq!(list.size, 0)
    }

    #[test]
    fn test_remove_from_middle(){
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        
        list.remove_value(3);
        list.remove_value(4);

        assert_eq!(list.tail.as_ref().unwrap()
        .borrow().previous.as_ref().unwrap()
        .upgrade().as_ref().unwrap().borrow()
        .value, 2);

        assert_eq!(list.find_first(2).as_ref().unwrap()
        .borrow().next.as_ref().unwrap().as_ref().borrow()
        .value, 5);

    }
    #[test]
    fn test_length(){
        let mut list = DoublyLinkedList::<i32>::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
    
        assert_eq!(list.size, 5);

        list.remove_value(1);
        list.remove_value(2);
        list.remove_value(3);
        list.remove_value(4);
        list.remove_value(5);

        assert_eq!(list.size, 0);

    }
}
