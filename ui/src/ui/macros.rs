macro_rules! get_widget {
    ($builder:expr, $wtype:ty, @$name:ident, $widget:expr) => {{
        let $widget: $wtype = $builder.get_object(stringify!($name)).expect(&format!("Could not find widget \"{}\"", stringify!($name)));
        $name
    }};
}