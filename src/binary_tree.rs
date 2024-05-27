type NodeRef<T> = Box<Node<T>>;

pub struct Node<T: std::fmt::Debug> {
    pub val: Option<T>,
    pub lef: Option<NodeRef<T>>,
    pub rig: Option<NodeRef<T>>,
}

#[allow(dead_code)]
impl<T: std::fmt::Debug> Node<T> {
    pub fn new(val: Option<T>) -> Self {
        Node {
            val,
            lef: None,
            rig: None,
        }
    }

    pub fn get_val(&self) -> Option<&T> {
        self.val.as_ref()
    }

    pub fn set_val(&mut self, val: Option<T>) {
        self.val = val
    }

    pub fn get_val_str(&self) -> String {
        match self.get_val() {
            Some(s) => format!("{:?}", s),
            None => String::from("\x1b[41m \x1b[0m"),
        }
    }

    pub fn get_left_val_str(&self) -> String {
        match self.get_left_val() {
            Some(s) => format!("{:?}", s),
            None => String::from("\x1b[41m \x1b[0m"),
        }
    }

    pub fn get_left_val(&self) -> Option<&T> {
        match &self.lef {
            Some(n) => n.get_val(),
            None => None,
        }
    }

    pub fn set_left(&mut self, val: Option<T>) {
        self.lef = Some(NodeRef::new(Node::new(val)));
    }

    pub fn get_right_val(&self) -> Option<&T> {
        match &self.rig {
            Some(n) => n.get_val(),
            None => None,
        }
    }

    pub fn get_right_val_str(&self) -> String {
        match self.get_right_val() {
            Some(s) => format!("{:?}", s),
            None => String::from("\x1b[41m \x1b[0m"),
        }
    }

    pub fn set_right(&mut self, val: Option<T>) {
        self.rig = Some(NodeRef::new(Node::new(val)));
    }

    pub fn print(&self) {
        println!(
            "left={}, self={}, right={}",
            self.get_left_val_str(),
            self.get_val_str(),
            self.get_right_val_str()
        )
    }

    pub fn tree_print(&self, space: Option<u32>) {
        let mut spaces = space.unwrap_or(0);
        spaces += 5;
        match &self.rig {
            Some(r) => r.tree_print(Some(spaces)),
            None => (),
        }
        for _ in 5..spaces {
            print!(" ")
        }
        println!("{}", self.get_val_str());

        match &self.lef {
            Some(r) => r.tree_print(Some(spaces)),
            None => (),
        }
    }

    pub fn set_by_id(&mut self, id: &[u8], val: Option<T>) -> Result<(), String> {
        if id.len() == 0 {
            return Err("There is not an id".to_string());
        }
        if id.len() == 1 {
            return match id[0] {
                0 => {
                    self.set_left(val);
                    Ok(())
                }
                1 => {
                    self.set_right(val);
                    Ok(())
                }
                b => Err(format!(
                    "An id only consists of binary digits, {} used, id is {:?}",
                    b, id
                )),
            };
        }
        return match id[0] {
            0 => {
                match self.lef {
                    Some(_) => (),
                    None => self.set_left(None),
                }
                self.lef.as_mut().unwrap().set_by_id(&id[1..], val)
            }
            1 => {
                match self.rig {
                    Some(_) => (),
                    None => self.set_right(None),
                }
                self.rig.as_mut().unwrap().set_by_id(&id[1..], val)
            }
            b => Err(format!(
                "An id only consists of binary digits, {} used, id is {:?}, failed in recursive",
                b, id
            )),
        };
    }
    pub fn get_by_id(&self, id: &[u8]) -> Result<Option<&T>, String> {
        if id.len() == 0 {
            return Ok(self.get_val());
        }
        return match id[0] {
            0 => match &self.lef {
                Some(l) => l.get_by_id(&id[1..]),
                None => Err(format!("Id Out of bounds at {:?}", id)),
            },
            1 => match &self.rig {
                Some(r) => r.get_by_id(&id[1..]),
                None => Err(format!("Id Out of bounds at {:?}", id)),
            },
            i => Err(format!(
                "An id only consists of binary digits, {} used, id is {:?}",
                i, id,
            )),
        };
    }
}

impl<T: std::fmt::Debug + std::cmp::PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        match ((&self.lef, &self.rig), (&other.lef, &other.rig)) {
            ((None, None), (None, None)) => self.get_val() == other.get_val(),
            ((Some(_), None), (None, None)) => false,
            ((None, Some(_)), (None, None)) => false,
            ((None, None), (Some(_), None)) => false,
            ((None, None), (None, Some(_))) => false,
            ((None, None), (Some(_), Some(_))) => false,
            ((Some(_), Some(_)), (None, None)) => false,
            ((None, Some(_)), (Some(_), None)) => false,
            ((Some(_), None), (None, Some(_))) => false,
            ((None, Some(_)), (Some(_), Some(_))) => false,
            ((Some(_), None), (Some(_), Some(_))) => false,
            ((Some(_), Some(_)), (Some(_), None)) => false,
            ((Some(_), Some(_)), (None, Some(_))) => false,
            ((None, Some(r1)), (None, Some(r2))) => r1 == r2,
            ((Some(l1), None), (Some(l2), None)) => l1 == l2,
            ((Some(l1), Some(r1)), (Some(l2), Some(r2))) => l1 == l2 && r1 == r2,
        }
    }
}
