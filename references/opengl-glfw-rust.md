# OpenGL en Rust — Crate glfw

Source: https://docs.rs/glfw/latest/glfw/

## Vue d'ensemble
Le crate `glfw` est un wrapper idiomatique pour la bibliothèque GLFW (gestion de fenêtre + contexte OpenGL).

## Types principaux
- `Glfw` : token principal obtenu via `init()` — doit rester sur le thread principal
- `Window` : wraps un `*GLFWwindow` — gère le contexte de rendu et les événements
- `Context` : trait commun aux contextes de rendu

## Initialisation type
```rust
let mut glfw = glfw::init(fail_on_errors!()).unwrap();

let (mut window, events) = glfw
    .create_window(800, 600, "Atoms Sim", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window");

window.make_current();
window.set_key_polling(true);
```

## Boucle principale
```rust
while !window.should_close() {
    // 1. Traiter les événements
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }

    // 2. Rendu OpenGL ici

    // 3. Swap buffers
    window.swap_buffers();
}
```

## Dépendances Cargo.toml
```toml
[dependencies]
glfw = "0.58"
gl = "0.14"
```

## Chargement des fonctions OpenGL
```rust
gl::load_with(|s| window.get_proc_address(s) as *const _);
```

## Alternatives
- `winit` + `glutin` : plus moderne, cross-platform, moins proche de l'OpenGL pur
- `wgpu` : abstraction GPU moderne (Vulkan/Metal/DX12), plus complexe
- `glfw` + `gl` : plus proche du code C++ original — recommandé pour ce port
