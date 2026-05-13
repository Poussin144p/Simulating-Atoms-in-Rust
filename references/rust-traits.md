# Rust — Traits

Source: https://doc.rust-lang.org/book/ch10-02-traits.html

## Définition
Un trait définit un comportement partagé (similaire aux interfaces) :
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

## Implémentation
```rust
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
```

## Implémentation par défaut
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
impl Summary for NewsArticle {}  // utilise le défaut
```

## Traits comme paramètres
```rust
pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// Équivalent avec trait bound :
pub fn notify<T: Summary>(item: &T) { ... }
```

## Plusieurs traits
```rust
pub fn notify(item: &(impl Summary + Display)) { ... }
```

## Clause where (lisibilité)
```rust
fn some_fn<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ ... }
```

## Retourner un trait
```rust
fn returns_summarizable() -> impl Summary { ... }
```

## Règle de cohérence (orphan rule)
On peut implémenter un trait sur un type seulement si le trait OU le type est local à notre crate.
