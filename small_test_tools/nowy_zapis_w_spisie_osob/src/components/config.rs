// ukształtowywanie

const LICZBA_NAGŁÓWKÓW_VCF: usize= 7;
const LICZBA_ZAWARTYCH_WIADOMOŚCI: usize = LICZBA_NAGŁÓWKÓW_VCF - 1;

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
];


pub const ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH: [&'static str; 2] = [
    "Nie udało się odczytać wiersza",
    "Przekroczono ilość prób odczytu wiersza. Wychodzę…",
];
