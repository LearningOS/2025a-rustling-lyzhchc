/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn reverse(&mut self){
		// TODO1 -> 2 -> 3 -> 4 -> None  ===> 4321None
        		// TODO1 -> 2 -> 3 -> 4 -> None  ===> 4321None
                let mut prev: Option<NonNull<Node<T>>> = None;
                let orin_start = self.start;
                let mut start_temp= self.start;
                // let end_temp = self.end;
                // println!("hahahah {:?}",(*end_temp.as_ptr()).val);
                // start  -> next -> next 。。。-> end
                while start_temp != None{
                    //获取start next的值
                    let next_node = unsafe { (*start_temp.unwrap().as_ptr()).next };
                    unsafe { (*start_temp.unwrap().as_ptr()).next = prev; };
                    prev = start_temp;
                    start_temp = next_node;
                }
                self.end = orin_start;
                self.start = prev; 

        //初始状态1 -> 2 -> 3 -> 4 -> None
        //  prev = None
        // current = Some(node1)
        // new_end = Some(node1)
        //第一次循环
        // next = Some(node2)
        // 反转 node1 的指针：node1.next = prev (None)
        // prev = Some(node1)
        // current = Some(node2)
        // None <- 1    2 -> 3 -> 4 -> None

        //检查current 是否为Some
        // let mut prev: Option<NonNull<Node<T>>> = None;
        // let mut current = self.start;
        // let mut new_end = self.start;
        // while let Some(current_ptr) = current {
        //     let next = unsafe { (*current_ptr.as_ptr()).next };
        //     unsafe { (*current_ptr.as_ptr()).next = prev };
        //     prev = Some(current_ptr);
        //     current = next;
        // }

        // self.start = prev;
        // self.end = new_end;


        // let mut prev: Option<NonNull<Node<T>>> = None;
        // let mut current = self.start;
        // let original_start = self.start; // 保存原始的头节点，作为新的尾节点
    
        // while current != None {
        //     let next_node = unsafe { (*current.unwrap().as_ptr()).next };
        //     unsafe { (*current.unwrap().as_ptr()).next = prev; }
        //     prev = current;
        //     current = next_node;
        // }
    
        // self.start = prev;
        // self.end = original_start;
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![2,3,5,11,9,7];
		let reverse_vec = vec![7,9,11,5,3,2];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}

	#[test]
	fn test_reverse_linked_list_2() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![34,56,78,25,90,10,19,34,21,45];
		let reverse_vec = vec![45,21,34,19,10,90,25,78,56,34];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}
}