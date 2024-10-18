#[test]

/*
/* Refactoring the following code using iterators */
fn main() {
    let arr = [0; 10];
    for i in 0..arr.len() {
        println!("{}",arr[i]);
    }
}
*/

fn main() {
    let arr = [0; 10];
    arr.iter().for_each(|&x| println!("{}", x));
}


/*
arr.iter() повертає ітератор по елементах масиву.
for_each(|&x| println!("{}", x)) застосовує функцію до кожного елемента масиву.
Оскільки ітератор по масиву повертає посилання на елементи, ми використовуємо &x, щоб розіменувати значення і передати його в println!.
*/