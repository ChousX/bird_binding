#[macro_use]
pub mod binding_macro;
pub mod key_codes;
use bevy::{self, input::InputSystem, prelude::*, reflect::Enum, utils::HashSet};

pub struct BirdBindingPlugin;
impl Plugin for BirdBindingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<UserInput>()
            .add_system(UserInput::update.after(InputSystem));
    }
}

#[derive(Resource, Default)]
pub struct UserInput {
    bindings: Vec<(String, Binding)>,
    action_state: HashSet<String>,
}

impl UserInput {
    pub fn update(
        mut user_input: ResMut<UserInput>,
        keys: Res<Input<KeyCode>>,
        mouse: Res<Input<MouseButton>>,
    ) {
        let mut action_sate = HashSet::new();
        for (name, binding) in user_input.bindings.iter() {
            if binding.check(&keys, &mouse) {
                action_sate.insert(name.clone());
            }
        }
        user_input.action_state = action_sate;
    }

    pub fn add_binding(&mut self, name: &impl ToString, mut binding: Binding) {
        let name = name.to_string();
        binding.sort();
        self.bindings.push((name, binding));
    }

    pub fn add_bindings(&mut self, bindings: &[(String, Binding)]) {
        for (name, binding) in bindings.iter() {
            self.add_binding(name, binding.clone())
        }
    }

    pub fn add_from_bindings(&mut self, bindings: impl Bindings) {
        self.add_bindings(&bindings.get_bindings());
    }

    pub fn find_problems(&mut self) {
        let mut seen = HashSet::new();
        for (name, binding) in self.bindings.iter() {
            let binding = binding.to_string();
            if seen.insert(binding.clone()) {
                warn!("{}: {}", name, binding);
            }
        }
    }

    pub fn check(&self, name: &str) -> bool {
        self.action_state.contains(name)
    }
}

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

type Keys = Vec<Key>;

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

pub trait Bindings: Default {
    fn get_bindings(&self) -> Vec<(String, Binding)> {
        let names = Self::binding_names().into_iter();
        let bindings = self.bindings().into_iter();
        names.zip(bindings).collect()
    }

    fn binding_names() -> Vec<String>;
    fn bindings(&self) -> Vec<Binding>;
    fn init(mut user_input: ResMut<UserInput>) {
        user_input.add_from_bindings(Self::default())
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
