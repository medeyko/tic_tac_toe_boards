use std::{fs::File, io::Write};

fn main() {
    const N: usize = 3; // Размерность игрового поля
                        // Универсальность в данном случае не усложняет решение
                        // На N = 2 легче отлаживать, а потом может потребоваться N > 3
                        // Но для N > 4 будут нужны дополнительные условие, что на поле нет
                        // двух независимых выигрышных комбинаций
    const OUT_FILE_NAME: &str = "boards.txt"; // Файл с результатом

    println!("Exhaustive valid tic-tac-toe boards generator");
    println!("Outputs to '{}' file", OUT_FILE_NAME);
    println!("2022 by Vladimir Medeyko");
    println!("CC0 licensed");
    const NN: usize = N * N; // Полное количество клеток
    let mut board: [u32; NN ] = [0; NN]; // Игровое поле
                                         // Ручная реализация двухмерного массива в даном случае удобнее
    let mut valid_count = 0; // Счётчик валидных позиций
    let mut file = File::create(OUT_FILE_NAME) // Файл со списком позиций
        .expect(&format!("Can't create '{}' file", OUT_FILE_NAME));

    while {
        let (x_fill, o_fill, x_count, o_count) = (
            is_fill::<N, NN>(&board, 1), // x заполнил ряд?
            is_fill::<N, NN>(&board, 2), // o заполнил ряд?
            calc_figs::<NN>(&board, 1),  // количество x
            calc_figs::<NN>(&board, 2),  // количество o
        );
        if x_count != o_count + 1 && x_count != o_count // Неправильное число ходов
            || x_fill && o_fill // Не должны оба выиграть
            || x_fill && x_count == o_count // x выиграл не на своём ходе
            || o_fill && x_count == o_count + 1 // o выиграл не на своём ходе
        {
            // println!("INVALID BOARD\n");
        } else {
            let board_layout = print_board::<N, NN>(&board);
            writeln!(file, "{}", &board_layout).expect("Output error (board)");
            // println!("valid board");
            valid_count += 1;
            if x_fill {
                writeln!(file, "x wins").expect("Output error (x)");
            } else if o_fill {
                writeln!(file, "o wins").expect("Output error (o)");
            } else if x_count + o_count == NN {
                writeln!(file, "draw").expect("Output error (draw)");
            }
            writeln!(file).expect("Output error (ln)");
        }
        next_board::<NN>(&mut board)
    } {} // Цикл как бы 'until'
    println!("Valid boards #: {}", valid_count);
    if N == 3 {
        assert_eq!(valid_count, 5478); // Правильное количество для поля 3х3 5478
                                      // https://books.google.ru/books?id=prXDDgAAQBAJ&pg=PA21&dq=tic+tac+toe+number+of+valid+positions
    }
}

/// Проверка того, что в позиции brd есть какой-либо полный ряд фигур fig
fn is_fill<const N: usize, const NN: usize>(brd: &[u32; NN], fig: u32) -> bool {
    for i in 0..N {
        if brd[i * N .. (i + 1) * N].iter().filter(|&x| *x == fig).count() == N // Горизонтальные
            || brd.iter().skip(i).step_by(N).filter(|&x| *x == fig).count() == N // Вертикальные
        {
            return true;
        };
    }
    let (mut prim, mut sec) = (0, 0);
    for j in 0..N {
        if brd[j * N + j] == fig {
            prim += 1; // Основная диагональ
        };
        if brd[j * N + N - 1 - j] == fig {
            sec += 1; // второстепенная диагональ                                                         		
        };
    }
    if prim == N || sec == N {
        return true;
    };
    false
}

/// Подсчёт количества фигур fig в позиции brd
fn calc_figs<const NN: usize>(brd: &[u32; NN], fig: u32) -> usize {
    brd.iter().filter(|&x| *x == fig).count()
}

/// Отрисовка раскладки позиции brd в текстовую строку
fn print_board<const N: usize, const NN: usize>(brd: &[u32; NN]) -> String {
    let mut result = String::new();
    for j in 0..N {
        for i in 0..N {
            let field = brd[j * N + i];
            result.push_str(&format!(
                " {} ",
                match field {
                    0 => " ",
                    1 => "x",
                    2 => "o",
                    _ => panic!("Invalid field type"),
                }
            ));
            if i != N - 1 {
                result.push('|');
            }
        }
        result.push('\n');
        if j != N - 1 {
            for i in 0..N {
                result.push_str("---");
                if i != N - 1 {
                    result.push('+');
                }
            }
            result.push('\n');
        }
    }
    result
}

/// Генерация следующей позиции в brd
/// Возвращает false если позицмя была последней
fn next_board<const NN: usize>(brd: &mut [u32; NN]) -> bool {
    let mut k = NN - 1;
    while k > 0 && brd[k] == 2 { // Троичная арифметика
        k -= 1;
    }
    if k == 0 && brd[k] == 2 {
        false
    } else {
        brd[k] += 1;
        k += 1;
        while k < NN {
            brd[k] = 0;
            k += 1;
        }
        true
    }
}
