# Physique — Orbitales atomiques

Source: https://en.wikipedia.org/wiki/Atomic_orbital

## Définition
Une orbitale atomique est une fonction mathématique décrivant la localisation et le comportement ondulatoire d'un électron. Elle donne la distribution de probabilité de présence de l'électron.

## Décomposition de la fonction d'onde
    ψ(r, θ, φ) = R(r) × Θ(θ) × Φ(φ)

- R(r) : composante radiale — dépend de la distance au noyau
- Θ(θ) × Φ(φ) = Y_ℓm(θ,φ) : harmoniques sphériques — donnent la forme

## Types d'orbitales

| ℓ | Lettre | Forme |
|---|--------|-------|
| 0 | s | Sphère |
| 1 | p | Haltère (2 lobes) |
| 2 | d | 4 lobes en trèfle (ou tore) |
| 3 | f | 7 lobes complexes |

## Densité de probabilité
    P(r,θ,φ) = |ψ(r,θ,φ)|²

Probabilité de trouver l'électron en un point donné = carré de la valeur absolue de la fonction d'onde.

## Coordonnées sphériques → cartésiennes
    x = r · sin(θ) · cos(φ)
    y = r · sin(θ) · sin(φ)
    z = r · cos(θ)

## Lien avec le projet
La simulation génère 100 000 points aléatoires dont la densité reflète |ψ|².
- Chaque point représente une position *possible* de l'électron
- L'ensemble des points forme visuellement la forme de l'orbitale
- Le sampling utilise les fonctions R(r), Θ(θ), Φ(φ) séparément (CDF sampling)
