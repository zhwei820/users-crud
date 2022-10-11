use std::cell::RefCell;

#[derive(Debug, Clone)]
struct A {
    b: RefCell<String>,
    c: i64,
}

impl Default for A {
    fn default() -> Self {
        A {
            b: "b".to_owned().into(),
            c: 0,
        }
    }
}
fn main() {
    let v = vec![A::default(), A::default()];
    println!("{:?}", v);

    *v.get(0).unwrap().b.borrow_mut() = "c".to_owned();
    println!("{:?}", v);
}
