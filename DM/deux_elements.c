/**
 * Exercice 2. (Deux éléments parmi n)
 *
 * Ce programme recherche deux éléments distincts x et y d'un ensemble S
 * tel que |x - y| ≤ (1/(n-1)) * (max(S) - min(S))
 *
 * Partie (a): Implémentation d'un algorithme naïf en O(n²)
 * Partie (b): Implémentation d'un algorithme optimisé en O(n)
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>
#include <math.h>

// Structure pour représenter une paire d'éléments
typedef struct {
    int x;
    int y;
    double diff;
} Pair;

// Prototypes des fonctions
int* generate_random_array(int n, int min_val, int max_val);
void print_array(int arr[], int n);
int find_min(int arr[], int n);
int find_max(int arr[], int n);

// Question (a): Algorithme naïf en O(n²)
Pair naive_approach(int S[], int n);

// Question (b): Algorithme optimisé en O(n)
Pair optimized_approach(int S[], int n);

// Fonction de comparaison pour qsort
int compare_ints(const void* a, const void* b);

int main() {
    // Initialisation du générateur de nombres aléatoires
    srand(time(NULL));

    // Tests avec différentes tailles d'ensemble
    int test_sizes[] = {10, 100, 1000, 10000, 100000};
    int num_tests = sizeof(test_sizes) / sizeof(test_sizes[0]);

    for (int t = 0; t < num_tests; t++) {
        int n = test_sizes[t];
        printf("\n=== Test avec n = %d ===\n", n);

        // Génération d'un ensemble aléatoire
        int* S = generate_random_array(n, 0, 1000000);

        // Trouver min et max
        int min_val = find_min(S, n);
        int max_val = find_max(S, n);
        double threshold = (double)(max_val - min_val) / (n - 1);

        printf("Min = %d, Max = %d\n", min_val, max_val);
        printf("Seuil = %f\n", threshold);

        // Si l'ensemble est petit, l'afficher
        if (n <= 20) {
            printf("Ensemble S: ");
            print_array(S, n);
        }

        // (a) Approche naïve
        clock_t start_naive = clock();
        Pair result_naive = naive_approach(S, n);
        clock_t end_naive = clock();
        double time_naive = (double)(end_naive - start_naive) / CLOCKS_PER_SEC;

        // (b) Approche optimisée
        clock_t start_opt = clock();
        Pair result_opt = optimized_approach(S, n);
        clock_t end_opt = clock();
        double time_opt = (double)(end_opt - start_opt) / CLOCKS_PER_SEC;

        // Afficher les résultats
        printf("\n(a) Approche naïve (O(n²)):\n");
        printf("   Temps d'exécution: %.6f secondes\n", time_naive);
        if (result_naive.x != result_naive.y) {
            printf("   Paire trouvée: (%d, %d) avec |x-y| = %.2f <= %.2f\n",
                   result_naive.x, result_naive.y, fabs(result_naive.diff), threshold);
        } else {
            printf("   Aucune paire trouvée\n");
        }

        printf("\n(b) Approche optimisée (O(n)):\n");
        printf("   Temps d'exécution: %.6f secondes\n", time_opt);
        if (result_opt.x != result_opt.y) {
            printf("   Paire trouvée: (%d, %d) avec |x-y| = %.2f <= %.2f\n",
                   result_opt.x, result_opt.y, fabs(result_opt.diff), threshold);
        } else {
            printf("   Aucune paire trouvée\n");
        }

        // Libérer la mémoire
        free(S);
    }

    return 0;
}

/**
 * Génère un tableau d'entiers aléatoires distincts
 */
int* generate_random_array(int n, int min_val, int max_val) {
    int* arr = (int*)malloc(n * sizeof(int));

    // Nous voulons des entiers distincts, donc on s'assure que la plage est assez grande
    if (max_val - min_val + 1 < n) {
        printf("Erreur: Plage trop petite pour générer %d entiers distincts\n", n);
        exit(1);
    }

    // Utiliser un tableau pour suivre les nombres déjà utilisés
    bool* used = (bool*)calloc(max_val - min_val + 1, sizeof(bool));

    for (int i = 0; i < n; i++) {
        int num;
        do {
            num = min_val + rand() % (max_val - min_val + 1);
        } while (used[num - min_val]);

        arr[i] = num;
        used[num - min_val] = true;
    }

    free(used);
    return arr;
}

/**
 * Affiche les éléments d'un tableau
 */
void print_array(int arr[], int n) {
    printf("[ ");
    for (int i = 0; i < n; i++) {
        printf("%d ", arr[i]);
    }
    printf("]\n");
}

/**
 * Trouve la valeur minimale dans un tableau
 */
int find_min(int arr[], int n) {
    int min_val = arr[0];
    for (int i = 1; i < n; i++) {
        if (arr[i] < min_val) {
            min_val = arr[i];
        }
    }
    return min_val;
}

/**
 * Trouve la valeur maximale dans un tableau
 */
int find_max(int arr[], int n) {
    int max_val = arr[0];
    for (int i = 1; i < n; i++) {
        if (arr[i] > max_val) {
            max_val = arr[i];
        }
    }
    return max_val;
}

/**
 * Fonction de comparaison pour qsort
 */
int compare_ints(const void* a, const void* b) {
    return (*(int*)a - *(int*)b);
}

/**
 * (a) Approche naïve en O(n²)
 * Parcours toutes les paires possibles d'éléments dans S
 */
Pair naive_approach(int S[], int n) {
    Pair result = {0, 0, -1}; // Initialisation avec une différence négative (non valide)
    int min_val = find_min(S, n);
    int max_val = find_max(S, n);
    double threshold = (double)(max_val - min_val) / (n - 1);

    // Parcourir toutes les paires possibles
    for (int i = 0; i < n - 1; i++) {
        for (int j = i + 1; j < n; j++) {
            double diff = fabs((double)(S[i] - S[j]));

            // Si c'est la première paire valide ou si la différence est plus petite
            if (diff <= threshold && (result.diff < 0 || diff < result.diff)) {
                result.x = S[i];
                result.y = S[j];
                result.diff = diff;
            }
        }
    }

    return result;
}

/**
 * (b) Approche optimisée en O(n)
 * Utilise le principe des tiroirs (pigeonhole principle)
 */
Pair optimized_approach(int S[], int n) {
    Pair result = {0, 0, -1}; // Initialisation avec une différence négative (non valide)

    // Trouver min et max
    int min_val = find_min(S, n);
    int max_val = find_max(S, n);
    double range = (double)(max_val - min_val);
    double threshold = range / (n - 1);

    // Si tous les éléments sont identiques
    if (range == 0) {
        result.x = min_val;
        result.y = min_val;
        result.diff = 0;
        return result;
    }

    // Création d'une copie triée du tableau
    int* sorted_S = (int*)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        sorted_S[i] = S[i];
    }
    qsort(sorted_S, n, sizeof(int), compare_ints);

    // Recherche de deux éléments consécutifs avec une différence suffisamment petite
    for (int i = 0; i < n - 1; i++) {
        double diff = (double)(sorted_S[i+1] - sorted_S[i]);
        if (diff <= threshold && (result.diff < 0 || diff < result.diff)) {
            result.x = sorted_S[i];
            result.y = sorted_S[i+1];
            result.diff = diff;
        }
    }

    // Libérer la mémoire
    free(sorted_S);

    return result;
}