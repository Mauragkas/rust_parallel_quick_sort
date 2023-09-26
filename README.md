## 🚀 Parallel QuickSort in Rust

This project is a simple demonstration of how the QuickSort algorithm can be parallelized in Rust using the Rayon library. It is designed to be an efficient solution for sorting large datasets.

### 📊 Why Parallel QuickSort?
- **Efficiency:** Uses multiple cores for simultaneous computations, reducing the overall time complexity and increasing speed.
- **Scalability:** Ideal for large datasets, allowing faster processing and sorting.
- **Usability:** Capable of sorting both primitive and complex user-defined types.

### 🏗️ Implementation Highlights
- Utilizes Rust’s **Rayon library**, abstracting the complexities of parallel computing.
- Implements **QuickSort**, a divide-and-conquer sorting algorithm, ideal for parallelization.
- **Custom and Primitive Types:** Sorts primitive data types and user-defined structs, like `Person`, provided they implement `PartialOrd` and `Send` traits.

```rust
#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}
```

### 💡 Conclusion
Parallel QuickSort in Rust, with the help of the Rayon library, offers an efficient and fast approach to sort large datasets by leveraging the power of multiple cores and parallel computing.
