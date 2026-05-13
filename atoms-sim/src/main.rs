struct Particle {
    x: f32, y: f32, z: f32,
    charge: i32,
    probability: f32
}

impl Particle {
    // new() c'est sans instance, display() c'est sur une instance existante

    //méthode associée (constructeur, pas de &self) / syntaxe ::
    fn new(x: f32, y: f32, z: f32, charge: i32, probability: f32) -> Particle {
        return Particle{ x: x, y: y, z: z, charge: charge, probability: probability}; // Particle { x, y, z, charge }
    }

    // Calculer la distance d'un point à l'origine
    fn distance_from_origin(&self) -> f32 {
        f32::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2)) // équivalent : (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn compute_probability(&mut self) {
        self.probability = f32::exp((-2.0*self.distance_from_origin())/0.529)
    }

    //méthode d'instance (&self = accès en lecture) / syntaxe . et reçoit &self
    fn display(&self) {
        println!("Particle(x={}, y={}, z={}, charge={}, probability={}, distance_from_origin={})", self.x, self.y, self.z, self.charge, self.probability, self.distance_from_origin());
    }
}

struct Engine {
    particles: Vec<Particle>
}

impl Engine {

    fn new() -> Engine {
        Engine{ particles: Vec::new() }
    }

    // &mut self car on modifie un vecteur (&self = lire, &mut self = modifier)
    fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

}

fn main() {

    // 1. Créer un Engine
    let mut engine = Engine::new() ;

    // 2. Créer quelques particules avec Particle::new()
    let mut particle_1 = Particle::new(0.0, 0.0, 0.0, 1, 0.5);
    let mut particle_2 = Particle::new(1.0, 1.0, 1.0, 2, 0.33);
    let mut particle_3 = Particle::new(2.0, 2.0, 2.0, 3, 0.85);

    // Calculer les probas
    particle_1.compute_probability();
    particle_2.compute_probability();
    particle_3.compute_probability();

    
    // 3. Ajouter avec add_particle()
    engine.add_particle(particle_1);
    engine.add_particle(particle_2);
    engine.add_particle(particle_3);

    // 4. Afficher chaque particule
    for p in &engine.particles {
        p.display();
    }
}
