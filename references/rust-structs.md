# Rust — Structs

Source: https://doc.rust-lang.org/book/ch05-01-defining-structs.html

## Définition
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

## Instanciation
```rust
let user1 = User {
    active: true,
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    sign_in_count: 1,
};
```

## Accès et mutation
```rust
let mut user1 = User { ... }; // toute la struct doit être mut
user1.email = String::from("new@email.com");
```

## Shorthand (si le paramètre a le même nom que le champ)
```rust
fn build_user(email: String, username: String) -> User {
    User { active: true, username, email, sign_in_count: 1 }
}
```

## Struct update syntax
```rust
let user2 = User {
    email: String::from("other@example.com"),
    ..user1  // copie les champs restants depuis user1
};
```

## Tuple structs
```rust
struct Color(i32, i32, i32);
struct Point(f32, f32, f32);

let origin = Point(0.0, 0.0, 0.0);
```

## Unit-like structs
```rust
struct AlwaysEqual; // sans champs, utile pour implémenter des traits
```

## Ownership dans les structs
Préférer les types owned (`String`) aux références (`&str`) pour éviter les lifetimes.
