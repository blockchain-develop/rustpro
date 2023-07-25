fn main() {
    let aa: u32 = 1;
    let a = Site {
        found: &aa,
    };
    let bb: u32 = 2;
    let b = Site {
        found: &bb,
    };
    func_call_ref(&a, &b);

    println!("Hello, world!");
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Site<'a> {
    pub found: &'a u32,
}

fn func_call_ref(
    a: &Site,
    b: &Site,
) -> u32 {
    let xx = [
        *a,
        *b,
    ];
    return 0
}
