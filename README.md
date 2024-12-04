Le principe est simple : Vous disposez d'une grille carrée, chacune des cases est remplie d'une couleur sur un nombre défini.

Une grille de carrés est donnée en entrée, colorés au hasard de plusieurs couleurs. À chaque mouvement, vous pouvez remplir la case supérieure gauche de la couleur de votre choix. Chaque case accessible depuis la case de départ par un chemin orthogonal de cases de la même couleur sera remplie de la nouvelle couleur. Au fur et à mesure, une partie de plus en plus grande de la grille est reliée à la case de départ.

Votre objectif est de faire en sorte que toute la grille soit de la même couleur, en un minimum de mouvements.
L'algorithme défini comme le meilleur vous permettra de gagner notre peluche Axolotl.

Les couleurs sont représentées par des numéros.
Situation initiale : la première case est de la couleur 1.
Premier coup : on choisit la couleur 2.
Deuxième coup : on choisit la couleur 4.
La partie continue jusqu'à ce que toute la grille soit remplie de la même couleur.

But du défi : Trouver une suite de coups à jouer pour remplir la grille en un minimum de coups.
Taille de la grille : n
Nombre de couleurs : p

Entrée fournie : fichier CSV contenant la grille.
La grille donnée est un carré de taille n, et utilise des couleurs de 0 à p exclus.

Dans l'exemple ci-dessous, la taille de la grille fait n = 4, et il y a p = 3 couleurs :
1,2,0,0
0,1,1,0
2,2,0,1
0,0,0,1

Sortie attendue : fichier CSV qui contient la liste des couleurs à jouer.

La sortie d'exemple donné ci-dessus correspond à une solution de la grille d'exemple donné en entrée. Ainsi, en jouant ces coups, on obtient la résolution par étape suivante
2,2,0,0    0,0,0,0     1,1,1,1     2,2,2,2     0,0,0,0
0,1,1,0    0,1,1,0     1,1,1,1     2,2,2,2     0,0,0,0
2,2,0,1 >  2,2,0,1 >   2,2,0,1 >   2,2,0,2 >   0,0,0,0
0,0,0,1    0,0,0,1     0,0,0,1     0,0,0,2     0,0,0,0

Analyse algo

On a besoin d'un algo:
- Pour chaque couleur il va compter les deblocages possibles
- Iteration sur notre tableau de couleurs de référence pour avoir le nombre de déblocages atteignables
- Besoin d'une estimation des formes
- Relier les formes entre elles

Grey -f- C1 -a- D1 - C1 -a'-
C2 -b'-
C3 -c'-
-g- C2 -b- D2 - C1 -a''-
C2 -c''-
C3 -c''-
-h- C3 -c- D3 - C1 -c'''-
C2 -c'''-
C3 -c'''-

MST