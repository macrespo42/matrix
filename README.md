# Welcome to the matrix 😎💊

## An introduction to Linear Algebra


All the unit test for the project can be run with `cargo test`  
And you can run a little CLI programm to show test output with `cargo run`


### Summary

<details>
<summary> Introduction </summary>

- [Exercise 00 - Add, Subtract and Scale](#ex00)
- [Exercise 01 - Linear combination](#ex01)
- [Exercise 02 - Linear interpolation](#ex02)
- [Exercise 03 - Dot product](#ex03)
- [Exercise 04 - Norm](#ex04)
- [Exercise 05 - Cosine](#ex05)
- [Exercise 06 - Cross product](#ex06)
- [Exercise 07 - Linear map, Matrix multiplication](#ex07)
- [Exercise 08 - Trace](#ex08)
- [Exercise 09 - Transpose](#ex09)
- [Exercise 10 - Row echelon form](#ex10)
- [Exercise 11 - Determinant](#ex11)

</details>

###  <a name="ex00">Exercise 00 - Add, Subtract and Scale</a>

Maximum time complexity: **O(n)**  
Maximum space complexity: **O(n)**

You must write functions that can add and subtract two vectors, or two matrices, of the
same size; and a function to multiply a vector, or a matrix, by a scalar (ie, "scaling").
You must also turn in a main function in order to test your functions, ready to be
compiled (if necessary) and run.

### <a name="ex01">Exercise 01 - Linear combination</a>

Maximum time complexity: **O(n)**  
Maximum space complexity: **O(n)**

You must write a function that computes a linear combination of the vectors provided,
using the corresponding scalar coefficients.
You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.


### <a name="ex02">Exercise 02 - Linear interpolation</a>

Maximum time complexity : **O(n)**
Maximum space complexity : **O(n)**

You must write a function that computes a linear interpolation between two objects of the same type.
You must also turn in a main function in order to test your function, ready to be compiled (if necessary) and run.

### <a name="ex03">Exercise 03 - Dot product</a>

Maximum time complexity : **O(n)**  
Maximum space complexity : **O(n)**  

You must write a function that computes the dot product of two vectors of the same dimension.  
You must also turn in a main function in order to test your function, ready to be compiled (if necessary) and run.  


### <a name="ex04">Exercise 04 - Norm</a>

Maximum time complexity : **O(n)**  
Maximum space complexity : **O(n)**  

You must write functions that compute different kinds of norms.  
You must also turn in a main function in order to test your functions, ready to be  
compiled (if necessary) and run.  

### <a name="ex05"> Exercise 05 - Cosine</a>

Maximum time complexity : **O(n)**  
Maximum space complexity : **O(n)**  

You must write functions that compute the cosine of the angle between two given vectors.  
You must also turn in a main function in order to test your function, ready to be  
compiled (if necessary) and run.  

**Reminder:** The usage of the standard library’s cos function is  
forbidden, of course.  

### <a name="ex06"> Exercise 06 - Cross product</a>

Maximum time complexity : ***N/A***
Maximum space complexity : ***N/A***  

You must write a function that computes the cross product of two 3-dimensional vectors.  
You must also turn in a main function in order to test your function, ready to be  
compiled (if necessary) and run.  

### <a name="ex07"> Exercise 07 - Linear map, Matrix multiplication</a>

Maximum time complexity : ***see below***  
Maximum space complexity : ***see below***  

You must write functions that multiply a matrix by a vector or a matrix by a matrix.  
You must also turn in a main function in order to test your functions, ready to be compiled (if necessary) and run.

Let *A* ∈ R m×n , B ∈ R n×p and u ∈ R n where (m, n, p) ∈ N3 (represented as variables of type u32).  
You must implement functions that compute:  
- *Au* (which returns a vector in R
m) (max time complexity O(nm), max space complexity O(nm))
- *AB* (which returns a matrix in Rm×p) (max time complexity O(nmp), max space complexity O(nm + mp + np))

### <a name="ex08"> Exercise 08 - Trace</a>

Maximum time complexity : ***O(n)***  
Maximum space complexity : ***N/A***  

You must write a function that computes the trace of the given matrix.  
You must also turn in a main function in order to test your function, ready to be compiled (if necessary) and run.  

Let A ∈ Kn×n be a square matrix, where K is the real numbers (represented as the type
f32) and (m, n) ∈ N
2
(represented as two variables of type u32).
The function must compute and return T r(A)

### <a name="ex09"> Exercise 09 - Transpose</a>

Maximum time complexity : ***O(nm)***  
Maximum space complexity : ***O(nm)***  

You must write a function that computes the transpose matrix of a given matrix.  
You must also turn in a main function in order to test your function, ready to be  
compiled (if necessary) and run.  

Let A ∈ Km×n where K is the real numbers (represented as the type f32) and (m, n) ∈ N2 (represented as values of the type u32).  
The function must return the transpose matrix B ∈ Kn×m  

### <a name="ex10"> Exercise 10 - Row-echelon form</a>

Maximum time complexity : ***O(n3)***  
Maximum space complexity : ***O(n2)***  

You must write a function that computes the row-echelon form of the given
matrix.  
You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.  

Let A ∈ Km×n, where K is the real numbers (represented as the type f32) and (m, n) ∈ N2 (represented as two values of type u32).
The function must return the row-echelon form of the matrix.

### <a name="ex11"> Exercise 11 - Determinant</a>

Maximum time complexity : ***O(n3)***  
Maximum space complexity : ***O(n2)***  

You must write a function that computes the determinant of the given matrix.  
You must also turn in a main function in order to test your function, ready to be compiled (if necessary) and run.  

Let A ∈ Kn×n where K is the real numbers (represented as the type f32) and n ∈ N and n ≤ 4 (represented as a value of the type u32).  
The function must return the determinant det(A) of the matrix.  
If the matrix is not square, the behaviour is undefined.  
Since algorithms to compute the determinant fast and accurately for higher dimensions  
tend to be pretty complex, we have limited the required cases for which you must compute  
the determinant to dimensions 4 and below.  
