import matplotlib.pyplot as plt
import numpy as np

disks = [1, 2, 5, 10, 15, 25]
times = [0.00000241, 0.0000007, 0.00000017, 0.00002365, 0.00071741, 0.074409715]

plt.figure(figsize=(10, 6))

plt.scatter(disks, times, color='blue', s=60, label='Measured times')

plt.plot(disks, times, 'b-', alpha=0.7)

x_theory = np.linspace(1, 25, 100)
scale_factor = times[-1] / (2**disks[-1])
y_theory = [scale_factor * (2**n) for n in x_theory]

plt.plot(x_theory, y_theory, 'r--', label='Theoretical O(2^n)')

plt.yscale('log')

plt.xlabel('Number of Disks')
plt.ylabel('Time (seconds)')
plt.title('Hanoi Tower Algorithm Time Complexity')
plt.grid(True, which="both", ls="-", alpha=0.2)
plt.legend()

for i, (d, t) in enumerate(zip(disks, times)):
    plt.annotate(f"{t:.8f}s",
                 xy=(d, t),
                 xytext=(5, 10),
                 textcoords='offset points',
                 fontsize=8)

plt.tight_layout()
plt.show()

plt.figure(figsize=(10, 6))
plt.plot(disks, times, 'bo-', label='Measured times')
plt.xlabel('Number of Disks')
plt.ylabel('Time (seconds)')
plt.title('Hanoi Tower Algorithm Time Complexity (Linear Scale)')
plt.grid(True)
plt.legend()
plt.tight_layout()
plt.show()