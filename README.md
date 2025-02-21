# Stack-Rust af Emil, Mads, Magnus, Mikkel og Peter
Projektet implementerer en stack i Rust, hvor elementerne kan være af enhver type.

Funktionerne: push(), pop(), peek(), is_empty() og read_elements()

Funktionalitet af funktionerne:
* push(), bliver brugt til at tilføje elementer ind i vores stack. push() retunerer en Result<(), &str>, som enten kan være en succes eller en fejlmeddelelse, hvis stacken er fuld.
* pop(), bliver brugt til at se det sidste element tilføjet til stacken og derefter fjerne det element
* peek(), bliver brugt til at se det sidste element tilføjet til stacken
* is_empty(), bliver brugt til at tjekke om stacken er tom
* read_elements(), bliver brugt til at læse alle elementerne inde i stacken

Stack<T>:
- Vores stack indeholder en Vec<T>, som gør det muligt at tilføje elementer dynamisk. Den bruger også en max_size: usize, som bruges til at lave en maksimal størrelse på stacken.
- Når man opretter en stack i main kunne det se sådan her ud "let mut stack = Stack{elements: Vec::new(), max_size: 5}; dette opretter en stack med maksimalt 5 pladser.
