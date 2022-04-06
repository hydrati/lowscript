# constants (2)
  0: (int) 1
  1: (int) 2
  2: "fib"

# captures (1)
  0: [PARENT SCOPE]

# locals (1)
  0: (int) n

# registers
0: Closure('fib')
3: isize(3)
4: (fib(isize(3))) isize(2)


################

# register - fib(isize(3))
0: isize(3)
# R:0 != K:0, pc = pc + 1
# R:0 != K:1, pc = pc + 0
1: Closure('fib') # self
2: (K:0) isize(1)
3: (R:0 - R:2) isize(2)
4: (fib(R:3=isize(2))) isize(1)

2: (K:1) isize(2)
3: (R:0 - R:2) isize(1)
5: (fib(R:3=isize(1))) isize(1)

6: (R:4+R:5) isize(2)
# return1 R:6


###############
# register - fib(isize(2))
0: isize(2)
# R:0 != K:0, pc = pc + 1
# R:0 == K:1, pc = pc + 1
# Return1 (constant) K:0 = 1

###############
# register - fib(isize(1))
0: isize(1)
# R:0 == K:0, pc = pc + 1, jmp PC:+2
# Return1 (constant) K:0 = 1