use std::alloc;

// doubly linked list
#[derive(Debug)]
struct DLL {
    root: *mut Node,
    last: *mut Node,
    layout: alloc::Layout,
}

impl DLL {
    fn new(c: char) -> DLL {
        let layout = alloc::Layout::new::<Node>();
        let root: *mut Node;
        unsafe {
            root = alloc::alloc(layout) as *mut Node;
            (*root).value = c;
            (*root).l = root;
            (*root).r = root;
        }
        let last = root;
        let dll = DLL { root, last, layout };
        return dll;
    }
    fn traverse_ptr<F>(&mut self, visit: F)
    where
        F: Fn(*mut Node),
    {
        let mut n = self.root;
        loop {
            println!("visiting node @ {:#?}", n);
            let next;
            unsafe {
                // must deref n before visit()-call, as visit might deallocate
                next = (*n).r;
            }
            visit(n);
            n = next;
            if n == self.root || n.is_null() {
                break;
            }
        }
    }
    fn traverse<F>(&mut self, visit: F)
    where
        F: Fn(&Node),
    {
        return self.traverse_ptr(|n| unsafe { visit(&(*n)) });
    }
    fn delete_all(&mut self) {
        let l = self.layout; // needed because 'self' cannot be borrowed twice
        self.traverse_ptr(|n| unsafe { alloc::dealloc(n as *mut u8, l) });
    }
    fn push(&mut self, c: char) -> () {
        unsafe {
            let node = alloc::alloc(self.layout) as *mut Node;
            (*node).value = c;
            (*self.last).r = node;
            (*self.root).l = node;
            (*node).l = self.last;
            (*node).r = self.root;
            self.last = node;
        }
    }
    fn pop(&mut self) -> char {
        let value: char;
        unsafe {
            (*(*self.last).l).r = (*self.last).r;
            (*(*self.last).r).l = (*self.last).l;
            value = (*self.last).value;
            let oldlast = self.last;
            self.last = (*self.last).l;
            alloc::dealloc(oldlast as *mut u8, self.layout);
        }
        return value;
    }
}
impl Drop for DLL {
    fn drop(&mut self) {
        self.delete_all();
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    l: *mut Node,
    r: *mut Node,
    value: char,
}
impl Node {
    fn new(c: char) -> Node {
        return Node {
            l: std::ptr::null_mut(),
            r: std::ptr::null_mut(),
            value: c,
        };
    }
}

fn main() {
    let n = Node::new('r');
    println!("{:#?}", n);
    let mut dll = DLL::new('r');
    println!("{:#?}", dll);
    dll.push('a');
    dll.push('b');
    dll.push('c');
    dll.pop();
    println!("{:#?}", dll);
    dll.traverse(|n| println!("{:#?}", n))
}

#[cfg(test)]
mod tests {
    use crate::DLL;
    use std::sync::Mutex;

    #[test]
    fn simple_list_1node() {
        let dll = DLL::new('r');
        assert!(!dll.root.is_null())
    }

    #[test]
    fn simple_list_push_nodes() {
        let mut dll = DLL::new('1');
        dll.push('2');
        dll.push('3');
        let v = Mutex::new(Vec::new());
        dll.traverse(|n| println!("{:#?}", n));
        dll.traverse(|n| v.lock().unwrap().push(n.value));
        let expected: Vec<char> = vec!['1', '2', '3'];
        let got = v.lock().unwrap().clone();
        assert_eq!(got, expected);
    }

    #[test]
    fn simple_list_pushandpop_nodes() {
        let mut dll = DLL::new('1');
        dll.push('2');
        dll.push('3');
        assert_eq!(dll.pop(), '3', "first pop");
        assert_eq!(dll.pop(), '2', "second pop");
    }
}
