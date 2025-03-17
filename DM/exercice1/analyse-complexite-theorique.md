# Analyse théorique en complexité des algorithmes de tri

## (a) Tri par insertion : Comparaison des versions itérative et récursive

### Version itérative

```c
void insertion_sort_iterative(int arr[], int n) {
    int i, key, j;
    for (i = 1; i < n; i++) {
        key = arr[i];
        j = i - 1;
        
        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j = j - 1;
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
void insertion_sort_recursive(int arr[], int n) {
    // Cas de base
    if (n <= 1)
        return;
    
    // Trier d'abord les premiers n-1 éléments
    insertion_sort_recursive(arr, n - 1);
    
    // Insérer le dernier élément à sa place dans le tableau trié
    int last = arr[n - 1];
    int j = n - 2;
    
    while (j >= 0 && arr[j] > last) {
        arr[j + 1] = arr[j];
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
void merge(int arr[], int l, int m, int r) {
    int i, j, k;
    int n1 = m - l + 1;
    int n2 = r - m;
    
    // Créer des tableaux temporaires
    int L[n1], R[n2];
    
    // Copier les données dans les tableaux temporaires
    for (i = 0; i < n1; i++)
        L[i] = arr[l + i];
    for (j = 0; j < n2; j++)
        R[j] = arr[m + 1 + j];
    
    // Fusionner les tableaux temporaires
    i = 0;
    j = 0;
    k = l;
    while (i < n1 && j < n2) {
        if (L[i] <= R[j]) {
            arr[k] = L[i];
            i++;
        } else {
            arr[k] = R[j];
            j++;
        }
        k++;
    }
    
    // Copier les éléments restants de L[] s'il y en a
    while (i < n1) {
        arr[k] = L[i];
        i++;
        k++;
    }
    
    // Copier les éléments restants de R[] s'il y en a
    while (j < n2) {
        arr[k] = R[j];
        j++;
        k++;
    }
}

void merge_sort_recursive(int arr[], int l, int r) {
    if (l < r) {
        int m = l + (r - l) / 2;
        
        merge_sort_recursive(arr, l, m);
        merge_sort_recursive(arr, m + 1, r);
        
        merge(arr, l, m, r);
    }
}
```

**Analyse de complexité :**
- **Cas favorable** : $O(n \log n)$
  - L'algorithme utilise une approche "diviser pour régner" qui divise systématiquement le tableau en deux moitiés.
  - Hauteur de l'arbre de récursion : $\log_2 n$ (car on divise par 2 à chaque étape)
  - À chaque niveau de l'arbre, on effectue $O(n)$ opérations pour la fusion.
  - Ce qui nous sonne une complexité totale de : $O(n) \times O(\log n) = O(n \log n)$

- **Cas défavorable** : $O(n \log n)$
  - Même calcul que le cas favorable, car l'algorithme divise toujours le tableau en deux parties égales.
  - La division est toujours équilibrée : $T(n) = 2T(n/2) + O(n)$
  - Ce qui nous sonne une complexité totale de : $O(n \log n)$

- **Cas moyen** : $O(n \log n)$
  - Identique aux cas favorable et défavorable, car la division et la fusion suivent toujours le même schéma.

### Version itérative

```c
void merge_sort_iterative(int arr[], int n) {
    int curr_size;
    int left_start;
    
    // Fusionner des sous-tableaux de taille 1, 2, 4, 8...
    for (curr_size = 1; curr_size <= n - 1; curr_size = 2 * curr_size) {
        for (left_start = 0; left_start < n - 1; left_start += 2 * curr_size) {
            int mid = min(left_start + curr_size - 1, n - 1);
            int right_end = min(left_start + 2 * curr_size - 1, n - 1);
            
            merge(arr, left_start, mid, right_end);
        }
    }
}
```

**Analyse de complexité :**
- **Cas favorable** : $O(n \log n)$
  - La boucle externe s'exécute $\log_2 n$ fois (tailles de fusion: 1, 2, 4, 8, ..., n/2)
  - À chaque itération, la boucle interne parcourt l'ensemble du tableau en effectuant des fusions de sous-tableaux.
  - Chaque fusion prend un temps linéaire $O(n)$ au total pour toutes les fusions à un niveau donné.
  - Ce qui nous sonne une complexité totale de : $O(\log n) \times O(n) = O(n \log n)$

- **Cas défavorable** : $O(n \log n)$
  - Même raisonnement que pour le cas favorable.
  - La complexité ne dépend pas de l'ordre initial des éléments.

- **Cas moyen** : $O(n \log n)$
  - Identique aux autres cas car l'algorithme suit toujours le même schéma de fusion indépendamment de l'ordre des données.

La version itérative a l'avantage de ne pas utiliser la pile d'appels récursifs, ce qui économise de l'espace mémoire ($O(1)$ au lieu de $O(\log n)$ pour la pile d'appels récursifs).

⟹ La version **itérative** est donc plus efficace en termes d'utilisation de la mémoire. Les deux versions ont la même complexité temporelle, mais la version itérative évite le risque de dépassement de pile pour de très grands tableaux.

## (c) Tri rapide : Algorithme classique et amélioration du choix du pivot

### Algorithme classique (pivot = dernier élément)

```c
int partition(int arr[], int low, int high) {
    int pivot = arr[high];
    int i = (low - 1);
    
    for (int j = low; j <= high - 1; j++) {
        if (arr[j] < pivot) {
            i++;
            swap(&arr[i], &arr[j]);
        }
    }
    swap(&arr[i + 1], &arr[high]);
    return (i + 1);
}

void quick_sort(int arr[], int low, int high) {
    if (low < high) {
        int pi = partition(arr, low, high);
        
        quick_sort(arr, low, pi - 1);
        quick_sort(arr, pi + 1, high);
    }
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

### Méthode 1 : Pivot = médiane de trois (premier, milieu, dernier)

```c
int medianOfThree(int arr[], int low, int high) {
    int mid = low + (high - low) / 2;
    
    // Tri des trois éléments
    if (arr[low] > arr[mid])
        swap(&arr[low], &arr[mid]);
    if (arr[low] > arr[high])
        swap(&arr[low], &arr[high]);
    if (arr[mid] > arr[high])
        swap(&arr[mid], &arr[high]);
    
    // Placer la médiane à la position high-1 (avant-dernière)
    swap(&arr[mid], &arr[high-1]);
    return arr[high-1];
}

int partitionMedianOfThree(int arr[], int low, int high) {
    if (high - low > 2) {
        int pivot = medianOfThree(arr, low, high);
        int i = low;
        int j = high - 1;
        
        while (1) {
            while (arr[++i] < pivot);
            while (arr[--j] > pivot);
            
            if (i >= j)
                break;
            swap(&arr[i], &arr[j]);
        }
        swap(&arr[i], &arr[high-1]);
        return i;
    } else {
        // Pour les petits tableaux, utiliser l'insertion directe
        if (arr[low] > arr[high])
            swap(&arr[low], &arr[high]);
        return low;
    }
}

void quick_sort_median(int arr[], int low, int high) {
    if (low < high) {
        int pi = partitionMedianOfThree(arr, low, high);
        
        quick_sort_median(arr, low, pi - 1);
        quick_sort_median(arr, pi + 1, high);
    }
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

### Méthode 2 : Pivot = moyenne des valeurs

```c
int averagePivot(int arr[], int low, int high) {
    long sum = 0;
    for (int i = low; i <= high; i++) {
        sum += arr[i];
    }
    int average = (int)(sum / (high - low + 1));
    
    // Trouver l'élément le plus proche de la moyenne
    int closest = arr[low];
    int min_diff = abs(arr[low] - average);
    
    for (int i = low + 1; i <= high; i++) {
        int diff = abs(arr[i] - average);
        if (diff < min_diff) {
            min_diff = diff;
            closest = arr[i];
        }
    }
    
    // Trouver la position de l'élément le plus proche
    for (int i = low; i <= high; i++) {
        if (arr[i] == closest) {
            swap(&arr[i], &arr[high]);
            break;
        }
    }
    
    return partition(arr, low, high);
}

void quick_sort_average(int arr[], int low, int high) {
    if (low < high) {
        int pi = averagePivot(arr, low, high);
        
        quick_sort_average(arr, low, pi - 1);
        quick_sort_average(arr, pi + 1, high);
    }
}
```

**Analyse de complexité :**
- **Cas favorable** : $O(n \log n)$
  - Similaire aux autres versions de quicksort.
  - Cependant, il faut ajouter le coût du calcul de la moyenne, qui est $O(n)$ à chaque niveau de récursion.
  - Relation de récurrence : $T(n) = 2T(n/2) + O(n) + O(n) = 2T(n/2) + O(n) = O(n \log n)$

- **Cas défavorable** : Théoriquement $O(n^2)$, mais en pratique souvent proche de $O(n \log n)$
  - Le calcul de la moyenne tend à produire un pivot proche de la médiane, réduisant la probabilité d'un partitionnement déséquilibré.

- **Cas moyen** : $O(n \log n)$
  - Le choix du pivot basé sur la moyenne améliore généralement l'équilibre du partitionnement.
  - Toutefois, le coût supplémentaire du calcul de la moyenne ($O(n)$ à chaque niveau) contrebalance partiellement ce gain.

Cependant, le calcul de la moyenne a un coût supplémentaire de $O(n)$ à chaque niveau de récursion, ce qui peut dégrader les performances pour les petits tableaux.

### Comparaison des approches :

La méthode de la médiane de trois semble être la plus efficace car :
- Elle est moins coûteuse à calculer que la moyenne des valeurs (O(1) vs O(n))
- Elle évite efficacement les cas pathologiques comme les tableaux déjà triés 
- Elle maintient la complexité en cas moyen à O(n log n) avec une constante réduite par rapport à l'algorithme classique

⟹ La méthode de la **médiane de trois** offre le meilleur compromis entre efficacité et fiabilité pour le choix du pivot.