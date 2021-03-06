#[macro_export]
#[doc(hidden)]
macro_rules! render {
    ($ctx:ident, $value:expr) => {
        (&($value))._sf_r_internal(&mut $ctx.buf)?
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! render_escaped {
    ($ctx:ident, $value:expr) => {
        (&($value))._sf_re_internal(&mut $ctx.buf)?
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! render_text {
    ($ctx:ident, $value:expr) => {
        $ctx.buf.push_str($value)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! render_noop {
    ($ctx:ident, $value:expr) => {};
}
