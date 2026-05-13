# Physique — Atome d'hydrogène (modèle quantique)

Source: https://en.wikipedia.org/wiki/Hydrogen_atom

## Équation de Schrödinger
L'équation de Schrödinger indépendante du temps pour l'hydrogène :

    (-ℏ²/2μ ∇² - e²/4πε₀r) ψ(r,θ,φ) = E ψ(r,θ,φ)

- ℏ = constante de Planck réduite
- μ = masse réduite électron-proton
- e = charge de l'électron
- ε₀ = permittivité du vide
- r = distance au noyau

## Fonction d'onde complète
En coordonnées sphériques, la solution se décompose en produit de composantes :

    ψ_nℓm(r,θ,φ) = R_nℓ(r) × Y_ℓm(θ,φ)

Où :
- R_nℓ(r) = composante radiale (polynômes de Laguerre associés)
- Y_ℓm(θ,φ) = harmoniques sphériques (polynômes de Legendre associés)

## Nombres quantiques

| Symbole | Nom | Valeurs | Rôle |
|---------|-----|---------|------|
| n | Principal | 1, 2, 3, ... | Niveau d'énergie, taille de l'orbitale |
| ℓ | Azimutal | 0, 1, ..., n-1 | Forme de l'orbitale (s, p, d, f) |
| m | Magnétique | -ℓ, ..., ℓ | Orientation de l'orbitale |

## Niveaux d'énergie (formule de Bohr)
    E_n = -13.6 eV / n²

## État fondamental (1s)
    ψ_1s(r) = 1/(√π · a₀^(3/2)) · e^(-r/a₀)

Densité de probabilité : |ψ_1s|² est maximale au rayon de Bohr a₀ ≈ 0.529 Å.

## Lien avec le projet
- n → rayon de l'orbitale (plus grand n = orbitale plus grande)
- ℓ, m → forme et orientation (s = sphère, p = haltère, d = trèfle)
- |ψ|² → densité de probabilité de trouver l'électron → positions des points dans la simulation
