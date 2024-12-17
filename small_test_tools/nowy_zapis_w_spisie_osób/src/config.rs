// ukształtowywanie

const LICZBA_NAGŁÓWKÓW_VCF: usize= 6;
const LICZBA_ZAWARTYCH_WIADOMOŚCI: usize = LICZBA_NAGŁÓWKÓW_VCF - 1;

pub const NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF: [&'static str; LICZBA_NAGŁÓWKÓW_VCF] = [
    "BEGIN:VCARD\nVERSION:3.0",
    "FN:",
    "N:",
    "ORG:",
    "EMAIL;TYPE=INTERNET:",
    "TEL;TYPE=CELL:",
    "END:VCARD",
];

pub const ZAWARTOŚCI_WIADOMOŚCI: [&'static str; LICZBA_ZAWARTYCH_WIADOMOŚCI] = [
    "QuickCVF - Quick way for VCF. Thank you! Dziękuję!",
    "Enter name: ",
    "Enter surname: ",
    "Enter company: ",
    "Enter email: ",
    "Enter phone numer: ",
];


pub const ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH: [&'static str; 2] = [
    "Tried to read line",
    "Exeed number of attemps. Unuble to read line. Program exit…",
];
