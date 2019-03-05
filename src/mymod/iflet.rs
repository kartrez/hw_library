
// Наш пример перечисления
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    // Все переменные типа `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // Конструкция `if let` читает, как: "Если `let` деструктуризирует `number` в
    // `Some(i)`, выполнить блок (`{}`).
    if let Some(i) = number {
        println!("Соответствует {:?}!", i);
    }

    // Если нужно указать, что делать, в случае ошибки, можно добавить else:
    if let Some(i) = letter {
        println!("Соответствует {:?}!", i);
    } else {
        // Ошибка деструктуризации. Переходим к обработке ошибки.
        println!("Не соответствует числу. Давайте попробуем строку!");
    };

    // Добавляем ещё одну ситуацию несоответствия образцу.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Соответствует {:?}!", i);
    // Оцените условие `else if`, чтобы увидеть,
    // должна ли быть альтернативная ветка отказа:
    } else if i_like_letters {
        println!("Не соответствует числу. Давайте попробуем строку!");
    } else {
        // Рассматриваем ложное условие. Эта ветвь по умолчанию:
        println!("Мне не нравится сравнивать строки. Давайте возьмём смайлик :)!");
    };


        // Создание переменных примера
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Переменная a соответствует Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Переменная b не соответствует Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // Переменная c соответствует Foo::Qux, которая имеет значение
    // аналогичное Some() как в предыдущем примере:
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}