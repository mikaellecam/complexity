#ifndef DEUX_ELEMENTS_H
#define DEUX_ELEMENTS_H

// Structure pour représenter une paire d'éléments
typedef struct {
    int x;
    int y;
    double diff;
} Pair;

// Prototypes des fonctions
int* generate_random_array(const int n, const int min_val, const int max_val);
void print_array(int arr[], const int n);
int find_min(const int arr[], const int n);
int find_max(const int arr[], const int n);

// Question (a) : Algorithme naïf en O(n²)
Pair naive_approach(int S[], const int n);

// Question (b) : Algorithme optimisé en O(n)
Pair optimized_approach(int S[], const int n);

// Fonction de comparaison pour qsort
int compare_ints(const void* a, const void* b);

#endif // DEUX_ELEMENTS_H