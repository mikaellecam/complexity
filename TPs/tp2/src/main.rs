use std::time::Instant;
use plotters::prelude::*;
use rand::Rng;

/// Tri par sélection
fn selection_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;
        for j in (i+1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

/// Tri par insertion
fn insertion_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    for i in 1..len {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

/// Tri à bulles
fn bubble_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

/// Tri fusion
fn merge_sort(arr: &mut Vec<i32>) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();

        merge_sort(&mut left);
        merge_sort(&mut right);

        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                arr[k] = left[i];
                i += 1;
            } else {
                arr[k] = right[j];
                j += 1;
            }
            k += 1;
        }
        while i < left.len() {
            arr[k] = left[i];
            i += 1;
            k += 1;
        }
        while j < right.len() {
            arr[k] = right[j];
            j += 1;
            k += 1;
        }
    }
}

/// Mesure le temps d'exécution d'un algorithme de tri
fn measure_time(sort_fn: fn(&mut Vec<i32>), arr: &mut Vec<i32>) -> u128 {
    let start = Instant::now();
    sort_fn(arr);
    start.elapsed().as_millis() // Valeur retournée (pas de besoin de point-virgule ni de mot-clé return)
}

/// Affiche un graphique comparant les performances des algorithmes de tri
fn plot_graph(results: &Vec<(usize, u128, u128, u128, u128)>) -> Result<(), Box<dyn std::error::Error>> {
let root = BitMapBackend::new("sorting_performance.png", (800, 600)).into_drawing_area();
root.fill(&WHITE)?;

let mut chart = ChartBuilder::on(&root)
.caption("Comparaison des Algorithmes de Tri", ("sans-serif", 20))
.margin(20)
.x_label_area_size(40)
.y_label_area_size(40)
.build_cartesian_2d(1000usize..80000usize, 0u128..10000u128)?;

chart.configure_mesh().draw()?;

let selection_times: Vec<(usize, u128)> = results.iter().map(|(s, sel, _, _, _)| (*s, *sel)).collect();
let insertion_times: Vec<(usize, u128)> = results.iter().map(|(s, _, ins, _, _)| (*s, *ins)).collect();
let bubble_times: Vec<(usize, u128)> = results.iter().map(|(s, _, _, bub, _)| (*s, *bub)).collect();
let merge_times: Vec<(usize, u128)> = results.iter().map(|(s, _, _, _, mer)| (*s, *mer)).collect();

chart.draw_series(LineSeries::new(selection_times, &RED))?.label("Tri par sélection").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));
chart.draw_series(LineSeries::new(insertion_times, &BLUE))?.label("Tri par insertion").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));
chart.draw_series(LineSeries::new(bubble_times, &GREEN))?.label("Tri à bulles").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &GREEN));
chart.draw_series(LineSeries::new(merge_times, &BLACK))?.label("Tri par fusion").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLACK));

chart.configure_series_labels().background_style(&WHITE).draw()?;

Ok(())
}

fn main() {
    let sizes_selection = vec![1000, 3160, 9500, 31600, 80000];
    let mut results = Vec::new();
    let mut rng = rand::rng();

    for &size in &sizes_selection {
        let arr: Vec<i32> = (0..size).map(|_| rng.random_range(0..10000)).collect();

        let sel_time = measure_time(selection_sort, &mut arr.clone());
        let ins_time = measure_time(insertion_sort, &mut arr.clone());
        let bub_time = measure_time(bubble_sort, &mut arr.clone());
        let mer_time = measure_time(merge_sort, &mut arr.clone());

        results.push((size, sel_time, ins_time, bub_time, 0));

        println!("Taille: {} | Selection Sort: {} ms | Insertion Sort: {} ms | Bubble Sort: {} ms | Merge Sort: {} ms",
                 size, sel_time, ins_time, bub_time, mer_time);
    }

    plot_graph(&results).unwrap();
}