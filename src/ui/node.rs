use super::{UserInterface, View};
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

mod id {
    use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

    const NEXT: AtomicUsize = ATOMIC_USIZE_INIT;

    #[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
    pub struct Id(usize);

    impl Id {
        pub fn next() -> Id {
            Id(NEXT.fetch_add(1, Ordering::Relaxed))
        }
    }
}

use id::Id;

#[derive(Debug)]
pub struct Node {
    id: Id,
    view: RefCell<Box<dyn Any>>,
    ui: Weak<UserInterface>,
    me: RefCell<Weak<Node>>,
    parent: RefCell<Option<Weak<Node>>>,
    first_child: RefCell<Option<Rc<Node>>>,
    last_child: RefCell<Option<Weak<Node>>>,
    prev_sibling: RefCell<Option<Weak<Node>>>,
    next_sibling: RefCell<Option<Rc<Node>>>,
}

impl Node {
    pub fn new<V>(view: V, ui: Rc<UserInterface>, parent: Option<Rc<Node>>) -> Rc<Node>
    where
        V: View + Any + 'static
    {
        let boxed = Box::new(view);
        let node = Rc::new(Node {
            id: Id::next(),
            view: RefCell::new(boxed),
            ui: Rc::downgrade(&ui),
            me: RefCell::new(Weak::default()),
            parent: RefCell::new(parent.map(|n| Rc::downgrade(&n))),
            first_child: RefCell::new(None),
            last_child: RefCell::new(None),
            prev_sibling: RefCell::new(None),
            next_sibling: RefCell::new(None),
        });
        *node.me.borrow_mut() = Rc::downgrade(&node);
        node
    }

    pub fn is_same(&self, other: &Node) -> bool {
        self.id == other.id
    }

    pub fn ui(&self) -> Rc<UserInterface> {
        self.ui.upgrade().unwrap()
    }

    pub fn view<V>(&self) -> Ref<V>
    where
        V: View + Any + ?Sized
    {
        unimplemented!()
        //Ref::map(self.view.borrow(), |v| &**v)
    }

    pub fn view_mut<V>(&self) -> RefMut<V>
    where
        V: View + Any + ?Sized
    {
        let view = self.view.borrow_mut();
        RefMut::map(view, |v| (&**v).downcast_mut::<V>().unwrap())
        //RefMut::map(self.view.borrow_mut(), |v| &mut **v)
    }

    pub fn parent(&self) -> Option<Rc<Node>> {
        match self.parent.borrow().as_ref() {
            None => None,
            Some(parent) => parent.upgrade(),
        }
    }

    pub fn first_child(&self) -> Option<Rc<Node>> {
        self.first_child.borrow().as_ref().map(Rc::clone)
    }

    pub fn last_child(&self) -> Option<Rc<Node>> {
        match self.last_child.borrow().as_ref() {
            None => None,
            Some(weak) => weak.upgrade(),
        }
    }

    pub fn prev_sibling(&self) -> Option<Rc<Node>> {
        match self.prev_sibling.borrow().as_ref() {
            None => None,
            Some(weak) => weak.upgrade(),
        }
    }

    pub fn next_sibling(&self) -> Option<Rc<Node>> {
        self.next_sibling.borrow().as_ref().map(Rc::clone)
    }

    pub fn has_children(&self) -> bool {
        debug_assert!(self.first_child.borrow().is_some() == self.last_child.borrow().is_none());
        self.first_child.borrow().is_some()
    }

    pub fn add_child(&self, node: &Rc<Node>, before: Option<&Rc<Node>>) {
        node.set_parent(Some(&self.me()));
        if !self.has_children() {
            assert!(before.is_none());
            self.set_first_child(Some(node));
            self.set_last_child(Some(node));
        } else {
            match before {
                Some(before) => {
                    if before.is_same(&self.first_child().unwrap()) {
                        self.set_first_child(Some(node));
                    } else {
                        let prev = before.prev_sibling().unwrap();
                        prev.set_next_sibling(Some(node));
                        node.set_prev_sibling(Some(&prev));
                    }
                    before.set_prev_sibling(Some(node));
                    node.set_next_sibling(Some(before));
                }
                None => {
                    let last_child = self.last_child().unwrap();
                    last_child.set_next_sibling(Some(node));
                    node.set_prev_sibling(Some(&last_child));
                    self.set_last_child(Some(&node));
                }
            }
        }
    }
}

impl Node
{
    fn me(&self) -> Rc<Node> {
        self.me.borrow().upgrade().unwrap()
    }
    fn set_parent(&self, node: Option<&Rc<Node>>) {
        *self.parent.borrow_mut() = node.map(Rc::downgrade);
    }
    fn set_first_child(&self, node: Option<&Rc<Node>>) {
        *self.first_child.borrow_mut() = node.map(Rc::clone);
    }
    fn set_last_child(&self, node: Option<&Rc<Node>>) {
        *self.last_child.borrow_mut() = node.map(Rc::downgrade);
    }
    fn set_prev_sibling(&self, node: Option<&Rc<Node>>) {
        *self.prev_sibling.borrow_mut() = node.map(Rc::downgrade);
    }
    fn set_next_sibling(&self, node: Option<&Rc<Node>>) {
        *self.next_sibling.borrow_mut() = node.map(Rc::clone);
    }
}
