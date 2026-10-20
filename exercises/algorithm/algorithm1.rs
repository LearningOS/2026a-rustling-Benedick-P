/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
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
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
	{
        where
        T: Ord, // 因为需要比较大小，必须加上 Ord 约束
    {
        let mut merged = LinkedList::new();
        merged.length = list_a.length + list_b.length;

        // 两个游标指针，用于遍历两个链表
        let mut ptr_a = list_a.start;
        let mut ptr_b = list_b.start;
        
        // 用于指向合并后链表的当前尾节点
        let mut tail: Option<NonNull<Node<T>>> = None;

        // 当两个链表都还有节点时，进行比较
        while let (Some(node_a), Some(node_b)) = (ptr_a, ptr_b) {
            // 安全性：node_a 和 node_b 都来自有效的堆分配节点，因此解引用是安全的
            let val_a = unsafe { &(*node_a.as_ptr()).val };
            let val_b = unsafe { &(*node_b.as_ptr()).val };

            // 挑选较小的节点
            let smaller = if val_a <= val_b {
                ptr_a = unsafe { (*node_a.as_ptr()).next }; // 移动 a 的游标
                node_a
            } else {
                ptr_b = unsafe { (*node_b.as_ptr()).next }; // 移动 b 的游标
                node_b
            };

            // 将挑选出的节点追加到合并链表的尾部
            if let Some(mut t) = tail {
                // 安全性：t 是合法的链表节点，修改它的 next 指针是安全的
                unsafe {
                    (*t.as_ptr()).next = Some(smaller);
                }
            } else {
                // 第一个被挑选出来的节点作为头节点
                merged.start = Some(smaller);
            }
            tail = Some(smaller);
        }

        // 拼接剩余部分
        let remaining = if ptr_a.is_some() { ptr_a } else { ptr_b };
        if let Some(rem) = remaining {
            if let Some(mut t) = tail {
                unsafe {
                    (*t.as_ptr()).next = Some(rem);
                }
            } else {
                // 如果一开始就有一个链表为空，直接将其设为头
                merged.start = Some(rem);
            }
        }

        // 更新合并后链表的 end 指针
        if ptr_a.is_some() {
            merged.end = list_a.end;
        } else if ptr_b.is_some() {
            merged.end = list_b.end;
        } else {
            // 如果两个链表都在循环中耗尽，那么尾部就是最后挑出的那个节点
            merged.end = tail;
        }

        merged
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
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}