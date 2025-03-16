#include <stdio.h>
#include <stdlib.h>
#include "tri_composite.h"

// Tri par insertion itératif
void insertion_sort_iterative(int arr[], const int n) {
    for (int i = 1; i < n; i++) {
        const int key = arr[i];
        int j = i - 1;

        while (j >= 0 && (++comparisons && arr[j] > key)) {
            arr[j + 1] = arr[j];
            swaps++;
            j--;
        }

        arr[j + 1] = key;
    }
}

// Tri par insertion récursif
void insertion_sort_recursive(int arr[], const int n) {
    if (n <= 1) return;

    // On trie les premiers n-1 éléments
    insertion_sort_recursive(arr, n - 1);

    // On ajoute le dernier élément dans la liste triée
    const int last = arr[n - 1];
    int j = n - 2;

    while (j >= 0 && (++comparisons && arr[j] > last)) {
        arr[j + 1] = arr[j];
        swaps++;
        j--;
    }

    arr[j + 1] = last;
}

// Tri Fusion récursif
void merge_sort_recursive(int arr[], const int n) {
    // Liste temporaire pour la fusion
    int* temp = malloc(n * sizeof(int));

    merge_sort_recursive_impl(arr, temp, 0, n - 1);

    free(temp);
}

// Tri Fusion récursif (méthode auxiliaire)
void merge_sort_recursive_impl(int arr[], int temp[], const int left, const int right) {
    if (left < right) {
        const int mid = left + (right - left) / 2;

        // On trie les deux côtés
        merge_sort_recursive_impl(arr, temp, left, mid);
        merge_sort_recursive_impl(arr, temp, mid + 1, right);

        // On fusionne les résultats des tris
        merge(arr, temp, left, mid, right);
    }
}

// Fonction de fusion pour le tri fusion
void merge(int arr[], int temp[], const int left, const int mid, const int right) {
    copy_array(arr, temp, right-left);

    int i = left;
    int j = mid + 1;
    int k = left;

    // Fusion des sub-listes
    while (i <= mid && j <= right) {
        comparisons++;
        if (temp[i] <= temp[j]) {
            arr[k++] = temp[i++];
        } else {
            arr[k++] = temp[j++];
        }
        swaps++;
    }

    // On copie les éléments restants de la liste gauche
    while (i <= mid) {
        arr[k++] = temp[i++];
        swaps++;
    }

    // On copie les éléments restants de la liste droite
    while (j <= right) {
        arr[k++] = temp[j++];
        swaps++;
    }
}

// Tri Fusion itératif
void merge_sort_iterative(int arr[], int n) {
    int* temp = malloc(n * sizeof(int));

    // On commence avec les listes de taille 1 et on continue de doubler
    for (int curr_size = 1; curr_size < n; curr_size = 2 * curr_size) {
        for (int left_start = 0; left_start < n - 1; left_start += 2 * curr_size) {

            // On trouve les points du milieu et de la fin
            const int mid = left_start + curr_size - 1;
            const int right_end = left_start + 2 * curr_size - 1 < n - 1 ?
                            left_start + 2 * curr_size - 1 : n - 1;

            // On fusionne les listes arr[left_start...mid] et arr[mid+1...right_end]
            if (mid < right_end) {
                merge(arr, temp, left_start, mid, right_end);
            }
        }
    }
    free(temp);
}

// Tri Rapide classique (en utilisant le premier élément comme pivot)
void quick_sort_classic(int arr[], const int n) {
    quick_sort_classic_impl(arr, 0, n - 1);
}

void quick_sort_classic_impl(int arr[], const int low, const int high) {
    if (low < high) {
        // On fait une partition de la liste
        const int pivot_index = partition_classic(arr, low, high);

        // On trie les éléments avant et après la partition
        quick_sort_classic_impl(arr, low, pivot_index - 1);
        quick_sort_classic_impl(arr, pivot_index + 1, high);
    }
}

int partition_classic(int arr[], const int low, const int high) {
    // Premier élément comme pivot
    const int pivot = arr[low];
    int i = low + 1;

    for (int j = low + 1; j <= high; j++) {
        comparisons++;
        if (arr[j] < pivot) {
            // On permute arr[i] et arr[j]
            const int temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
            swaps++;
            i++;
        }
    }

    // On permute le pivot à la dernière position
    const int temp = arr[low];
    arr[low] = arr[i - 1];
    arr[i - 1] = temp;
    swaps++;

    return i - 1;
}

// Tri Rapide (médiane)
void quick_sort_median(int arr[], const int n) {
    quick_sort_median_impl(arr, 0, n - 1);
}

void quick_sort_median_impl(int arr[], const int low, const int high) {
    if (low < high) {
        // On fait une partition de la liste
        const int pivot_index = partition_median(arr, low, high);

        // On trie les éléments avant et après la partition
        quick_sort_median_impl(arr, low, pivot_index - 1);
        quick_sort_median_impl(arr, pivot_index + 1, high);
    }
}

int partition_median(int arr[], const int low, const int high) {
    // On trouve la médiane des trois (premier, milieu, dernier) pour le pivot
    const int mid = low + (high - low) / 2;

    // On trie les trois éléments
    if (++comparisons && arr[mid] < arr[low]) {
        const int temp = arr[mid];
        arr[mid] = arr[low];
        arr[low] = temp;
        swaps++;
    }

    if (++comparisons && arr[high] < arr[low]) {
        const int temp = arr[high];
        arr[high] = arr[low];
        arr[low] = temp;
        swaps++;
    }

    if (++comparisons && arr[high] < arr[mid]) {
        const int temp = arr[high];
        arr[high] = arr[mid];
        arr[mid] = temp;
        swaps++;
    }

    // La médiane est maintenant au milieu
    // On l'échange pour la partition.
    const int temp = arr[low];
    arr[low] = arr[mid];
    arr[mid] = temp;
    swaps++;

    // On utilise la médiane comme le pivot de la partition classique
    return partition_classic(arr, low, high);
}