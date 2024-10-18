#[test]

/*
/* Make it work */
use std::collections::HashMap;
fn main() {
    let names = [("sunface",18), ("sunfei",18)];
    let folks: HashMap<_, _> = names.into_iter().collect();

    println!("{:?}",folks);

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2 = v1.iter().collect();

    assert_eq!(v2, vec![1, 2, 3]);
}
*/



fn main() {
    let names = [("sunface", 18), ("sunfei", 18)];

    // Використовуємо cloned() для копіювання значень і отримання правильного типу
    let folks: HashMap<_, _> = names.iter().cloned().collect();

    println!("{:?}", folks);

    let v1: Vec<i32> = vec![1, 2, 3];

    // Колекціонуємо значення в новий вектор
    let v2: Vec<_> = v1.iter().collect();

    assert_eq!(v2, vec![&1, &2, &3]); // Порівнюємо з посиланнями

    println!("{:?}", v2);
}
use std::collections::HashMap;



/*
names.iter().cloned(): iter() дає ітератор, який повертає посилання на елементи масиву (тип &(&str, i32)). Метод cloned() створює копії значень і повертає тип (&str, i32), що вже підходить для collect().

collect(): Тепер метод collect() може правильно перетворити ітератор у HashMap, оскільки ми передаємо йому елементи типу (&str, i32).

v1.iter().collect(): Для колекціонування елементів з ітератора v1 використовується аналогічний підхід. Тут ви збираєте посилання на елементи, тому порівняння в assert_eq! потребує використання посилань на числа.
*/