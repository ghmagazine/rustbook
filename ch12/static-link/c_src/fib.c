unsigned long long
fib(unsigned int n)
{
  unsigned long long n0 = 1, n1 =1;

  if (n == 0) {
    return n0;
  } else if (n == 1) {
    return n1;
  } else {
    for(unsigned int i = 1; i < n; ++i) {
      n0 ^= n1; n1 ^= n0; n0 ^= n1;
      n1 += n0;
    }
    return n1;
  }
}

