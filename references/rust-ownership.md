# Rust — Ownership

Source: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

## Règles fondamentales
1. Chaque valeur a un propriétaire
2. Il ne peut y avoir qu'un seul propriétaire à la fois
3. Quand le propriétaire sort du scope, la valeur est détruite (dropped)

## Stack vs Heap
- **Stack** : taille fixe connue à la compilation, LIFO, rapide
- **Heap** : taille variable, allocation dynamique, plus lent (passage par pointeur)

## Move semantics
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 est invalidé — ownership transféré à s2
// println!("{s1}"); // ERREUR : s1 a été moved
```

## Clone (copie profonde)
```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // copie du heap, les deux restent valides
```

## Copy trait
Les types stack-only (`i32`, `f32`, `bool`, `char`, tuples de types Copy) sont copiés automatiquement :
```rust
let x = 5;
let y = x; // copie, x reste valide
```

## Ownership et fonctions
Passer une valeur à une fonction transfère l'ownership (ou copie si type Copy) :
```rust
fn takes_ownership(s: String) { }  // prend possession
fn makes_copy(x: i32) { }          // copie (i32 = Copy)
```
