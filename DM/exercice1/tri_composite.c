#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "tri_composite.h"

unsigned long comparisons = 0;
unsigned long swaps = 0;

int main() {
    // Initialisation du générateur de nombres aléatoires
    srand(time(NULL));

    const int sizes[TEST_SIZES] = {1000, 2000, 5000, 8000, 10000};

    printf("=== Tri Composite - Analyse Pratique ===\n\n");

    // ---------------------------------------------------
    // Section (a)
    // ---------------------------------------------------
    printf("1. Analyse pratique pour les cas défavorables:\n");
    printf("--------------------------------------------------\n");

    for (int i = 0; i < TEST_SIZES; i++) {
        const int size = sizes[i];
        printf("\nTaille de la liste pour ce test: %d\n", size);
        printf("----------------------------------\n");

        // La liste inverse correspond (dans la majorité des cas) au pire cas
        int* reverse_array = create_array(size, "reverse");

        // Liste qu'on utilise pour le tri (permettant de ne jamais modifier reverse_array)
        int* test_array = malloc(size * sizeof(int));

        printf("Temps d'exécution pour le cas défavorable (ordre inversé):\n");

        copy_array(reverse_array, test_array, size);
        measure_time(test_array, size, insertion_sort_iterative, "Tri par Insertion (itérative)");

        copy_array(reverse_array, test_array, size);
        measure_time(test_array, size, insertion_sort_recursive, "Tri par Insertion (récursive)");

        copy_array(reverse_array, test_array, size);
        measure_time(test_array, size, merge_sort_recursive, "Tri Fusion (récursive)");

        copy_array(reverse_array, test_array, size);
        measure_time(test_array, size, merge_sort_iterative, "Tri Fusion (itérative)");

        copy_array(reverse_array, test_array, size);
        measure_time(test_array, size, quick_sort_classic, "Tri Rapide (classique)");

        copy_array(reverse_array, test_array, size);
        measure_time(test_array, size, quick_sort_median, "Tri Rapide (médiane)");

        free(reverse_array);
        free(test_array);
    }

    // ---------------------------------------------------
    // Section (b)
    // ---------------------------------------------------
    printf("\n\n2. Analyse pratique pour le cas moyen:\n");
    printf("--------------------------------------------------\n");

    for (int i = 0; i < TEST_SIZES; i++) {
        const int size = sizes[i];
        printf("\nTaille de la liste pour ce test: %d\n", size);
        printf("----------------------------------\n");

        // Utilisation d'une liste aléatoire (cas moyen)
        int* random_array = create_array(size, "random");

        // Liste qu'on utilise pour le tri (permettant de ne jamais modifier random_array)
        int* test_array = malloc(size * sizeof(int));

        printf("Temps d'exécution pour le cas moyen (ordre aléatoire):\n");

        copy_array(random_array, test_array, size);
        measure_time(test_array, size, insertion_sort_iterative, "Tri par Insertion (itérative)");

        copy_array(random_array, test_array, size);
        measure_time(test_array, size, insertion_sort_recursive, "Tri par Insertion (récursive)");

        copy_array(random_array, test_array, size);
        measure_time(test_array, size, merge_sort_recursive, "Tri Fusion (récursive)");

        copy_array(random_array, test_array, size);
        measure_time(test_array, size, merge_sort_iterative, "Tri Fusion (itérative)");

        copy_array(random_array, test_array, size);
        measure_time(test_array, size, quick_sort_classic, "Tri Rapide (classique)");

        copy_array(random_array, test_array, size);
        measure_time(test_array, size, quick_sort_median, "Tri Rapide (médiane)");

        free(random_array);
        free(test_array);
    }

    // ---------------------------------------------------
    // Section (c)
    // ---------------------------------------------------
    printf("\n\n3. Comparaison du nombre d'opérations:\n");
    printf("--------------------------------------------------\n");

    char* array_types[] = {"sorted", "nearly_sorted", "reverse", "random"};
    const int num_types = 4;

    // Matrice de stockage pour les comparaisons et les permutations
    unsigned long comparison_counts[num_types][NUM_ALGORITHMS];
    unsigned long swap_counts[num_types][NUM_ALGORITHMS];

    // Function de pointeurs pour tous les algorithmes
    void (*sort_functions[NUM_ALGORITHMS])(int[], int) = {
        insertion_sort_iterative,
        insertion_sort_recursive,
        merge_sort_recursive,
        quick_sort_classic,
        quick_sort_median
    };

    char* algorithm_names[NUM_ALGORITHMS] = {
        "Insértion (iter)",
        "Insértion (rec)",
        "Tri Fusion",
        "Tri Rapide",
        "Tri Rapide (médiane)"
    };

    // Pour chaque type de liste (array_types)
    for (int t = 0; t < num_types; t++) {
        const int test_size = 1000;

        printf("\nType de données: %s\n", array_types[t]);
        printf("--------------------------------------------------\n");
        printf("%-20s %15s %15s\n", "Algorithme", "Comparaisons", "Permutations");
        printf("--------------------------------------------------\n");

        int* test_array = create_array(test_size, array_types[t]);
        int* temp_array = malloc(test_size * sizeof(int));

        // Pour chaque algorithme de tri
        for (int a = 0; a < NUM_ALGORITHMS; a++) {
            reset_counters();

            // On copie la liste et elle se fait trier
            copy_array(test_array, temp_array, test_size);
            sort_functions[a](temp_array, test_size);

            // On enregistre les comparaisons et permutations
            comparison_counts[t][a] = comparisons;
            swap_counts[t][a] = swaps;

            printf("%-20s %15lu %15lu\n", algorithm_names[a], comparisons, swaps);
        }

        free(test_array);
        free(temp_array);
    }

    printf("\n=== Fin de l'Analyse Pratique ===\n");

    return 0;
}
