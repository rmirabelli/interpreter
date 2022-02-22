enum Primitive {
    Add,
    Multiply,
    Number(i32)
}

fn eval_prim(primitive: &Primitive) -> i32 {
    match primitive {
        Primitive::Number(val) => *val,
        _ => 0
    }
}

fn evaluate(array: Vec<Primitive>) -> i32 {
    match array[0] {
        Primitive::Add => { eval_prim(&array[1]) + eval_prim(&array[2])}
        Primitive::Multiply => { eval_prim(&array[1]) * eval_prim(&array[2])}
        _ => 0
    }
}

fn main() {
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Multiply);
    primitives.push(Primitive::Number(3));
    primitives.push(Primitive::Number(4));
    let result = evaluate(primitives);
    println!("{}", result);
}
