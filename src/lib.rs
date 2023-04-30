mod binding;
mod bindings;
mod key;
mod user_input;
#[macro_use]
pub mod binding_macro;
pub mod key_codes;
use bevy::{self, input::InputSystem, prelude::*};
pub use binding::Binding;
pub use bindings::Bindings;
pub use key::{Key, Keys};
pub use user_input::UserInput;

pub struct BirdBindingPlugin;
impl Plugin for BirdBindingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<UserInput>()
            .add_system(UserInput::update.after(InputSystem));
    }
}
