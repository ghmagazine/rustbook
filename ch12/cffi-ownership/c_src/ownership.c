#include <stdlib.h>
#include <stdio.h>

void
take_ownership(int *i, void(*dtor)(int *))
{
  printf("got %d\n", *i);
  // Cのコードでメモリを解放する。
  // Rustで用意した値はRustから貰ったデストラクタで解放する
  dtor(i);
}

int *
make_memory() {
  int *i;

  i = malloc(sizeof(int));
  *i = 2;

  return i;
}

