pub struct Foo;

pub trait Bar {
    fn bar(&self) -> Foo;
}

/// # Examples
/// 
/// ```
/// use foo::Foo;
/// use foo::Bar;
///
/// struct MyFoo;
/// impl Bar for MyFoo {
///     fn bar(&self) -> Foo {
///         Foo
///     }
/// }
/// ```
pub fn foo() {

}
