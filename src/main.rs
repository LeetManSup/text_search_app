use clap::Parser;
use std::fs;
use std::process;

/// Структура для хранения аргументов командной строки
#[derive(Parser, Debug)]
#[command(
    name = "text_search_app",
    version = "1.0",
    author = "SergeyLutsko",
    about = "Находит слово в текстовом файле"
)]
struct Args {
    /// Путь к исследуемому файлу
    #[arg(short, long)]
    file: String,

    /// Искомое слово
    #[arg(short, long)]
    word: String,
}

fn main() {
    // Парсинг аргументов командной строки
    let args = Args::parse();

    // Чтение файла
    let contents = match fs::read_to_string(&args.file) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Ошибка чтения файла: {}", err);
            process::exit(1);
        }
    };

    // Поиск слова в содержимом файла
    let mut found = false;
    for (line_num, line) in contents.lines().enumerate() {
        if line.contains(&args.word) {
            println!(
                "Найдено слово '{}' в строке {}: {}",
                args.word,
                line_num + 1,
                line
            );
            found = true;
        }
    }

    if !found {
        println!("Слово '{}' не было найдено в выбранном файле.", args.word);
    }
}
