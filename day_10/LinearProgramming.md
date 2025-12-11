# 📘 Linear Programming (ILP) Reference – Joltage Button Solver

This document explains:

1. What Linear Programming (LP) and Integer Linear Programming (ILP) are  
2. **How to spot** when a problem is an LP/ILP  
3. How our **joltage + buttons** puzzle maps to ILP  
4. The matrix formulation  
5. A fully worked example with **3 counters, 3 buttons**  
6. Visual intuition / diagrams

You can keep this in `README.md` or as a big doc-comment near the ILP code.

---

## 1. What is LP / ILP?

A **Linear Program (LP)** is an optimization problem where:

- You choose real-valued variables \(x_1, x_2, \dots, x_n\)
- Subject to linear constraints
- And optimize a linear objective

General LP form:

- **Variables:**

  \[
  x_1, x_2, \dots, x_n \in \mathbb{R}
  \]

- **Objective (minimize / maximize):**

  \[
  \min / \max \; c_1 x_1 + c_2 x_2 + \dots + c_n x_n
  \]

- **Constraints (linear):**

  \[
  a_{1} x_1 + a_{2} x_2 + \dots + a_{n} x_n \le / = / \ge b
  \]

If you additionally require **integer variables**:

- \(x_i \in \mathbb{Z}\) (usually \(\mathbb{Z}_{\ge 0}\))

then it becomes an **Integer Linear Program (ILP)**.

Our joltage problem is an **ILP** because we can only press buttons an integer number of times.

---

## 2. How to Spot an LP / ILP Problem

A problem is a good fit for LP/ILP if:

1. **Decision variables** represent “how much” you do something  
   - e.g. how many times to press each button, how many units to ship, how many items to pick, etc.

2. The effect on the system is **linear** in those variables  
   - Pressing a button once adds `+1` to some counters.  
   - Pressing it `x` times adds `x` to those counters.  
   - No squares, products of variables, or other nonlinear operations.

3. Constraints are **linear sums** you want to match or bound  
   - “Final joltage on counter i must be exactly `t[i]`”  
   - “Total cost ≤ budget”, “total weight ≤ capacity”, etc.

4. The cost / score you want to optimize is a **linear function** of the variables  
   - Minimize `x₀ + x₁ + ...`  
   - Maximize profit like `5x₀ + 3x₁`.

5. If variables must be **integers or 0/1**, that’s ILP  
   - You can’t press a button 0.7 times → integer variables.  
   - You either choose an item or you don’t → binary 0/1 variables.

For this puzzle:

- Variables = how many times to press each button (**integers**, ≥ 0)  
- Effects = linear (each press adds +1 to some counters)  
- Constraints = final counter values must equal target joltages  
- Objective = minimize total presses (sum of variables, linear)

✅ This is a textbook ILP.

---

## 3. Mapping the Joltage Problem to ILP

For **one machine**:

- You have `n` counters (joltage requirements)
- You have `m` buttons

Each button `j` affects some subset of counters `idxs_j`.  
Pressing button `j` **once** adds `+1` to each counter in `idxs_j`.

### Variables

Define one integer variable per button:

\[
x_j = \text{number of times we press button } j, \quad j = 0, 1, \dots, m-1
\]

We require:

\[
x_j \ge 0, \quad x_j \in \mathbb{Z}
\]

### Constraints

Define a matrix \(A\) where:

\[
A[i][j] =
\begin{cases}
1 & \text{if button } j \text{ affects counter } i \\
0 & \text{otherwise}
\end{cases}
\]

Let `t[i]` be the target joltage for counter `i`.  
Then the constraint for each counter `i` is:

\[
\sum_{j=0}^{m-1} A[i][j] \, x_j = t[i] \quad \text{for all } i = 0, 1, \dots, n-1
\]

### Objective

We want the **fewest total button presses**:

\[
\min \sum_{j=0}^{m-1} x_j
\]

Putting it together:

- Variables: \(x_j \in \mathbb{Z}_{\ge 0}\)
- Constraints: \(A x = t\)
- Objective: \(\min 1^\top x\) (sum of button presses)

---

## 4. Matrix Form + ASCII Diagram

Think of each button as a **column** in a matrix, and each counter as a **row**.

Example with 3 counters and 3 buttons:

- Button A affects counters 0 and 1 → `[1, 1, 0]`
- Button B affects counters 1 and 2 → `[0, 1, 1]`
- Button C affects counter 0 → `[1, 0, 0]`

Matrix \(A\):

```text
       Button A    Button B    Button C
       --------    --------    --------
C0  |     1           0           1
C1  |     1           1           0
C2  |     0           1           0
```

As a 2D array:

```text
A = [
  [1, 0, 1],   // counter 0
  [1, 1, 0],   // counter 1
  [0, 1, 0],   // counter 2
]
```

Let:

```text
x = [xA, xB, xC]  // how many times we press each button
t = [4, 3, 2]     // target joltage for counters 0, 1, 2
```
Then constraints are:
```text
A * x = t
```

Written out:
```text
counter 0: 1*xA + 0*xB + 1*xC = 4
counter 1: 1*xA + 1*xB + 0*xC = 3
counter 2: 0*xA + 1*xB + 0*xC = 2
```

Objective:
```text
minimize xA + xB + xC
```


## 5. Worked Example: 3 Counters, 3 Buttons

Target:
```text
t = [4, 3, 2]
```

Buttons:

```text
A → [1, 1, 0]

B → [0, 1, 1]

C → [1, 0, 0]
```

Variables:
```text
xA = presses of A
xB = presses of B
xC = presses of C
xA, xB, xC ∈ ℤ, x ≥ 0
```

### 5.1 Constraints

From the matrix:
```text
counter 0: xA + xC       = 4
counter 1: xA + xB       = 3
counter 2:      xB       = 2
```
### 5.2 Solve by hand

From counter 2:

```text
xB = 2
```

Substitute into counter 1:
```text
xA + xB = 3
xA + 2  = 3  →  xA = 1
```

Substitute into counter 0:
```text
xA + xC = 4
1  + xC = 4  →  xC = 3
```

So the integer solution is:
```text
xA = 1
xB = 2
xC = 3
```
### 5.3 Objective value

Total presses:
```text
xA + xB + xC = 1 + 2 + 3 = 6
```

This is exactly what an ILP solver will find:
it searches over all integer triples (xA, xB, xC) satisfying the constraints, and picks the one with minimal xA + xB + xC.
