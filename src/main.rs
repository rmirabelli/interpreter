#[derive(Copy, Clone)]
enum Primitive {
    Add,
    Multiply,
    Subtract,
    Number(i32)
}

fn evaluate(array: Vec<Primitive>) -> i32 {
    // Grab a reference to the zeroth element before doing anything else.
    // This allows us to not have to unwrap the element during our match.
    // To make this more robust, we could verify that the length of the
    // vector is nonzero before starting.
    let element = &array[0];
    // Create an iterator that we will use for our evaluations. Since an
    // iterator must be mutable to use next, we've done so.
    let mut iter = array.iter();
    // Since we already have the zeroth element, we can use iter.next() to
    // get the element we're going to work with afterwards. IOW, we discard
    // the result of iter.next() because we've already got that value
    // stored in element.
    iter.next();
    // Pick the correct method of evaluation.
    match element {
        // This is a little complex.
        // Since subtraction is not commutative, we have to do things
        // a little differently.
        Primitive::Subtract => {
            // instead of working with this as subtraction alone, create
            // a vector with an add token and the rest of the array,
            // then subtract the evaluation of that new vector from
            // the first element
            if let Some(Primitive::Number(val)) = iter.next() {
                // let folded = iter.fold(0, |total, next| total + evaluate(vec![*next]));
                // *val - folded

                let prims: Vec::<Primitive> = iter.map(|x| *x).collect();
                let mut flat = vec![vec![Primitive::Add], prims];
                let coll: Vec::<&Primitive> = flat.iter().flatten().collect();
                let c2 = coll.iter().map(|x| **x);
                val - evaluate(c2.collect())
            } else {
                0
            }
        }
        // Start with zero, and add each element. iter.sum could also work,
        // but I don't want to shortcut the usefulness of folding.
        Primitive::Add => { iter.fold(0, |total, next| total + evaluate(vec![*next])) }
        // Start with 1 for multiplication, because otherwise the value will
        // always be zero.
        Primitive::Multiply => { iter.fold(1, |total, next| total * evaluate(vec![*next])) }
        // If it's a number, just return its value
        Primitive::Number(val) => *val
    }
}

fn testAddition() {
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Add);
    primitives.push(Primitive::Number(13));
    primitives.push(Primitive::Number(4));
    primitives.push(Primitive::Number(2));
    let result = evaluate(primitives);
    println!("addition, should be 19: {}", result);
}

fn testSubtraction() {
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Subtract);
    primitives.push(Primitive::Number(13));
    primitives.push(Primitive::Number(4));
    primitives.push(Primitive::Number(2));
    let result = evaluate(primitives);
    println!("subtraction, should be 7: {}", result);
}

fn testMultiplication() {
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Multiply);
    primitives.push(Primitive::Number(13));
    primitives.push(Primitive::Number(4));
    primitives.push(Primitive::Number(2));
    let result = evaluate(primitives);
    println!("multiplication, should be 104: {}", result);
}

fn main() {
    testAddition();
    testMultiplication();
    testSubtraction();
}
