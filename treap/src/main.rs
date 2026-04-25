use rand::random;

#[derive(Clone, Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    priority: i32,
    key: i32,
    sum: i32,
}

impl Node {
    fn new(key: i32) -> Node {
        let priority = (random::<u32>() % 1_000_000_000) as i32;
        Node {
            left: None,
            right: None,
            priority: priority,
            key,
            sum: key,
        }
    }

    fn update_sum(&mut self) {
        self.sum = self.key
            + self.left.as_ref().map_or(0, |n| n.sum)
            + self.right.as_ref().map_or(0, |n| n.sum);
    }
}

type Treap = Option<Box<Node>>;

fn split(root: Treap, x: i32) -> (Treap, Treap) {
    match root {
        None => (None, None),
        Some(mut node) => {
            if node.key < x {
                let (l, r) = split(node.right.take(), x);
                node.right = l;
                node.update_sum();
                (Some(node), r)
            } else {
                let (l, r) = split(node.left.take(), x);
                node.left = r;
                node.update_sum();
                (l, Some(node))
            }
        }
    }
}

fn join(l: Treap, r: Treap) -> Treap {
    match (l, r) {
        (None, r) => r,
        (l, None) => l,
        (Some(mut l_node), Some(mut r_node)) => {
            if l_node.priority < r_node.priority {
                l_node.right = join(l_node.right.take(), Some(r_node));
                l_node.update_sum();
                Some(l_node)
            } else {
                r_node.left = join(Some(l_node), r_node.left.take());
                r_node.update_sum();
                Some(r_node)
            }
        }
    }
}

pub fn insert(root: Treap, x: i32) -> Treap {
    let (l, r) = split(root, x);
    let new_node = Some(Box::new(Node::new(x)));
    join(join(l, new_node), r)
}

pub fn delete(root: Treap, x: i32) -> Treap {
    let (l, r) = split(root, x);
    let (mid, r) = split(r, x + 1);
    join(l, r)
}

pub fn find(root: &Treap, x: i32) -> bool {
    match root {
        None => false,
        Some(node) => {
            match x.cmp(&node.key) {
                std::cmp::Ordering::Less => find(&node.left, x),
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Greater => find(&node.right, x),
            }
        },
    }
}

fn main() {
    let mut root: Treap = None;

    root = insert(root, 10);
    root = insert(root, 20);
    root = insert(root, 5);

    println!("Has 10? {}", find(&root, 10)); // true
    root = delete(root, 10);
    println!("Has 10? {}", find(&root, 10)); // false
}
