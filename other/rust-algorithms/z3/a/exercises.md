# Zestaw 3a

1. Napisz funkcję

   ```
   fn met_newt(f: fn(f64) -> f64, fprim: fn(f64) -> f64, x0: f64, eps: f64, n: u128) -> f64
   ```

   realizującą znajdowanie przybliżonego miejsca zerowego metodą Newtona (przy założeniu, że funkcje w parametrach spełniają odpowiednie założenia: druga jest pochodną pierwszej) — w czterech wersjach:

   * iteracyjnej z pętlą loop (z ewentualnymi break continue return);
   * iteracyjnej z pętlą while (bez żadnych break continue return);
   * rekurencyjnej;
   * iteracyjnej z pętlą for (z ewentualnymi break continue return).

---

1. Write a function

   ```
   fn met_newt(f: fn(f64) -> f64, fprim: fn(f64) -> f64, x0: f64, eps: f64, n: u128) -> f64
   ```

   finding the approximate zero using Newton's method (assuming that the functions in the parameters meet the appropriate assumptions: the second one is a derivative of the first one) - in four versions:

   * iterative with a loop (with possible break continue return);
   * iterative with a while loop (without any break continue return);
   * recursive;
   * iterative with a for loop (with possible break continue return).
