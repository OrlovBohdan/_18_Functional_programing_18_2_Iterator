#[test]

/*
/* Fill in the blank */
fn main() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.__;

    if let Some(v) = values_iter.__{
        __
    }

    assert_eq!(values, vec![0, 2, 3]);
}
*/

fn main() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut(); // Ітератор, який дозволяє змінювати елементи

    if let Some(v) = values_iter.next() { // Використовуємо next для отримання елемента
        *v = 0; // Змінюємо значення елемента
    }

    assert_eq!(values, vec![0, 2, 3]); // Перевіряємо, чи змінився вектор
}


/*
values.iter_mut() — створює ітератор, який дає доступ до мутабельних посилань на елементи вектора.
values_iter.next() — витягує перший елемент і повертає Some(&mut v), якщо елемент є. Використовуємо if let для перевірки і обробки елемента.
*v = 0 — розіменовуємо мутабельне посилання на елемент і змінюємо його значення на 0.
assert_eq!(values, vec![0, 2, 3]) — перевіряємо, чи змінився вектор так, як ми очікуємо.
*/