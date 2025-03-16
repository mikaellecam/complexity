#ifndef TRI_COMPOSITE_H
#define TRI_COMPOSITE_H

#define TEST_SIZES 5
#define NUM_ALGORITHMS 5

extern unsigned long comparisons;
extern unsigned long swaps;

void insertion_sort_iterative(int arr[], int n);
void insertion_sort_recursive(int arr[], int n);
void merge_sort_recursive(int arr[], int n);
void merge_sort_recursive_impl(int arr[], int temp[], int left, int right);
void merge_sort_iterative(int arr[], int n);
void merge(int arr[], int temp[], int left, int mid, int right);
void quick_sort_classic(int arr[], int n);
void quick_sort_classic_impl(int arr[], int low, int high);
int partition_classic(int arr[], int low, int high);
void quick_sort_median(int arr[], int n);
void quick_sort_median_impl(int arr[], int low, int high);
int partition_median(int arr[], int low, int high);

void reset_counters();
void print_array(int arr[], int size);
void copy_array(int src[], int dest[], int size);
int* create_array(int size, char* type);
void measure_time(int array[], int size, void (*sort_function)(int[], int), char* sort_name);
void shuffle_array(int arr[], int n);

#endif
