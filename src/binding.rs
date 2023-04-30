use super::{Key, Keys};
use bevy::reflect::Enum;
use bevy::{
    input::{keyboard::KeyCode, mouse::MouseButton},
    prelude::*,
};

#[derive(Debug, Clone)]
pub enum Binding {
    Pressed(Keys),
    JustPressed(Keys),
}

impl Binding {
    pub fn new(input: Vec<Key>) -> Self {
        Self::Pressed(input)
    }
    pub fn sort(&mut self) {
        let keys = match self {
            Self::Pressed(keys) => keys,
            Self::JustPressed(keys) => keys,
        };
        keys.sort()
    }
}

impl Default for Binding {
    fn default() -> Self {
        Binding::Pressed(vec![])
    }
}

impl Binding {
    pub fn check(&self, keys: &Input<KeyCode>, mouse: &Input<MouseButton>) -> bool {
        match self {
            Self::Pressed(key) => {
                for key in key.iter() {
                    if !match key {
                        Key::Mouse(key) => mouse.pressed(*key),
                        Key::Board(key) => keys.pressed(*key),
                    } {
                        return false;
                    }
                }
                true
            }
            Self::JustPressed(key) => {
                for key in key.iter() {
                    if !match key {
                        Key::Mouse(key) => mouse.just_pressed(*key),
                        Key::Board(key) => keys.just_pressed(*key),
                    } {
                        return false;
                    }
                }
                true
            }
        }
    }
    pub fn keys(&self) -> &Keys {
        match self {
            Self::JustPressed(keys) => keys,
            Self::Pressed(keys) => keys,
        }
    }
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for key in self.keys().iter() {
            match key {
                Key::Board(key) => out.push_str(&format!("b:{}|", key.variant_name())),
                Key::Mouse(key) => out.push_str(&format!("m:{}|", key.variant_name())),
            }
        }
        return write!(f, "{}", out);
    }
}
