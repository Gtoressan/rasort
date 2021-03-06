# Rasort
This repository is designed to implement various sorting algorithms in the Rust programming language for educational purposes.

## Array Sorting Algorithms
Below are the algorithms that have been implemented or will be implemented in this project. The list may change.

| Algorithm     | Best Time  | Average Time | Worst Time | Worst Space Complexity | Is Implemented |
| ------------- | ---------- | ------------ | ---------- | ---------------------- | -------------- |
| Quick Sort    | `n log(n)` | `n log(n)`   | `n^2     ` | `log(n)`               | Not yet        |
| Merge Sort    | `n log(n)` | `n log(n)`   | `n log(n)` | `n     `               | ✔ |
| Bubble Sort   | `n       ` | `n^2     `   | `n^2     ` | `1     `               | ✔ |
| Counting Sort | `n + k   ` | `n + k   `   | `n + k   ` | `k     `               | ✔ |

## Console Interface
Now you can use console to interact with program. Below specified the list of console features which have been implemented or will be implemented. The list may change.

| Name            | Long name | Short name | Takes value | Kind       | Is Implemented |
| --------------- | --------- | ---------- | ----------- | ---------- | -------------- |
| file            | file      | -          | String      | Required   | ✔ |
| algorithm-name  | algo      | -          | String      | Required   | ✔ |
| unsorted-vector | unsv      | u          | Number      | Optional   | ✔ |
| sorted-vector   | srtv      | s          | Number      | Optional   | ✔ |
| comparisons     | cmop      | c          | -           | Optional   | ✔ |
| memory-acsesses | memac     | m          | -           | Optional   | ✔ |
| time            | time      | t          | -           | Optional   | ✔ |
| vector-len      | vlen      | l          | -           | Optional   | ✔ |
