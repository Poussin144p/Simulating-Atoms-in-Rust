# Rust — Vectors (Vec<T>)

Source: https://doc.rust-lang.org/book/ch08-01-vectors.html

## Création
```rust
let v: Vec<i32> = Vec::new();       // vide, type annotation requise
let v = vec![1, 2, 3];              // macro vec!, type inféré
```

## Ajout d'éléments
```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
```
La vec doit être `mut`.

## Lecture
```rust
let third = &v[2];          // panic si hors limites
let third = v.get(2);       // retourne Option<&T>, None si hors limites
```

## Itération
```rust
for i in &v { println!("{i}"); }         // immutable
for i in &mut v { *i += 50; }            // mutable (déréférencer avec *)
```

## Règle d'emprunt
Impossible de tenir une référence immutable ET de pousser des éléments en même temps :
```rust
let first = &v[0];
v.push(6);               // ERREUR — réallocation possible invaliderait first
println!("{first}");
```

## Destruction
La vec et tout son contenu sont détruits quand elle sort du scope.

## Stocker plusieurs types
Utiliser un enum :
```rust
enum Cell { Int(i32), Float(f64), Text(String) }
let row = vec![Cell::Int(3), Cell::Text(String::from("blue"))];
```
