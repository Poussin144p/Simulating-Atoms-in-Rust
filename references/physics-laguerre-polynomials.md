# Physique — Polynômes de Laguerre associés

Source: https://en.wikipedia.org/wiki/Laguerre_polynomials

## Définition
Les polynômes de Laguerre associés L_n^(α)(x) sont solutions de l'équation différentielle :

    x·y″ + (α + 1 − x)·y′ + n·y = 0

où n est un entier non négatif et α un paramètre réel.

## Formule de récurrence (pour implémentation)
    L_0^(α)(x) = 1
    L_1^(α)(x) = 1 + α − x
    L_{k+1}^(α)(x) = [(2k + 1 + α − x)·L_k^(α)(x) − (k + α)·L_{k-1}^(α)(x)] / (k + 1)

## Exemples basse ordre
    L_0^(α)(x) = 1
    L_1^(α)(x) = −x + α + 1
    L_2^(α)(x) = ½(x² − 2(α+2)x + (α+1)(α+2))

## Formule fermée
    L_n^(α)(x) = Σ_{i=0}^{n} (-1)^i · C(n+α, n-i) · x^i / i!

## Application à l'atome d'hydrogène
La composante radiale R_nℓ(r) de la fonction d'onde contient :

    L_{n-ℓ-1}^{2ℓ+1}(ρ)    où ρ = 2r/(n·a₀)

Paramètres :
- n = nombre quantique principal
- ℓ = nombre quantique azimutal
- ordre du polynôme = n - ℓ - 1
- α = 2ℓ + 1

## Rôle dans le projet
La fonction `sample_r()` utilise ces polynômes pour calculer la probabilité radiale P(r) ∝ |R_nℓ(r)|² · r².
Le CDF sampling choisit ensuite un r selon cette distribution.
