/**
 * Tri Composite - Implementation of various sorting algorithms
 *
 * This program implements and compares different sorting algorithms:
 * - Insertion sort (iterative and recursive)
 * - Merge sort (recursive and iterative)
 * - Quick sort with different pivot selection strategies
 *
 * The program analyzes the performance of these algorithms in different cases:
 * - Favorable case (already sorted)
 * - Unfavorable case (reverse sorted)
 * - Average case (random permutation)
 *
 * Also counts the number of comparisons and swaps performed.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>

// Constants for array sizes and test types
#define MAX_SIZE 10000
#define TEST_SIZES 5
#define NUM_ALGORITHMS 5

// Counters for comparisons and swaps
unsigned long comparisons = 0;
unsigned long swaps = 0;

// Function prototypes
// Utility functions
void reset_counters();
void print_array(int arr[], int size);
void copy_array(int src[], int dest[], int size);
int* create_array(int size, char* type);
void measure_time(int array[], int size, char* array_type,
                 void (*sort_function)(int[], int), char* sort_name);

// (a) Insertion Sort: iterative and recursive versions
void insertion_sort_iterative(int arr[], int n);
void insertion_sort_recursive(int arr[], int n);

// (b) Merge Sort: recursive and iterative versions
void merge_sort_recursive(int arr[], int n);
void merge_sort_recursive_impl(int arr[], int temp[], int left, int right);
void merge(int arr[], int temp[], int left, int mid, int right);
void merge_sort_iterative(int arr[], int n);

// (c) Quick Sort with different pivot strategies
void quick_sort_classic(int arr[], int n);
void quick_sort_classic_impl(int arr[], int low, int high);
int partition_classic(int arr[], int low, int high);

void quick_sort_median(int arr[], int n);
void quick_sort_median_impl(int arr[], int low, int high);
int partition_median(int arr[], int low, int high);

// Random permutation generator for average case
void shuffle_array(int arr[], int n);

int main() {
    // Initialize random seed
    srand(time(NULL));

    // Define array sizes for testing
    int sizes[TEST_SIZES] = {1000, 2000, 5000, 8000, 10000};

    printf("=== Tri Composite - Analyse Pratique ===\n\n");

    for (int i = 0; i < TEST_SIZES; i++) {
        int size = sizes[i];
        printf("Testing with array size: %d\n", size);
        printf("----------------------------------\n");

        // Create arrays for different test cases
        int* sorted_array = create_array(size, "sorted");
        int* reverse_array = create_array(size, "reverse");
        int* random_array = create_array(size, "random");

        // Arrays to hold copies for testing
        int* test_array = (int*)malloc(size * sizeof(int));

        // (a) Part 1: Compare insertion sort versions
        printf("\n1. Insertion Sort Comparison:\n");

        // Test iterative insertion sort
        copy_array(sorted_array, test_array, size);
        measure_time(test_array, size, "sorted", insertion_sort_iterative, "Iterative");

        copy_array(reverse_array, test_array, size);
        measure_time(test_array, size, "reverse", insertion_sort_iterative, "Iterative");

        copy_array(random_array, test_array, size);
        measure_time(test_array, size, "random", insertion_sort_iterative, "Iterative");

        // Test recursive insertion sort
        copy_array(sorted_array, test_array, size);
        measure_time(test_array, size, "sorted", insertion_sort_recursive, "Recursive");

        copy_array(reverse_array, test_array, size);
        measure_time(test_array, size, "reverse", insertion_sort_recursive, "Recursive");

        copy_array(random_array, test_array, size);
        measure_time(test_array, size, "random", insertion_sort_recursive, "Recursive");

        // (b) Part 1: Compare merge sort versions
        printf("\n2. Merge Sort Comparison:\n");

        // Test recursive merge sort
        copy_array(sorted_array, test_array, size);
        measure_time(test_array, size, "sorted", merge_sort_recursive, "Recursive");

        copy_array(reverse_array, test_array, size);
        measure_time(test_array, size, "reverse", merge_sort_recursive, "Recursive");

        copy_array(random_array, test_array, size);
        measure_time(test_array, size, "random", merge_sort_recursive, "Recursive");

        // Test iterative merge sort
        copy_array(sorted_array, test_array, size);
        measure_time(test_array, size, "sorted", merge_sort_iterative, "Iterative");

        copy_array(reverse_array, test_array, size);
        measure_time(test_array, size, "reverse", merge_sort_iterative, "Iterative");

        copy_array(random_array, test_array, size);
        measure_time(test_array, size, "random", merge_sort_iterative, "Iterative");

        // (c) Part 1: Compare quicksort versions
        printf("\n3. Quick Sort Comparison:\n");

        // Test classic quicksort
        copy_array(sorted_array, test_array, size);
        measure_time(test_array, size, "sorted", quick_sort_classic, "Classic");

        copy_array(reverse_array, test_array, size);
        measure_time(test_array, size, "reverse", quick_sort_classic, "Classic");

        copy_array(random_array, test_array, size);
        measure_time(test_array, size, "random", quick_sort_classic, "Classic");

        // Test median-of-three quicksort
        copy_array(sorted_array, test_array, size);
        measure_time(test_array, size, "sorted", quick_sort_median, "Median");

        copy_array(reverse_array, test_array, size);
        measure_time(test_array, size, "reverse", quick_sort_median, "Median");

        copy_array(random_array, test_array, size);
        measure_time(test_array, size, "random", quick_sort_median, "Median");

        // Free allocated memory
        free(sorted_array);
        free(reverse_array);
        free(random_array);
        free(test_array);

        printf("\n");
    }

    // (c) Part 3: Count and compare operations for different data types
    printf("\n=== Comparison of Operation Counts ===\n");
    int test_size = 1000; // Smaller size for detailed operation counting

    // Array types to test
    char* array_types[] = {"sorted", "nearly_sorted", "reverse", "random"};
    int num_types = 4;

    // Create a table to store comparison and swap counts
    unsigned long comparison_counts[num_types][NUM_ALGORITHMS];
    unsigned long swap_counts[num_types][NUM_ALGORITHMS];

    // Function pointers for all algorithms
    void (*sort_functions[NUM_ALGORITHMS])(int[], int) = {
        insertion_sort_iterative,
        insertion_sort_recursive,
        merge_sort_recursive,
        quick_sort_classic,
        quick_sort_median
    };

    char* algorithm_names[NUM_ALGORITHMS] = {
        "Insertion (iter)",
        "Insertion (rec)",
        "Merge Sort",
        "Quick Sort",
        "Quick Sort (median)"
    };

    // For each array type
    for (int t = 0; t < num_types; t++) {
        printf("\nArray type: %s\n", array_types[t]);
        printf("--------------------------------------------------\n");
        printf("%-20s %15s %15s\n", "Algorithm", "Comparisons", "Swaps");
        printf("--------------------------------------------------\n");

        // Create the test array
        int* test_array = create_array(test_size, array_types[t]);
        int* temp_array = (int*)malloc(test_size * sizeof(int));

        // For each sorting algorithm
        for (int a = 0; a < NUM_ALGORITHMS; a++) {
            // Reset counters
            reset_counters();

            // Copy the array and sort it
            copy_array(test_array, temp_array, test_size);
            sort_functions[a](temp_array, test_size);

            // Store the counts
            comparison_counts[t][a] = comparisons;
            swap_counts[t][a] = swaps;

            // Print the results
            printf("%-20s %15lu %15lu\n", algorithm_names[a], comparisons, swaps);
        }

        // Free memory
        free(test_array);
        free(temp_array);
    }

    printf("\n=== End of Analysis ===\n");

    return 0;
}

// Reset the operation counters
void reset_counters() {
    comparisons = 0;
    swaps = 0;
}

// Print array (for debugging)
void print_array(int arr[], int size) {
    for (int i = 0; i < size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}

// Copy an array
void copy_array(int src[], int dest[], int size) {
    memcpy(dest, src, size * sizeof(int));
}

// Create different types of arrays
int* create_array(int size, char* type) {
    int* arr = (int*)malloc(size * sizeof(int));

    if (strcmp(type, "sorted") == 0) {
        // Create sorted array
        for (int i = 0; i < size; i++) {
            arr[i] = i;
        }
    } else if (strcmp(type, "reverse") == 0) {
        // Create reverse sorted array
        for (int i = 0; i < size; i++) {
            arr[i] = size - i - 1;
        }
    } else if (strcmp(type, "nearly_sorted") == 0) {
        // Create nearly sorted array (95% sorted)
        for (int i = 0; i < size; i++) {
            arr[i] = i;
        }

        // Swap about 5% of elements
        int swaps = size * 0.05;
        for (int i = 0; i < swaps; i++) {
            int pos1 = rand() % size;
            int pos2 = rand() % size;
            int temp = arr[pos1];
            arr[pos1] = arr[pos2];
            arr[pos2] = temp;
        }
    } else { // random
        // Create random array
        for (int i = 0; i < size; i++) {
            arr[i] = i;
        }
        shuffle_array(arr, size);
    }

    return arr;
}

// Measure execution time for a sorting algorithm
void measure_time(int array[], int size, char* array_type,
                 void (*sort_function)(int[], int), char* sort_name) {
    // Reset operation counters
    reset_counters();

    // Start timer
    clock_t start = clock();

    // Sort the array
    sort_function(array, size);

    // End timer
    clock_t end = clock();
    double cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;

    // Print results
    printf("- %s on %s array: %.6f seconds, %lu comparisons, %lu swaps\n",
           sort_name, array_type, cpu_time_used, comparisons, swaps);
}

/**
 * (a) Insertion Sort Implementation
 * Both iterative and recursive versions
 */

// Iterative insertion sort
void insertion_sort_iterative(int arr[], int n) {
    for (int i = 1; i < n; i++) {
        int key = arr[i];
        int j = i - 1;

        while (j >= 0 && (++comparisons && arr[j] > key)) {
            arr[j + 1] = arr[j];
            swaps++;
            j--;
        }

        arr[j + 1] = key;
    }
}

// Recursive insertion sort
void insertion_sort_recursive(int arr[], int n) {
    // Base case
    if (n <= 1) {
        return;
    }

    // Sort first n-1 elements
    insertion_sort_recursive(arr, n - 1);

    // Insert last element in sorted array
    int last = arr[n - 1];
    int j = n - 2;

    while (j >= 0 && (++comparisons && arr[j] > last)) {
        arr[j + 1] = arr[j];
        swaps++;
        j--;
    }

    arr[j + 1] = last;
}

/**
 * (b) Merge Sort Implementation
 * Both recursive and iterative versions
 */

// Recursive merge sort
void merge_sort_recursive(int arr[], int n) {
    // Temporary array for merging
    int* temp = (int*)malloc(n * sizeof(int));

    // Call the implementation
    merge_sort_recursive_impl(arr, temp, 0, n - 1);

    // Free temporary array
    free(temp);
}

// Recursive merge sort implementation
void merge_sort_recursive_impl(int arr[], int temp[], int left, int right) {
    if (left < right) {
        int mid = left + (right - left) / 2;

        // Sort left and right halves
        merge_sort_recursive_impl(arr, temp, left, mid);
        merge_sort_recursive_impl(arr, temp, mid + 1, right);

        // Merge the sorted halves
        merge(arr, temp, left, mid, right);
    }
}

// Merge function for merge sort
void merge(int arr[], int temp[], int left, int mid, int right) {
    // Copy data to temporary arrays
    for (int i = left; i <= right; i++) {
        temp[i] = arr[i];
    }

    int i = left;     // Initial index of left subarray
    int j = mid + 1;  // Initial index of right subarray
    int k = left;     // Initial index of merged subarray

    // Merge the subarrays
    while (i <= mid && j <= right) {
        comparisons++;
        if (temp[i] <= temp[j]) {
            arr[k++] = temp[i++];
        } else {
            arr[k++] = temp[j++];
        }
        swaps++;
    }

    // Copy the remaining elements of left subarray
    while (i <= mid) {
        arr[k++] = temp[i++];
        swaps++;
    }

    // Copy the remaining elements of right subarray
    while (j <= right) {
        arr[k++] = temp[j++];
        swaps++;
    }
}

// Iterative merge sort
void merge_sort_iterative(int arr[], int n) {
    // Temporary array for merging
    int* temp = (int*)malloc(n * sizeof(int));

    // Start with subarrays of size 1, and keep doubling
    for (int curr_size = 1; curr_size < n; curr_size = 2 * curr_size) {
        // Pick starting point of different subarrays of current size
        for (int left_start = 0; left_start < n - 1; left_start += 2 * curr_size) {
            // Find middle and end points
            int mid = left_start + curr_size - 1;
            int right_end = (left_start + 2 * curr_size - 1 < n - 1) ?
                            (left_start + 2 * curr_size - 1) : (n - 1);

            // Merge subarrays arr[left_start...mid] & arr[mid+1...right_end]
            if (mid < right_end) {
                merge(arr, temp, left_start, mid, right_end);
            }
        }
    }

    // Free temporary array
    free(temp);
}

/**
 * (c) Quick Sort Implementation
 * With different pivot selection strategies
 */

// Classic quick sort (using first element as pivot)
void quick_sort_classic(int arr[], int n) {
    quick_sort_classic_impl(arr, 0, n - 1);
}

void quick_sort_classic_impl(int arr[], int low, int high) {
    if (low < high) {
        // Partition the array
        int pivot_index = partition_classic(arr, low, high);

        // Sort the elements before and after partition
        quick_sort_classic_impl(arr, low, pivot_index - 1);
        quick_sort_classic_impl(arr, pivot_index + 1, high);
    }
}

int partition_classic(int arr[], int low, int high) {
    // First element as pivot
    int pivot = arr[low];
    int i = low + 1;

    for (int j = low + 1; j <= high; j++) {
        comparisons++;
        if (arr[j] < pivot) {
            // Swap arr[i] and arr[j]
            int temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
            swaps++;
            i++;
        }
    }

    // Swap pivot to its final position
    int temp = arr[low];
    arr[low] = arr[i - 1];
    arr[i - 1] = temp;
    swaps++;

    return i - 1;
}

// Quick sort with median-of-three pivot selection
void quick_sort_median(int arr[], int n) {
    quick_sort_median_impl(arr, 0, n - 1);
}

void quick_sort_median_impl(int arr[], int low, int high) {
    if (low < high) {
        // Partition the array
        int pivot_index = partition_median(arr, low, high);

        // Sort the elements before and after partition
        quick_sort_median_impl(arr, low, pivot_index - 1);
        quick_sort_median_impl(arr, pivot_index + 1, high);
    }
}

int partition_median(int arr[], int low, int high) {
    // Find median of three (first, middle, last) for pivot
    int mid = low + (high - low) / 2;

    // Sort the three elements
    if (arr[mid] < arr[low]) {
        int temp = arr[mid];
        arr[mid] = arr[low];
        arr[low] = temp;
        swaps++;
    }

    if (arr[high] < arr[low]) {
        int temp = arr[high];
        arr[high] = arr[low];
        arr[low] = temp;
        swaps++;
    }

    if (arr[high] < arr[mid]) {
        int temp = arr[high];
        arr[high] = arr[mid];
        arr[mid] = temp;
        swaps++;
    }

    // Median is now in the middle position
    // Swap it to the low position for partitioning
    int temp = arr[low];
    arr[low] = arr[mid];
    arr[mid] = temp;
    swaps++;

    // Now use the median as pivot for standard partitioning
    return partition_classic(arr, low, high);
}

// Shuffle array using Fisher-Yates algorithm
void shuffle_array(int arr[], int n) {
    for (int i = n - 1; i > 0; i--) {
        int j = rand() % (i + 1);
        int temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
    }
}
