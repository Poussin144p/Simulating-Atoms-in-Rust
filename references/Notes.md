**Un peu de physique**

On a commencé par calculer la distance d'un point à l'origine (0, 0, 0)

Pour l'état fondamental de l'hydrogène (n=1), la densité de probabilité est :
P(r) = e^(-2r/a₀)
Où a₀ = 0.529 Å (rayon de Bohr — unité de distance en physique atomique).


Particle(x=0, y=0, z=0, charge=1, probability=1, distance_from_origin=0)
Particle(x=1, y=1, z=1, charge=2, probability=0.0014324111, distance_from_origin=1.7320508)
Particle(x=2, y=2, z=2, charge=3, probability=0.0000020518019, distance_from_origin=3.4641016)


Les valeurs sont physiquement correctes :

  - r=0 → P=1.0 : l'électron est le plus probable au noyau
  - r=1.73 → P=0.0014 : probabilité qui chute rapidement
  - r=3.46 → P=0.000002 : quasi zéro à distance

C'est la décroissance exponentielle de la fonction d'onde hydrogène — tu viens de simuler de la vraie mécanique quantique.


Les coordonnées sphériques permettent de décrire la position d'un électron autour du noyau avec trois valeurs :

  - r — distance au noyau (tu l'as déjà calculée)
  - θ (theta) — angle polaire (de 0 à π)
  - φ (phi) — angle azimutal (de 0 à 2π)

  Maintenant les nombres quantiques. Ils définissent l'état de l'électron :

  ┌─────────┬────────────┬────────────┐
  │ Symbole │    Nom     │  Valeurs   │
  ├─────────┼────────────┼────────────┤
  │ n       │ principal  │ 1, 2, 3... │
  ├─────────┼────────────┼────────────┤
  │ l       │ orbital    │ 0 à n-1    │
  ├─────────┼────────────┼────────────┤
  │ m       │ magnétique │ -l à +l    │
  └─────────┴────────────┴────────────┘

  Pour n=1 tu as l=0, m=0 — c'est l'état fondamental qu'on a simulé.
  Pour n=2 tu as l=0,1 et m=-1,0,1 — orbitales 2s et 2p.


Le C++ utilise deux polynômes séparément :

  1. Laguerre → partie radiale (dépend de r et de n, l)
  2. Legendre → partie angulaire (dépend de θ et de l, m)

  La probabilité complète combine les deux :
  P(r, θ) = R(r)² × |Y(θ, φ)|²

  C'est ce qu'on va porter en Rust. On commence par Laguerre.

  Le C++ calcule le polynôme de Laguerre par récurrence — chaque valeur dépend des deux précédentes. C'est un algorithme
   itératif, pas une formule directe.


comprendre ce qu'on calcule avant de continuer.

  ---
  Laguerre — partie radiale

  Il répond à : "à quelle distance du noyau l'électron a-t-il des chances d'être ?"

  rho = 2r / (n * a0) normalise la distance selon le niveau d'énergie n. Plus n est grand, plus l'électron est loin.

  Le polynôme de Laguerre module cette distance — il crée des couches (les nœuds radiaux). Pour n=1 l'électron est
  concentré près du noyau. Pour n=2 il y a une zone vide entre deux couches de probabilité.

  En pratique dans le code : laguerre(n-l-1, 2l+1, rho) — les paramètres k et alpha sont déterminés par n et l.

  ---
  Legendre — partie angulaire

  Il répond à : "dans quelle direction autour du noyau l'électron a-t-il des chances d'être ?"

  cos(θ) est l'angle par rapport à l'axe z. Legendre donne la forme de l'orbitale en 3D :

  - l=0 → orbitale s : sphère, probabilité identique dans toutes les directions
  - l=1 → orbitale p : forme en haltère, concentrée sur un axe
  - l=2 → orbitale d : forme en trèfle à quatre feuilles

  m choisit l'orientation de cette forme dans l'espace.

  ---
  Résumé visuel :

  P(r, θ) = [Laguerre → à quelle distance]² × [Legendre → dans quelle direction]²

  C'est ça que tu calcules dans compute_probability().

En RUST :

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