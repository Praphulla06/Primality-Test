### Miller-Rabin primality test

Miller-Rabin primality test is a a test used to check if a number is composite.

If the test results in true (yes / positive), then number is probably prime. Id the result is false (no / negative) then the number is for sure composite.

This test is built on top of femat's little primality test.

### Algorithm

1. Find n - 1 = 2^k . m
2. Choose 'a' such that 2 <= a < n - 1.
3. Compute b0 = a^m (mod n),.., bi = (bi-1)^2 (mod n). If b0 = +1 or n - 1, the number is probably prime.

If neither +1 nor n - 1 then compute next step 
b1 = b0 ^ 2 (mod n) repeat for k - 1 times.