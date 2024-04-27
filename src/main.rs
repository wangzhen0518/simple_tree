use std::fmt::Display;

struct Node<T: Display> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

const SPACE_PREFIX: &str = "    ";
const ONE_SILBING_PREFIX: &str = "└── ";
const MULTI_SILBING_PREFIX: &str = "├── ";
const UNCLE_PREFIX: &str = "│   ";

impl<T: Display> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, data: T)
    where
        T: PartialOrd,
    {
        if data < self.data {
            match self.left.as_mut() {
                Some(left) => left.insert(data),
                None => self.left = Some(Box::new(Node::new(data))),
            }
        } else {
            match self.right.as_mut() {
                Some(right) => right.insert(data),
                None => self.right = Some(Box::new(Node::new(data))),
            }
        }
    }

    fn _dfs(&self, depth: usize, silbings_cnt: u32, uncle_cnt: u32) {
        let mut cnt = self.left.is_some() as u32 + self.right.is_some() as u32;

        if depth == 0 {
            println!("{}", self.data);
        } else {
            let prefix = if silbings_cnt == 1 {
                ONE_SILBING_PREFIX
            } else {
                MULTI_SILBING_PREFIX
            };
            let content = if uncle_cnt == 1 {
                format!("{}{}{}", SPACE_PREFIX.repeat(depth - 1), prefix, self.data)
            } else {
                assert!(depth >= 2);
                format!(
                    "{}{}{}{}",
                    SPACE_PREFIX.repeat(depth - 2),
                    UNCLE_PREFIX,
                    prefix,
                    self.data
                )
            };
            println!("{}", content);
        }

        if let Some(left) = self.left.as_ref() {
            left._dfs(depth.wrapping_add(1), cnt, silbings_cnt);
            cnt -= 1;
        }

        if let Some(right) = self.right.as_ref() {
            right._dfs(depth.wrapping_add(1), cnt, silbings_cnt);
            // cnt -= 1;
        }
    }

    fn dfs(&self) {
        self._dfs(0, 1, 0);
    }
}

fn main() {
    let mut tree = Node::new(50);
    tree.insert(30);
    tree.insert(20);
    tree.insert(40);
    tree.insert(70);
    tree.insert(60);
    tree.insert(80);

    tree.dfs();
}
