# 0: closure (roots), () -> never
# captured = 1, locals = 0, 4 instructions 

# constants (0)

# captures (1)
  U:0: [PARENT SCOPE]

# locals (1)
  R:0: (isize) a


##########################################

4: Closure(R:0, P:1)
5: LoadInt(R:1, usize:3)
6: Call1R1(R:0, R:0, R:1)
7: Return0()

# end closure (roots)


# 1: closure fib, (n: int) -> int
# captured = 1, locals = 1,  instructions 

# constants (2)
  K:0: (int) 1
  K:1: (int) 2
  K:2: "fib"

# captures (1)
  U:0: [PARENT SCOPE]

# locals (1)
  R:0: (isize) n

##########################################

 0: EqualK(R:0, K:0, true)
 1: Jump(PC:+2)

 2: EqualK(R:0, K:1, false)
 3: Jump(PC:+1)

 4: Return1K(K:0)

 5: GetCapturedParnet(R:1, K:2) # Closure Self

 6: LoadConst(R:2, K:0)
 7: Sub(R:2, R:0, R:2)
 9: Call1R1(R:2, R:1, R:2)

10: LoadConst(R:3, K:1)
11: Sub(R:3, R:0, R:3)
12: Call1R1(R:3, R:1, R:3)

13: Add(R:1, R:2, R:3)
14: Return1(R:1)
