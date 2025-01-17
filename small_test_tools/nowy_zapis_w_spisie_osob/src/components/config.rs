// ukształtowywanie

const LICZBA_NAGŁÓWKÓW_VCF: usize= 7;
const LICZBA_ZAWARTYCH_WIADOMOŚCI: usize = 8;
const LICZBA_ZAWARTYCH_WIADOMOŚCI_O_BŁĄDACH: usize = 6;

pub const NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF: [&'static str; LICZBA_NAGŁÓWKÓW_VCF] = [
    "BEGIN:VCARD\nVERSION:2.0",
    "FN:",
    "N:",
    "ORG:",
    "EMAIL;TYPE=INTERNET:",
    "TEL;TYPE=CELL:",
    "END:VCARD",
];

pub const ZAWARTOŚCI_WIADOMOŚCI: [&'static str; LICZBA_ZAWARTYCH_WIADOMOŚCI] = [
    "QuickCVF - Quick way for VCF. Thank you!!",
    "Wpisz imię: ",
    "Wpisz nazwisko: ",
    "Wpisz zrzeszenie: ",
    "Wpisz pocztę: ",
    "Wpisz liczby dalnomówinika: ",
    "Pomyślnie zapisani plik CVF! ",
    "Brak katalogu docelowego. Utworzono katalog „QuickCVF”",
];

pub const ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH: [&'static str; LICZBA_ZAWARTYCH_WIADOMOŚCI_O_BŁĄDACH] = [
    "Nie udało się odczytać wiersza",
    "Przekroczono ilość prób odczytu wiersza. Wychodzę…",
    "Wiadomość sprawdzająca",
    "Nie udało się zapisać pliku. ",
    "Wystąpił niespodziewany błąd: nie można odnaleźć ścieszki do miejsca domowego użytkownika. Zapisuje plik w miejscu,  gdzie znajduje się program. ",
    "Z niewiadomych przyczyn nie można utworzyć katalogu, w którym mają być przechowywane styczności. Stąd styczności będą przechowywane w miejscu istnienia działalnika. ",
];
