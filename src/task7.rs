#[test]

/*
struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;

    /* Implement next method */
    fn next(&mut self)
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let mut fib = fibonacci();
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
}
*/
fn main() {
    let mut fib = fibonacci();

    assert_eq!(fib.next(), Some(0));  // Перший елемент: 0
    assert_eq!(fib.next(), Some(1));  // Другий елемент: 1
    assert_eq!(fib.next(), Some(1));  // Третій елемент: 1
    assert_eq!(fib.next(), Some(2));  // Четвертий елемент: 2
    assert_eq!(fib.next(), Some(3));  // П'ятий елемент: 3
}
struct Fibonacci {
    curr: u32,
    next: u32,
}

// Реалізуємо `Iterator` для `Fibonacci`.
impl Iterator for Fibonacci {
    // Визначаємо тип елемента, який буде повертатися
    type Item = u32;

    // Реалізуємо метод `next`
    fn next(&mut self) -> Option<Self::Item> {
        // Зберігаємо поточне значення
        let curr = self.curr;

        // Оновлюємо поточні значення: нове поточне стане старим next
        self.curr = self.next;
        self.next = self.next + curr;

        // Повертаємо значення як Some
        Some(curr)
    }
}

// Функція для створення нового генератора послідовності Фібоначчі
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}




/*
Тип Item: Ми визначили тип елемента, який ітератор повертає, як u32.
Метод next:
Ми зберігаємо поточне значення у змінну curr.
Оновлюємо значення curr на значення next, а значення next на суму curr і next.
Повертаємо значення curr в обгорнутому вигляді через Some(curr).
Функція fibonacci: Вона ініціалізує початкові значення curr і next для генератора послідовності Фібоначчі.
*/