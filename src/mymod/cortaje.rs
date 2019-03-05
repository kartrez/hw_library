
use std::fmt::{Formatter, Display, Result};

// Кортежи могут быть использованы как аргументы функции
// и как возвращаемые значения
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` можно использовать для создания связи между кортежем и переменной
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {

    let Matrix(first, second, three, quadro) = matrix;

    return Matrix(first, three, second, quadro);
}

// Это структура используется для задания
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f:&mut Formatter) -> Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}


pub fn main() {
    // Кортеж с множеством различных типов данных
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // К значениям переменных внутри кортежа можно обратиться по индексу
    println!("первое значение длинного кортежа: {}", long_tuple.0);
    println!("второе значение длинного кортежа: {}", long_tuple.1);

    // Кортежи могут содержать в себе кортежи
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Кортежи можно напечатать
    println!("кортеж из кортежей: {:?}", tuple_of_tuples);

    // Но длинные Кортежи не могут быть напечатаны
//     let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
//     println!("слишком длинный кортеж: {:?}", too_long_tuple);
    // TODO ^ Раскомментируйте выше 2 строки, чтобы увидеть ошибку компилятораr

    let pair = (1, true);
    println!("pair хранит в себе {:?}", pair);

    println!("перевёрнутая pair будет {:?}", reverse(pair));

    // Для создания кортежа, содержащего один элемент, необходимо написать элемент и
    // поставить запятую внутри круглых скобок.
    println!("кортеж из одного элемента: {:?}", (5u32,));
    println!("просто целочисленное значение: {:?}", (5u32));

    // Кортежи можно разобрать на части (деструктурировать) для создания связи
    let tuple = (1, "привет", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{}", transpose(matrix));

}
