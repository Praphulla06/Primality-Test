### Miller–Rabin Primality Test

The **Miller–Rabin primality test** is a test used to check if a number is **composite**.

- If the test results in `true` (yes / positive), then the number is **probably prime**.  
- If the test results in `false` (no / negative), then the number is **definitely composite**.

This test is based on **Fermat's little theorem**, but improves on it by detecting many composites that Fermat's test fails to catch.

---

### Algorithm

1. **Factorization step**  
   Express \( n - 1 \) as:  
   $$
   n - 1 = 2^k \cdot m, \quad m \text{ is odd}
   $$

2. **Choose a random base \( a \)**  
   Select \( a \) such that:
   $$
   2 \le a \le n - 2
   $$

3. **Compute the initial value**  
   $$
   b_0 = a^m \bmod n
   $$

4. **First check**  
   - If:
     $$
     b_0 \equiv 1 \pmod{n} \quad \text{or} \quad b_0 \equiv n - 1 \pmod{n}
     $$
     then the number is **probably prime**.

### 5. Repeated Squaring

If 
$$
b_0 \neq 1 \quad \text{and} \quad b_0 \neq n - 1,
$$
then repeat the following for 
$$
i = 1, \dots, k-1:
$$

1. Compute 
$$
b_i = b_{i-1}^2 \bmod n
$$

2. Check each \( b_i \):
   - If 
   $$
   b_i \equiv n - 1 \pmod{n},
   $$
   then the number is **probably prime**.
   - If 
   $$
   b_i \equiv 1 \pmod{n} \quad \text{before reaching } n-1,
   $$
   then the number is **composite**.
