use std::alloc;

// doubly linked list
#[derive(Debug)]
struct DLL {
    root: *mut Node,
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
        let dll = DLL { root, layout };
        return dll;
    }
    fn traverse<F>(&mut self, visit: F)
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
    fn delete_all(&mut self) {
        let l = self.layout; // needed because 'self' cannot be borrowed twice
        self.traverse(|n| unsafe { alloc::dealloc(n as *mut u8, l) });
    }
}
impl Drop for DLL {
    fn drop(&mut self) {
        self.delete_all();
    }
}

#[derive(Debug)]
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
    //fn Delete(&self) {
    //    self.l.unwrap().r = self.r.unwrap().l;
    //    self.r.unwrap().l = self.l.unwrap().r;
    //}
    //fn Insert(&'a mut self) {
    //    self.l = Some(self);
    //    self.r = Some(self);
    //}
}

fn main() {
    let n = Node::new('r');
    println!("{:#?}", n);
    let dll = DLL::new('r');
    println!("{:#?}", dll);
    //let ar = [1i8, 2i8, 3i8];
    //let ll = DLL::new(&ar);
    //println!("{:#?}", ar);
}
