## Синтаксис
Usage: `text_search_app.exe --file <FILE> --word <WORD>`

## Пример использования
```
> .\text_search_app.exe --file ..\..\src\main.rs --word let
Найдено слово 'let' в строке 25:     let args = Args::parse();
Найдено слово 'let' в строке 28:     let contents = match fs::read_to_string(&args.file) {
Найдено слово 'let' в строке 37:     let mut found = false;
```