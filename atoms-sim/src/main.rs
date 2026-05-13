struct Particle {
    x: f32, y: f32, z: f32,
    charge: i32
}

struct Engine {
    particles: Vec<Particle>
}

fn main() {
    let particle = Particle{x: 0.0, y:0.0, z:0.0, charge: 1};

    let particles = Engine{particles: Vec::new()};
}
