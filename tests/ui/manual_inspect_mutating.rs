//@ check-pass
#![allow(clippy::all)]
#![warn(clippy::manual_inspect)]

struct A {
    a: u32,
}

impl A {
    fn update(&mut self) {
        self.a += 1;
    }
}

fn update(value: &mut u32) {
    *value += 1;
}

fn option_field_compound_assignment() {
    let mut opt = Some(A { a: 1 });
    let _ = opt.as_mut().map(|a| {
        a.a += 1;
        a
    });
}

fn option_field_assignment() {
    let mut opt = Some(A { a: 1 });
    let _ = opt.as_mut().map(|a| {
        a.a = 456;
        a
    });
}

fn option_mutating_method() {
    let mut opt = Some(A { a: 1 });
    let _ = opt.as_mut().map(|a| {
        a.update();
        a
    });
}

fn option_mutable_borrow() {
    let mut opt = Some(A { a: 1 });
    let _ = opt.as_mut().map(|a| {
        update(&mut a.a);
        a
    });
}

fn option_nested_closure() {
    let mut opt = Some(A { a: 1 });
    let _ = opt.as_mut().map(|a| {
        let mut update = || a.a += 1;
        update();
        a
    });
}

fn option_index_compound_assignment() {
    let mut values = Some([1, 456]);
    let _ = values.as_mut().map(|a| {
        a[0] += 1;
        a
    });
}

fn main() {}
