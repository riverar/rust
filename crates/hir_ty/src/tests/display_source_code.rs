use super::check_types_source_code;

#[test]
fn qualify_path_to_submodule() {
    check_types_source_code(
        r#"
mod foo {
    pub struct Foo;
}

fn bar() {
    let foo: foo::Foo = foo::Foo;
    foo;
} //^^^ foo::Foo

"#,
    );
}

#[test]
fn omit_default_type_parameters() {
    check_types_source_code(
        r#"
struct Foo<T = u8> { t: T }
fn main() {
    let foo = Foo { t: 5u8 };
    foo;
} //^^^ Foo
"#,
    );

    check_types_source_code(
        r#"
struct Foo<K, T = u8> { k: K, t: T }
fn main() {
    let foo = Foo { k: 400, t: 5u8 };
    foo;
} //^^^ Foo<i32>
"#,
    );
}

#[test]
fn render_raw_ptr_impl_ty() {
    // FIXME: remove parens, they apper because there is an implicit Sized bound
    check_types_source_code(
        r#"
#[lang = "sized"] trait Sized {}
trait Unpin {}
fn foo() -> *const impl Unpin { loop {} }
fn main() {
    let foo = foo();
    foo;
} //^^^ *const (impl Unpin)
"#,
    );
}

#[test]
fn render_dyn_for_ty() {
    // FIXME
    check_types_source_code(
        r#"
trait Foo<'a> {}

fn foo(foo: &dyn for<'a> Foo<'a>) {}
    // ^^^ &dyn Foo
"#,
    );
}

#[test]
fn sized_bounds_apit() {
    check_types_source_code(
        r#"
#[lang = "sized"] trait Sized {}

trait Foo {}
trait Bar<T> {}
struct S<T>;
fn test(
    a: impl Foo,
    b: impl Foo + Sized,
    c: &(impl Foo + ?Sized),
    d: S<impl Foo>,
    ref_any: &impl ?Sized,
    empty: impl,
) {
    a;
  //^ impl Foo
    b;
  //^ impl Foo
    c;
  //^ &impl Foo + ?Sized
    d;
  //^ S<impl Foo>
    ref_any;
  //^ &impl ?Sized
    empty;
} //^ impl Sized
"#,
    );
}

#[test]
fn sized_bounds_rpit() {
    check_types_source_code(
        r#"
#[lang = "sized"] trait Sized {}

trait Foo {}
fn foo() -> impl Foo { loop {} }
fn test<T: Foo>() {
    let foo = foo();
    foo;
}   //^ impl Foo
"#,
    );
}

#[test]
fn sized_bounds_impl_traits_in_fn_signature() {
    check_types_source_code(
        r#"
#[lang = "sized"] trait Sized {}

trait Foo {}
fn test(
    a: fn(impl Foo) -> impl Foo,
    b: fn(impl Foo + Sized) -> impl Foo + Sized,
    c: fn(&(impl Foo + ?Sized)) -> &(impl Foo + ?Sized),
) {
    a;
  //^ fn(impl Foo) -> impl Foo
    b;
  //^ fn(impl Foo) -> impl Foo
    c;
} //^ fn(&impl Foo + ?Sized) -> &impl Foo + ?Sized
"#,
    );
}
