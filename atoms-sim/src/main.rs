use rand::RngExt;

struct Particle {
    x: f32, y: f32, z: f32,
    charge: i32,
    probability: f32,
    n: i32, l: i32, m: i32
}

impl Particle {
    // new() c'est sans instance, display() c'est sur une instance existante

    //méthode associée (constructeur, pas de &self) / syntaxe ::
    fn new(x: f32, y: f32, z: f32, charge: i32, n: i32, l: i32, m: i32) -> Particle {
        return Particle{ x: x, y: y, z: z, charge: charge, probability: 0.0, n: n, l: l, m: m}; // Particle { x, y, z, charge }
    }

    // Calculer la distance d'un point à l'origine
    fn distance_from_origin(&self) -> f32 {
        f32::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2)) // équivalent : (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn compute_probability(&mut self) {
        let a0 = 0.529_f64;
        let r = self.distance_from_origin() as f64;
        let rho = 2.0*r / (self.n as f64 * a0);

        let l_val = laguerre(self.n - self.l - 1, 2 * self.l + 1, rho);
        let big_r = f64::exp(-rho / 2.0) * rho.powi(self.l) * l_val;

        let cos_theta = (self.z as f64) / r.max(1e-10);
        let y_val = legendre(self.l, self.m.abs(), cos_theta);

        self.probability = (big_r * big_r * y_val * y_val) as f32;
    }

    fn theta(&self) -> f32 {
        f32::acos(self.z / self.distance_from_origin())
    }

    fn phi(&self) -> f32 {
        f32::atan2(self.y, self.x)
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

fn laguerre(k: i32, alpha: i32, rho: f64) -> f64 {
    if k == 0 { return 1.0; }

    let mut lm1 = 1.0 + alpha as f64 - rho;
    if k == 1 { return lm1; }

    let mut lm2 = 1.0_f64;
    let mut l = lm1;

    for j in 2..=k {
        l = ((2.0*j as f64 - 1.0 + alpha as f64 - rho) * lm1 - (j as f64 - 1.0 + alpha as f64) * lm2)/j as f64;
        lm2 = lm1;
        lm1 = l;
    }

    return l;

}

fn legendre(l: i32, m: i32, x: f64) -> f64 {
    let mut pmm: f64 = 1.0;

    if m > 0 {
        let somx2 = f64::sqrt((1.0 - x) * (1.0 + x));
        let mut fact = 1.0;
        for j in 1..=m {
            pmm = pmm * (-fact * somx2);
            fact += 2.0;
        }
    }

    let plm: f64;
    if l == m {
        plm = pmm;
    } else {
        let mut pm1m: f64 = x * (2.0 * m as f64 + 1.0) * pmm;
        
        if l == m + 1 {
            plm = pm1m;
        } else {
            let mut pll: f64;

            for ll in m+2..=l {
                pll = ((2 * ll - 1) as f64 * x * pm1m - (ll + m - 1) as f64 * pmm) / (ll - m) as f64;
                pmm = pm1m;
                pm1m = pll;
            }
            plm = pm1m;
        }
    }
    return plm;
}


fn sample_r(n: i32, l: i32) -> f64 {
    let a0 = 0.529_f64;
    let n_points = 4096;
    let r_max = 10.0 * (n * n) as f64 * a0;
    let dr = r_max / (n_points - 1) as f64;

    // 1. Construire le PDF puis la CDF
    let mut cdf = vec![0.0_f64; n_points];
    let mut sum = 0.0_f64;

    for i in 0..n_points {
        let r = i as f64 * dr;
        let rho = 2.0 * r / (n as f64 * a0);
        let l_val = laguerre(n - l - 1, 2 * l + 1, rho);
        let big_r = f64::exp(-rho / 2.0) * rho.powi(l) * l_val;
        let pdf = r * r * big_r * big_r; // r² pondère la densité sphérique
        sum += pdf;
        cdf[i] = sum;
    }

    // 2. Normaliser la CDF entre 0 et 1
    for v in &mut cdf {
        *v /= sum;
    }

    // 3. Tirer u aléatoire et trouver le r correspondant.
    let u: f64 = rand::rng().random();
    let idx = cdf.partition_point(|&v| v < u);

    idx as f64 * dr
}


fn sample_theta(l: i32, m: i32) -> f64 {
    let n_points = 4096;
    let theta_max = std::f64::consts::PI;
    let dtheta = theta_max / (n_points - 1) as f64;

    let mut cdf = vec![0.0_f64; n_points];
    let mut sum = 0.0_f64;

    for i in 0..n_points {
        let theta = i as f64 * dtheta;
        let plm = legendre(l, m.abs(), theta.cos());
        let pdf = theta.sin() * plm * plm;
        
        sum += pdf;
        cdf[i] = sum;
    }

    for v in &mut cdf {
        *v /= sum;
    }

    let u: f64 = rand::rng().random();
    let idx = cdf.partition_point(|&v| v < u);

    return idx as f64 * dtheta;

}


fn sample_phi() -> f64 {
    rand::rng().random::<f64>() * 2.0 * std::f64::consts::PI
}


fn sample_position(n: i32, l: i32, m: i32) -> (f64, f64, f64) {
    let r = sample_r(n, l);
    let theta = sample_theta(l, m);
    let phi = sample_phi();

    let x = r * theta.sin() * phi.cos();
    let y = r * theta.sin() * phi.sin();
    let z = r * theta.cos();

    return (x, y, z);
}

fn main() {

    // 1. Créer un Engine
    let mut engine = Engine::new() ;

    // 2. Créer quelques particules avec Particle::new()
    let mut particle_1 = Particle::new(0.0, 0.0, 0.0, 1, 1, 0, 0);
    let mut particle_2 = Particle::new(1.0, 1.0, 1.0, 2, 3, 2, 2);
    let mut particle_3 = Particle::new(2.0, 2.0, 2.0, 3, 2, 1, 1); 

    // Calculer les probas
    particle_1.compute_probability();
    particle_2.compute_probability();
    particle_3.compute_probability();

    // test theta/phi
    let particle = Particle::new(1.0, 0.0, 0.0, 1, 0, 1, 1);
    println!("theta={}, phi={}", particle.theta(), particle.phi());
    
    // 3. Ajouter avec add_particle()
    engine.add_particle(particle_1);
    engine.add_particle(particle_2);
    engine.add_particle(particle_3);

    // 4. Afficher chaque particule
    for p in &engine.particles {
        p.display();
    }

    println!("{}", laguerre(0, 0, 0.0));
    println!("{}", legendre(1, 0, 1.0));  // attendu : 1.0

    println!("r samples pour n=1, l=0 :");
    for _ in 0..5 {
        println!(" r = {:.3}, theta = {:.3}", sample_r(1, 0), sample_theta(1, 0));
    }
    for _ in 0..5 {
        let (x, y, z) = sample_position(2, 1, 0);
        println!("x={:.3}, y={:.3}, z={:.3}", x, y, z);
    }
}
