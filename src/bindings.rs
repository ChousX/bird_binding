use super::{Binding, UserInput};
use bevy::prelude::*;
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
