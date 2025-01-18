# Hands-on 2

## Exercise 1
You are given an array A[1,n] of positive integers, each integer is at most n.
You have to build a data structure to answer two different types of queries:

- Update(i, j, T) that replaces every value A[k] with min(T, A[k]) for each k in [i; j]
- Max(i, j) that returns the largest value in A[i..j]

You are also given m of these queries to solve. The target solution must run
in O((n+m)log(n)) time.

## Exercise 2
You are given n segments. A segment <l, r> is such that 0 ≤ l ≤ r ≤ n-1.
Then, you are given m queries IsThere.

A query IsThere(i,j,k) has to return 1 if there exists a position p,
with 0 ≤ i ≤ p ≤ j ≤ n-1, such that exactly k segments contain position p,
0 otherwise.

The solution must run in O((n+m)log(n)) time.