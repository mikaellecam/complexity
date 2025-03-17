### Groupe: 
- MZOUGHI Khalil
- LE CAM Mikael

# Analyse théorique en complexité des algorithmes de tri

## (a) Tri par insertion : Comparaison des versions itérative et récursive

### Version itérative

```c
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
```

**Analyse de complexité :**
- **Cas favorable** (tableau déjà trié) : $O(n)$
    - Dans le cas favorable, la boucle `while` intérieure ne s'exécute jamais car tous les éléments sont déjà à leur place.
    - Nous avons donc seulement la boucle externe qui s'exécute $n-1$ fois, d'où une complexité $O(n)$.
    - Concrètement, chaque élément est déjà plus grand que tous les éléments précédents, donc aucun déplacement n'est nécessaire.

- **Cas défavorable** (tableau trié en ordre inverse) : $O(n^2)$
    - Pour chaque élément à la position $i$, nous devons le comparer et le déplacer à travers tous les éléments précédents (de $0$ à $i-1$).
    - Cela donne: $\sum_{i=1}^{n-1} i = \frac{n(n-1)}{2} = O(n^2)$
    - Le premier élément est comparé 0 fois, le deuxième 1 fois, le troisième 2 fois, etc.

- **Cas moyen** : $O(n^2)$
    - En supposant une distribution uniforme, chaque élément devra être comparé en moyenne avec la moitié des éléments précédents.
    - Cela donne: $\sum_{i=1}^{n-1} \frac{i}{2} = \frac{n(n-1)}{4} = O(n^2)$

### Version récursive

```c
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
```

**Analyse de complexité :**
- **Cas favorable** (tableau déjà trié) : $O(n)$
    - La récursion se fait sur $n$ appels, chacun effectuant une opération en temps constant pour l'insertion du dernier élément.
    - La fonction récursive peut être modélisée par: $T(n) = T(n-1) + O(1) = O(n)$
    - Chaque insertion prend un temps constant car aucun déplacement n'est nécessaire.

- **Cas défavorable** (tableau trié en ordre inverse) : $O(n^2)$
    - Pour un tableau de taille $n$, l'appel récursif a une complexité $T(n-1)$ et l'insertion du dernier élément prend $O(n)$ opérations.
    - La relation de récurrence est: $T(n) = T(n-1) + O(n)$
    - En développant, on obtient : $T(n) = T(n-2) + O(n-1) + O(n) = ... = O(1) + O(2) + ... + O(n) = O(n^2)$

- **Cas moyen** : $O(n^2)$
    - Similaire au cas défavorable mais avec en moyenne la moitié des comparaisons à chaque niveau.
    - Relation de récurrence : $T(n) = T(n-1) + O(n/2) = O(n^2)$

La version récursive a cependant un coût supplémentaire dû à l'empilement des appels récursifs sur la pile d'exécution, ce qui nécessite $O(n)$ espace mémoire supplémentaire.

⟹ La version **itérative** est donc plus efficace car elle évite le surcoût lié aux appels récursifs et utilise moins de mémoire, tout en ayant la même complexité temporelle.

## (b) Tri fusion : Comparaison des versions récursive et itérative

### Version récursive

```c
void merge_sort_recursive(int arr[], const int n) {
    // Liste temporaire pour la fusion
    int* temp = malloc(n * sizeof(int));

    merge_sort_recursive_impl(arr, temp, 0, n - 1);

    free(temp);
}

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

void merge(int arr[], int temp[], const int left, const int mid, const int right) {
    // On copie les données dans le tableau temporaire
    for (int i = left; i <= right; i++) {
        temp[i] = arr[i];
    }

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
```

**Analyse de complexité :**
- **Cas favorable** : $O(n \log n)$
    - L'algorithme utilise une approche "diviser pour régner" qui divise systématiquement le tableau en deux moitiés.
    - Hauteur de l'arbre de récursion : $\log_2 n$ (car on divise par 2 à chaque étape)
    - À chaque niveau de l'arbre, on effectue $O(n)$ opérations pour la fusion.
    - Ce qui nous donne une complexité totale de : $O(n) \times O(\log n) = O(n \log n)$

- **Cas défavorable** : $O(n \log n)$
    - Même calcul que le cas favorable, car l'algorithme divise toujours le tableau en deux parties égales.
    - La division est toujours équilibrée : $T(n) = 2T(n/2) + O(n)$
    - Ce qui nous donne une complexité totale de : $O(n \log n)$

- **Cas moyen** : $O(n \log n)$
    - Identique aux cas favorable et défavorable, car la division et la fusion suivent toujours le même schéma.

### Version itérative

```c
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
```

**Analyse de complexité :**
- **Cas favorable** : $O(n \log n)$
    - La boucle externe s'exécute $\log_2 n$ fois (tailles de fusion: 1, 2, 4, 8, ..., n/2)
    - À chaque itération, la boucle interne parcourt l'ensemble du tableau en effectuant des fusions de sous-tableaux.
    - Chaque fusion prend un temps linéaire $O(n)$ au total pour toutes les fusions à un niveau donné.
    - Ce qui nous donne une complexité totale de : $O(\log n) \times O(n) = O(n \log n)$

- **Cas défavorable** : $O(n \log n)$
    - Même raisonnement que pour le cas favorable.
    - La complexité ne dépend pas de l'ordre initial des éléments.

- **Cas moyen** : $O(n \log n)$
    - Identique aux autres cas car l'algorithme suit toujours le même schéma de fusion indépendamment de l'ordre des données.

La version itérative a l'avantage de ne pas utiliser la pile d'appels récursifs, ce qui économise de l'espace mémoire ($O(1)$ au lieu de $O(\log n)$ pour la pile d'appels récursifs).

⟹ La version **itérative** est donc plus efficace en termes d'utilisation de la mémoire. Les deux versions ont la même complexité temporelle, mais la version itérative évite le risque de dépassement de pile pour de très grands tableaux.

## (c) Tri rapide : Algorithme classique et amélioration du choix du pivot

### Algorithme classique (pivot = premier élément)

```c
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
```

**Analyse de complexité :**
- **Cas favorable** (pivot divise le tableau en deux parties égales) : $O(n \log n)$
    - Lorsque le pivot divise parfaitement le tableau en deux parties égales, nous obtenons une relation de récurrence:
    - $T(n) = 2T(n/2) + O(n)$, où $O(n)$ est le coût de la partition.
    - La hauteur de l'arbre de récursion est $\log_2 n$ et à chaque niveau nous effectuons $O(n)$ opérations.
    - Donc $T(n) = O(n \log n)$

- **Cas défavorable** (tableau déjà trié ou trié en ordre inverse) : $O(n^2)$
    - Lorsque le pivot est toujours l'élément le plus petit ou le plus grand, une partition est vide et l'autre contient $n-1$ éléments.
    - La relation de récurrence devient : $T(n) = T(n-1) + O(n)$
    - En développant, on obtient : $T(n) = T(n-2) + O(n-1) + O(n) = ... = O(n) + O(n-1) + ... + O(1) = O(n^2)$
    - L'arbre de récursion devient une chaîne de profondeur $n$, avec un coût de $O(n)$ à chaque niveau.

- **Cas moyen** : $O(n \log n)$
    - En supposant que le pivot divise le tableau en proportions constantes (pas nécessairement égales), par exemple 1/4 et 3/4.
    - Dans ce cas, la profondeur de l'arbre reste $O(\log n)$ et le coût total est $O(n \log n)$
    - Une analyse probabiliste montre que la complexité moyenne est $O(n \log n)$ sur l'ensemble des permutations possibles.

### Méthode améliorée : Pivot = médiane de trois (premier, milieu, dernier)

```c
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
```

**Analyse de complexité :**
- **Cas favorable** : $O(n \log n)$
    - Similaire à l'algorithme classique avec pivot optimal.
    - Le choix de la médiane de trois augmente la probabilité d'obtenir un pivot qui divise le tableau de manière équilibrée.
    - La relation de récurrence reste : $T(n) = 2T(n/2) + O(n) = O(n \log n)$

- **Cas défavorable** : Théoriquement $O(n^2)$, mais en pratique proche de $O(n \log n)$
    - Pour qu'un cas défavorable se produise, il faudrait que la médiane des trois éléments choisis soit systématiquement un élément extrême, ce qui est très improbable.
    - Pour un tableau déjà trié, la médiane des trois sera généralement proche du milieu, évitant la dégénérescence.
    - La probabilité d'obtenir un pivot très déséquilibré est considérablement réduite, mais pas éliminée.
    - Dans le pire cas, on pourrait toujours avoir : $T(n) = T(n-1) + O(n) = O(n^2)$

- **Cas moyen** : $O(n \log n)$
    - Le choix de la médiane de trois réduit la variance de la position du pivot, ce qui se traduit par une complexité moyenne de $O(n \log n)$ avec une constante inférieure à celle de l'algorithme classique.

### Comparaison des approches :

La méthode de la médiane de trois semble être la plus efficace car :
- Elle est peu coûteuse à calculer (seulement quelques comparaisons et échanges)
- Elle évite efficacement les cas pathologiques comme les tableaux déjà triés
- Elle maintient la complexité en cas moyen à O(n log n) avec une constante réduite par rapport à l'algorithme classique
- Elle résiste mieux aux données déjà triées ou inversées, qui posent problème pour le tri rapide classique

⟹ La méthode de la **médiane de trois** offre le meilleur compromis entre efficacité et fiabilité pour le choix du pivot, et évite la dégradation vers une complexité quadratique dans de nombreux cas pratiques.