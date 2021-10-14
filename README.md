# mdclasses4rs
Реализация объектной модели метаданных конфигурации 1С на языке Rust.

## Отличия от существующих аналогов
Ключевые отличия от [mdclasses](https://github.com/1c-syntax/mdclasses):
* Реализация на Rust вместо Java, что потенциально даст более высокое быстродействие, меньшее потребление памяти и уберет
зависимость от Java рантайма;
* Поддержка только формата EDT;

## Особенности
1. Проект находится на ранних стадиях разработки, стабильность и совместимость API **НЕ** гарантируется;
2. Отсутствие поддержки формата конфигуратора является осознанным решением и **НЕ** подлежит пересмотру;
3. Возможность сериалзиации данных рассматривается ПОСЛЕ реализации полной объектной модели;
4. Сравнительных бенчмарков (пока) нет.

## Прогресс метаданных
- [x] Корень
- Общие
    - [x] Подсистемы
    - [x] Общие модули
    - [x] Параметры сеанса
    - [ ] Роли
    - [ ] Общие реквизиты
    - [ ] Планы обмена
    - [ ] Критерии отбора
    - [ ] Подписки на события
    - [ ] Регламентные задания
    - [ ] Боты
    - [ ] Функциональные опции
    - [ ] Параметры ФО
    - [ ] Определяемые типы
    - [ ] Хранилища настроек
    - [ ] Общие формы
    - [ ] Общие команды
    - [ ] Группы команд
    - [ ] Общие макеты
    - [ ] Общие картинки
    - [ ] XDTO-пакеты
    - [ ] Web-сервисы
    - [ ] HTTP-сервисы
    - [ ] WS-ссылки
    - [ ] Сервисы интеграции
    - [ ] Элементы стиля
    - [ ] Стили
    - [x] Языки
- [ ] Константы
- [ ] Справочники
- [ ] Документы
- [ ] Журналы документов
- [ ] Перечисления
- [ ] Отчеты
- [ ] Обработки
- [ ] Планы видов характеристик
- [ ] Планы счетов
- [ ] Планы видов расчета
- [ ] Регистры сведений
- [ ] Регистры накопления
- [ ] Регистры бухгалтерии
- [ ] Регистры расчета
- [ ] Бизнес-процессы
- [ ] Задачи
- [ ] Внешние источники данных