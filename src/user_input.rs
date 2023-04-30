use super::Binding;
use super::Bindings;
use bevy::prelude::*;
use bevy::utils::HashSet;

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
