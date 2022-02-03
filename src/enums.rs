enum Alphabet {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

enum StatusChar {
    Gray,
    Yellow,
    Green,
}

struct Word([Alphabet; 5]);
struct Status([StatusChar; 5]);

impl Word {
    fn to_status() -> Status {
        todo!()
    }
}
