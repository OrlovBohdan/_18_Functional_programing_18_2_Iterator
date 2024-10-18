#[test]

/*

/* Fill in the blank and fix the errors */
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // The sum method will take the ownership of the iterator and iterates through the items by repeatedly calling next method
    let total = v1_iter.sum();

    assert_eq!(total, __);

    println!("{:?}, {:?}",v1, v1_iter);
}
*/
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // Створюємо ітератор для вектора

    // Обчислюємо суму елементів ітератора
    let total: i32 = v1_iter.clone().sum(); // Використовуємо clone() для збереження доступу до ітератора

    assert_eq!(total, 6); // Перевірка суми

    // Оскільки ми клонували ітератор, ми можемо його знову використовувати
    println!("{:?}", v1);
}



/*
v1_iter.clone(): Ітератор v1_iter переміщає власність, коли викликається метод sum(), тому ми використовуємо .clone(), щоб створити копію ітератора, щоб його можна було використати пізніше.

sum(): Метод sum() бере ітератор, викликає для нього метод next(), і сумує всі значення. Для того, щоб це працювало, ітератор має бути типу, який можна перетворити на число, наприклад, i32.

assert_eq!(total, 6): Сума чисел [1, 2, 3] дорівнює 6.

println!("{:?}", v1): Вектор v1 не змінюється, тому можна вивести його після обчислення суми.
*/