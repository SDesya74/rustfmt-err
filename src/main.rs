fn main() {
    let vec = Vec::<(usize,)>::new();
    let mut v: usize = 0;

    for (mut i,) in vec.iter() {
        foo(&mut v, i)
    }
}

fn foo<T>(_: &mut T, _: T) {}
