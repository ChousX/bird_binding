#[macro_export]
macro_rules! bind_pressed{

    ( $( $x:expr ),* ) => {
        {Binding::Pressed(vec![$($x), *])}
    };
}

#[macro_export]
macro_rules! bind_just_pressed{
    ( $( $x:expr ),* ) => {
        {Binding::JustPressed(vec![$($x), *])}
    };
}
