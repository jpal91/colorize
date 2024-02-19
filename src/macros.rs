#[doc(hidden)]
#[macro_export]
macro_rules! macro_colorize {

    () => {String::new()};

    ( [ $($acc:tt)* ]; $tag:ident -> $msg:expr, $($rest:tt)* ) => {
        {
            let color = $crate::color_str( $msg , stringify!($tag));
            $crate::macro_colorize!([ $($acc)* color, ]; $($rest)* )
        }
    };

    ( [ $($acc:tt)* ]; $msg:expr, $($rest:tt)* ) => {$crate::macro_colorize!([$($acc)* $msg.to_string() ,]; $($rest)*)};

    ( [ $($acc:tt)* ]; $tag:ident => $id:ident -> $msg:expr, $($rest:tt)*) => {
        paste::paste!{
            {
                let color = $crate::color_str( $msg, stringify!([<$tag $id>]));
                $crate::macro_colorize!(
                    [$($acc)* color,]; $tag => $($rest)*
                )
            }
        }
    };

    ( [ $($acc:tt)* ]; $tag:ident => $msg:expr, $($rest:tt)*) => {
        {
            let color = $crate::color_str( $msg, stringify!($tag));
            $crate::macro_colorize!(
                [$($acc)* color,]; $tag => $($rest)*
            )
        }
    };

    ( [ $($acc:tt)* ]; $tag:ident => $(,)*) => {
        [$($acc)*].join(" ")
    };

    ( [ $($acc:tt)* ]; $(,)*) =>  { [$($acc)*].join(" ") };

}
