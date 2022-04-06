console.log("Hello, world")

type int = number

function fib(n: int): int {
  if (n == 1 || n == 2) {
    return 1
  }

  return fib(n - 1) + fib(n - 2)
}

let a = fib(3)
