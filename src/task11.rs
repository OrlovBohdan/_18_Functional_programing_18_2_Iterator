#[test]

/*
/* Fill in the blanks */
use std::collections::HashMap;
fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().__.collect();

    println!("{:?}",folks);
}
*/


fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];

    // З'єднуємо масиви в пари (name, age) та створюємо HashMap
    let folks: HashMap<_, _> = names.iter().zip(ages.iter()).collect();

    println!("{:?}", folks);
}
use std::collections::HashMap;


/*
names.iter().zip(ages.iter()): iter() створює ітератори для масивів names і ages.
Метод zip() об'єднує ці ітератори, створюючи пари типу (&str, &i32), де кожна пара містить ім'я та вік.

collect(): Призначений для збору пар у HashMap. Оскільки елементи ітератора — це пари посилань,
типи автоматично правильно визначаються як (&str, &i32) для створення HashMap<&str, &i32>.
*/