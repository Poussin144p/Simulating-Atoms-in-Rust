# Physique — Polynômes de Legendre associés

Source: https://en.wikipedia.org/wiki/Associated_Legendre_polynomials

## Définition
Les polynômes de Legendre associés P_ℓ^m(x) sont définis par :

    P_ℓ^m(x) = (-1)^m · (1 - x²)^(m/2) · d^m/dx^m [P_ℓ(x)]

où P_ℓ(x) sont les polynômes de Legendre standards.

## Formules basse ordre

**ℓ = 0 :**
    P_0^0(x) = 1

**ℓ = 1 :**
    P_1^0(x) = x
    P_1^1(x) = -(1 - x²)^(1/2)

**ℓ = 2 :**
    P_2^0(x) = ½(3x² - 1)
    P_2^1(x) = -3x(1 - x²)^(1/2)
    P_2^2(x) = 3(1 - x²)

## Harmoniques sphériques
Les polynômes de Legendre entrent dans la définition des harmoniques sphériques Y_ℓm(θ,φ) :

    Y_ℓm(θ,φ) = √[(2ℓ+1)(ℓ-m)! / 4π(ℓ+m)!] · P_ℓ^m(cos θ) · e^(imφ)

## Application à l'atome d'hydrogène
La composante angulaire θ de la fonction d'onde utilise P_ℓ^|m|(cos θ).

La probabilité angulaire pour θ est proportionnelle à :

    P(θ) ∝ |P_ℓ^m(cos θ)|² · sin(θ)

## Rôle dans le projet
La fonction `sample_theta()` utilise ces polynômes.
La valeur φ est uniformément distribuée (aléatoire pur) — seul θ nécessite ce calcul.
