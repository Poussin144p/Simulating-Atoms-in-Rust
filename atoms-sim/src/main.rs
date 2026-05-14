use rand::RngExt;
use glfw::{Action, Context, Key, Window};
use nalgebra_glm as glm;

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


fn build_cdf_r(n: i32, l: i32) -> (Vec<f64>, f64, f64) {
    let a0 = 0.529_f64;
    let n_points = 2048;
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

    return (cdf, dr, r_max);
}


fn sample_r(cdf: &[f64], dr: f64) -> f64 {
    let u = rand::rng().random::<f64>();
    let idx = cdf.partition_point(|&v|v < u);
    return idx as f64 * dr;
}


fn build_cdf_theta(l: i32, m: i32) -> (Vec<f64>, f64) {
    let n_points = 2048;
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

    return (cdf, dtheta);
}

fn sample_theta(cdf: &[f64], dtheta: f64) -> f64 {
    let u: f64 = rand::rng().random();
    let idx = cdf.partition_point(|&v| v < u);
    return idx as f64 * dtheta;
}


fn sample_phi() -> f64 {
    rand::rng().random::<f64>() * 2.0 * std::f64::consts::PI
}


fn sample_position(r: f64, theta: f64, phi: f64) -> (f64, f64, f64) {
    let x = r * theta.sin() * phi.cos();
    let y = r * theta.sin() * phi.sin();
    let z = r * theta.cos();

    return (x, y, z);
}


fn generate_positions(n: i32, l: i32, m: i32, count: usize) -> (Vec<f32>, f32) {
        let mut positions = Vec::new();
        let (cdf_r, dr, r_max) = build_cdf_r(n, l);
        let (cdf_theta, dtheta) = build_cdf_theta(l, m);
    
        for _ in 0..count {
            let r     = sample_r(&cdf_r, dr);
            let theta = sample_theta(&cdf_theta, dtheta);
            let phi = sample_phi();

            let (x, y, z) = sample_position(r, theta, phi);

            positions.push(x as f32);
            positions.push(y as f32);
            positions.push(z as f32);
            let (r, g, b) = inferno(x, y, z, n, l, m);
            positions.push(r as f32);
            positions.push(g as f32);
            positions.push(b as f32);
        } 
        (positions, r_max as f32)
}


fn update_orbital(
    n: i32, l: i32, m: i32,
    n_electrons: usize,
    ssbo: u32,
    window: &mut glfw::Window,
) -> (Vec<f32>, f32) {
    let (positions, r_max) = generate_positions(n, l, m, n_electrons);
    unsafe {
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
        gl::BufferData(
            gl::SHADER_STORAGE_BUFFER,
            (positions.len() * std::mem::size_of::<f32>()) as isize,
            positions.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );
    }
    println!("n={} l={} m={}", n, l, m);
    window.set_title(&format!("Atoms | n={} l={} m={}", n, l, m));
    (positions, r_max)
} 


fn heatmap_fire(t: f32) -> (f32, f32, f32) {
    let stops: [(f32, f32, f32, f32); 6] = [
        (0.0,  0.3, 0.0, 0.5),
        (0.2,  1.0, 0.0, 0.2),
        (0.4,  0.6, 0.0, 0.0),
        (0.6,  0.9, 0.4, 0.0),
        (0.8,  1.0, 0.9, 0.0),
        (1.0,  1.0, 1.0, 1.0),
    ];

    for i in 0..stops.len() - 1 {
        let (t0, r0, g0, b0) = stops[i];
        let (t1, r1, g1, b1) = stops[i+1];

        if t >= t0 && t <= t1 {
            let s = (t - t0) / (t1 - t0);
            return (r0 + s * (r1 - r0), g0 + s * (g1 - g0), b0 + s * (b1 - b0));
        }
    }
    (1.0, 1.0, 1.0)

}


fn inferno(x: f64, y: f64, z: f64, n: i32, l: i32, m: i32) -> (f32, f32, f32) {
    let a0 = 0.529_f64;
    let r = f64::sqrt(x*x + y*y + z*z).max(1e-10);
    let rho = 2.0 * r / (n as f64 * a0);

    let l_val = laguerre(n - l - 1, 2 * l + 1, rho);
    let big_r = f64::exp(-rho / 2.0) * rho.powi(l) * l_val;

    let cos_theta = z / r;
    let y_val = legendre(l, m.abs(), cos_theta);

    let prob = big_r * big_r * y_val * y_val;

    let intensity = (prob * 700.0 + 1.0).ln() / (701.0_f64).ln();
    heatmap_fire(intensity as f32)
}


fn main() {

    // Shaders ray tracing
    let vertex_src = include_str!("shaders\\vertex.glsl");

    let fragment_src = include_str!("shaders\\fragment.glsl");


    // Init GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Créer la fenêtre
    let (mut window, events) = glfw
        .create_window(800, 600, "Atoms", glfw::WindowMode::Windowed)
        .expect("Fenêtre GLFW impossible");

    window.make_current();
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    
    gl::load_with(|s| match window.get_proc_address(s) {
        Some(f) => f as *const _,
        None => std::ptr::null(),
    });

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::PROGRAM_POINT_SIZE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
    }

    let n_electrons = 100_000;
    let mut orb_n: i32 = 2;
    let mut orb_l: i32 = 1;
    let mut orb_m: i32 = 0;
    let (mut positions, mut r_max) = generate_positions(orb_n, orb_l, orb_m, n_electrons);

    window.set_title(&format!("Atoms | n={} l={} m={}", orb_n, orb_l, orb_m));


    //Créer le VBO et envoyer les données au GPU
    let mut vbo: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (positions.len() * std::mem::size_of::<f32>()) as isize, 
            positions.as_ptr() as *const _, 
            gl::STATIC_DRAW);
    }


    // Créer le VAO (Vertex Array Object) qui mémorise comment lire les données du VBO (3 floats : x, y, z)
    let mut vao: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let stride = (6 * std::mem::size_of::<f32>()) as i32;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());          // position
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<f32>()) as *const _);  // couleur
        gl::EnableVertexAttribArray(1);
    }


    // Fullscreen quad
    let quad_verts: [f32; 12] = [
        -1.0, -1.0,  1.0, -1.0,  1.0,  1.0,
        -1.0, -1.0,  1.0,  1.0, -1.0,  1.0,
    ];
    let mut quad_vao: u32 = 0;
    let mut quad_vbo: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut quad_vao);
        gl::GenBuffers(1, &mut quad_vbo);
        gl::BindVertexArray(quad_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, quad_vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
            (quad_verts.len() * std::mem::size_of::<f32>()) as isize,
            quad_verts.as_ptr() as *const _,
            gl::STATIC_DRAW);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    // SSBO — positions et couleurs des sphères (6 floats par sphère : x,y,z,r,g,b)
    let mut ssbo: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut ssbo);
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
        gl::BufferData(gl::SHADER_STORAGE_BUFFER,
            (positions.len() * std::mem::size_of::<f32>()) as isize,
            positions.as_ptr() as *const _,
            gl::DYNAMIC_DRAW);
        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, ssbo);
    }

    // Compiler et lier les shaders
    let shader_program = unsafe {
        let vs = gl::CreateShader(gl::VERTEX_SHADER);
        let src = std::ffi::CString::new(vertex_src).unwrap();
        gl::ShaderSource(vs, 1, &src.as_ptr(), std::ptr::null());
        gl::CompileShader(vs);


        let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
        let src = std::ffi::CString::new(fragment_src).unwrap();
        gl::ShaderSource(fs, 1, &src.as_ptr(), std::ptr::null());
        gl::CompileShader(fs);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
        program
    };


    // MVP
    let cam_pos_location = unsafe {
        let name = std::ffi::CString::new("cam_pos").unwrap();
        gl::GetUniformLocation(shader_program, name.as_ptr())
    };

    let n_sphere_location = unsafe {
        let name = std::ffi::CString::new("n_spheres").unwrap();
        gl::GetUniformLocation(shader_program, name.as_ptr())
    };

    let resolution_location = unsafe {
        let name = std::ffi::CString::new("resolution").unwrap();
        gl::GetUniformLocation(shader_program, name.as_ptr())
    };

    let bound_radius_location = unsafe {
        let name = std::ffi::CString::new("bound_radius").unwrap();
        gl::GetUniformLocation(shader_program, name.as_ptr())
    };

    let mut azimuth: f32 = 0.0;    // angle horizontal en radians
    let mut elevation: f32 = 0.3;  // angle vertical en radians (légèrement au-dessus)
    let mut radius: f32 = 15.0;     // distance de la caméra (ex: 15.0)
    let mut is_dragging = false;
    let mut last_mouse: (f64, f64) = (0.0, 0.0);

    // Boucle de rendu
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) => {
                    orb_n += 1;
                    orb_l = orb_l.min(orb_n - 1);
                    orb_m = orb_m.clamp(-orb_l, orb_l);
                    (positions, r_max) = update_orbital(orb_n, orb_l, orb_m, n_electrons, ssbo, &mut window);

                }
                glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) => {
                    orb_n = (orb_n - 1).max(1);
                    orb_l = orb_l.min(orb_n - 1);
                    orb_m = orb_m.clamp(-orb_l, orb_l);
                    (positions, r_max) = update_orbital(orb_n, orb_l, orb_m, n_electrons, ssbo, &mut window);

                }
                glfw::WindowEvent::Key(Key::Left, _, Action::Press, _) => {
                    orb_l = (orb_l - 1).max(0);
                    orb_m = orb_m.clamp(-orb_l, orb_l);
                    (positions, r_max) = update_orbital(orb_n, orb_l, orb_m, n_electrons, ssbo, &mut window);
                }
                glfw::WindowEvent::Key(Key::Right, _, Action::Press, _) => {
                    orb_l = (orb_l + 1).max(0);
                    orb_m = orb_m.clamp(-orb_l, orb_l);
                    (positions, r_max) = update_orbital(orb_n, orb_l, orb_m, n_electrons, ssbo, &mut window);
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                    orb_m = (orb_m - 1).clamp(-orb_l, orb_l);
                    (positions, r_max) = update_orbital(orb_n, orb_l, orb_m, n_electrons, ssbo, &mut window);
                }
                glfw::WindowEvent::Key(Key::E, _, Action::Press, _) => {
                    orb_m = (orb_m + 1).clamp(-orb_l, orb_l);
                    (positions, r_max) = update_orbital(orb_n, orb_l, orb_m, n_electrons, ssbo, &mut window);
                }

                glfw::WindowEvent::MouseButton(glfw::MouseButtonLeft, Action::Press, _) => {
                    is_dragging = true;
                }
                glfw::WindowEvent::MouseButton(glfw::MouseButtonLeft, Action::Release, _) => {
                    is_dragging = false;
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    if is_dragging {
                        let dx = (x - last_mouse.0) as f32;
                        let dy = (y - last_mouse.1) as f32;
                        azimuth -= dx * 0.005;
                        elevation = (elevation + dy * 0.005).clamp(-1.5, 1.5);
                    }
                    last_mouse = (x, y);
                }
                glfw::WindowEvent::Scroll(_, dy) => {
                    radius = (radius - dy as f32).max(1.0);
                }
                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(shader_program);
            let cam_x = radius * elevation.cos() * azimuth.sin();
            let cam_y = radius * elevation.sin();
            let cam_z = radius * elevation.cos() * azimuth.cos();
            gl::Uniform3f(cam_pos_location, cam_x, cam_y, cam_z);
            gl::Uniform1i(n_sphere_location, n_electrons as i32);
            gl::Uniform2f(resolution_location, 800.0, 600.0);
            gl::Uniform1f(bound_radius_location, r_max);
            gl::BindVertexArray(quad_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        window.swap_buffers();
    }


}
