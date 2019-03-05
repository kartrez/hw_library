
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// Единичная структура
struct Nil;

// Кортежная структура
struct Pair(i32, f64);

// Структура с двумя полями
struct Point {
    x: f64,
    y: f64,
}

// Структуры могут быть использованы как поля другой структуры
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(length: f64, height : f64) -> f64 {
    return length * height;
}

/*fn square(point : Point, length : f64) -> Rectangle {
    Rectangle{ p1:point, p2: Point{x:length, y: length}}
}*/

pub fn main() {
    // Создаём структуру с помощью короткой инициализации полей
    let name = "Петя";
    let age = 27;
    let peter = Person { name, age };

    // Дебаг вывод структуры
    println!("{:?}", peter);


    // Создаём структуру `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Получаем доступ к полям структуры `Point`
    println!("Координаты точки: ({}, {})", point.x, point.y);

    // Деструктурируем `Point` создавая связь с помощью `let`
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // инициализация структуры так же является выражением
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Создаём связь с единичной структурой
    let _nil = Nil;

    // Создаём связь с кортежной структурой
    let pair = Pair(1, 0.1);

    // Деструктурируем кортежную структуру
    let Pair(integer, decimal) = pair;

    println!("Pair хранит в себе {:?} и {:?}", integer, decimal);

    let (lenght, height) = (3.2, 4.0);

    println!("Площадь прямоугольника c высотой {} и шириной {} равна {}", lenght, height,rect_area(lenght, height));
}
