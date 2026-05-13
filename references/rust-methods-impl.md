# Rust — Méthodes et impl

Source: https://doc.rust-lang.org/book/ch05-03-method-syntax.html

## Bloc impl
Les méthodes sont définies dans un bloc `impl` :
```rust
struct Rectangle { width: u32, height: u32 }

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

let rect = Rectangle { width: 30, height: 50 };
println!("{}", rect.area()); // appel avec dot notation
```

## Le paramètre self
- `&self` — emprunt immutable (lecture seule, le plus courant)
- `&mut self` — emprunt mutable (modification)
- `self` — prend ownership (rare, détruit l'instance après)

## Plusieurs paramètres
```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

## Fonctions associées (constructeurs)
Sans `self` — appelées avec `::` :
```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

let sq = Rectangle::square(10);
```
`String::from()` et `Vec::new()` sont des fonctions associées.

## Plusieurs blocs impl
Un type peut avoir plusieurs blocs `impl` — équivalent, utilisé pour organiser le code.

## Pas d'opérateur ->
Rust fait l'auto-référencement : `rect.area()` est identique à `(&rect).area()`.
