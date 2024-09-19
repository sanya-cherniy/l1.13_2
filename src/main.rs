use bloomfilter::Bloom; // используем крейт реализующий фильтр Блума
use std::io;

fn main() {
    let num_items = 100000; // указываем оценочное количество элементов
    let fp_rate = 0.001; // указываем необходимую частоту ложных срабатываний
    let mut bloom = Bloom::new_for_fp_rate(num_items, fp_rate);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // считываем строку
                                                    // проверяем уникальность строки в фильтре Блума
        if !bloom.check(&input) {
            print!("{}", input); // если строка не найдена в фильтре выводим ее в терминал и добавляем в фильтр
            bloom.set(&input);
        }
    }
}
