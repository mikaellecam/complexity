#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "tri_composite.h"

// Reinitialization des compteurs
void reset_counters() {
    comparisons = 0;
    swaps = 0;
}

// Affiche d'une liste
void print_array(int arr[], int size) {
    for (int i = 0; i < size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}

// Copie la liste src dans dest
void copy_array(int src[], int dest[], int size) {
    memcpy(dest, src, size * sizeof(int));
}

// Créer différents types de liste (triée, inversée, presque triée, aléatoire)
int* create_array(int size, char* type) {
    int* arr = (int*)malloc(size * sizeof(int));

    if (strcmp(type, "sorted") == 0) {
        for (int i = 0; i < size; i++) {
            arr[i] = i;
        }
    } else if (strcmp(type, "reverse") == 0) {
        for (int i = 0; i < size; i++) {
            arr[i] = size - i - 1;
        }
    } else if (strcmp(type, "nearly_sorted") == 0) {
        for (int i = 0; i < size; i++) {
            arr[i] = i;
        }

        // On permute 5% des éléments pour rendre la liste presque triée
        int permutations = size * 0.05;
        for (int i = 0; i < permutations; i++) {
            int pos1 = rand() % size;
            int pos2 = rand() % size;
            int temp = arr[pos1];
            arr[pos1] = arr[pos2];
            arr[pos2] = temp;
        }
    } else {
        for (int i = 0; i < size; i++) {
            arr[i] = i;
        }
        shuffle_array(arr, size);
    }
    return arr;
}

// Mesure le temps d'exécution pour un algorithme de tri
void measure_time(int array[], int size,
                 void (*sort_function)(int[], int), char* sort_name) {
    reset_counters();

    clock_t start = clock();

    sort_function(array, size);

    clock_t end = clock();

    double cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;


    printf("- %s: %.6f secondes, %lu comparaisons, %lu permutations\n",
           sort_name, cpu_time_used, comparisons, swaps);
}

// Fonction pour mélanger une liste de taille n
void shuffle_array(int arr[], int n) {
    for (int i = n - 1; i > 0; i--) {
        int j = rand() % (i + 1);
        int temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
    }
}