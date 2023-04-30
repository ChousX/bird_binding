use bevy::input::{keyboard::KeyCode, mouse::MouseButton};

use bevy::reflect::Enum;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Mouse(MouseButton),
    Board(KeyCode),
}

impl Key {
    pub fn is_mouse(&self) -> bool {
        match *self {
            Key::Board(_) => false,
            Key::Mouse(_) => true,
        }
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match self {
            Key::Board(b) => match other {
                Key::Board(ob) => b.cmp(ob),
                Key::Mouse(_om) => Greater,
            },
            Key::Mouse(m) => match other {
                Key::Board(_ob) => Less,
                Key::Mouse(om) => C(*m).cmp(&C(*om)),
            },
        }
    }
}

#[derive(PartialEq)]
pub struct C(pub MouseButton);

impl C {
    fn get_num(&self) -> usize {
        self.0.variant_index()
    }
}

impl PartialOrd for C {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_num().partial_cmp(&other.get_num())
    }
}

impl Eq for C {}

impl Ord for C {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_num().cmp(&other.get_num())
    }
}

pub type Keys = Vec<Key>;
