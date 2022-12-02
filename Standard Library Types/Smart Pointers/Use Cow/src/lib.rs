use std::borrow::Cow;

pub fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[test]
fn case1() {
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    assert!(matches!(input, Cow::Borrowed(_)), "Sorry, your assumption is incorrect!");
    assert!(matches!(abs_all(&mut input), Cow::Borrowed(_)), "Sorry, your assumption is incorrect!");
}

#[test]
fn case2() {
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    assert!(matches!(input, Cow::Borrowed(_)), "Sorry, your assumption is incorrect!");
    assert!(matches!(abs_all(&mut input), Cow::Owned(_)), "Sorry, your assumption is incorrect!");
}

#[test]
fn case3() {
    let vec = vec![0, 1, 2];
    let mut input = Cow::from(vec);
    assert!(matches!(input, Cow::Owned(_)), "Sorry, your assumption is incorrect!");
    assert!(matches!(abs_all(&mut input), Cow::Owned(_)), "Sorry, your assumption is incorrect!");
}

#[test]
fn case4() {
    let vec = vec![-1, 0, 1];
    let mut input = Cow::from(vec);
    assert!(matches!(input, Cow::Owned(_)), "Sorry, your assumption is incorrect!");
    assert!(matches!(abs_all(&mut input), Cow::Owned(_)), "Sorry, your assumption is incorrect!");
}