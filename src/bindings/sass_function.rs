/// Allow user to define custom functions to be called from libsass.
/// https://github.com/sass/libsass/wiki/Custom-Functions-internal


use bindings::sass_value::SassValue;

/// Trait to be implemented by providers of custom sass functions.
pub trait SassFunction: Send + Sync {
    fn custom(&self, input: &SassValue) -> SassValue;
}
