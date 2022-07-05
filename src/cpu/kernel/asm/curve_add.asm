// #define N 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47 // BN254 base field order

global ec_add:
    PUSH 2
    PUSH 1
    PUSH 2
    PUSH 1
    JUMPDEST
    // stack: x0, y0, x1, y1, retdest
    DUP4
    // stack: y1, x0, y0, x1, y1, retdest
    DUP4
    // stack: x1, y1, x0, y0, x1, y1, retdest
    DUP4
    // stack: y0, x1, y1, x0, y0, x1, y1, retdest
    DUP4
    // stack: x0, y0, x1, y1, x0, y0, x1, y1, retdest
    %ec_check
    // stack: isValid(x0, y0), x1, y1, x0, y0, x1, y1, retdest
    PUSH ec_add_valid_first_point
    // stack: ec_add_valid_first_point, isValid(x0, y0), x1, y1, x0, y0, x1, y1, retdest
    JUMPI
    // stack: x1, y1, x0, y0, x1, y1, retdest
    POP
    // stack: y1, x0, y0, x1, y1, retdest
    POP
    // stack: x0, y0, x1, y1, retdest
    POP
    // stack: y0, x1, y1, retdest
    POP
    // stack: x1, y1, retdest
    POP
    // stack: y1, retdest
    POP
    // stack: retdest
    JUMP


ec_add_valid_first_point:
    JUMPDEST
    // stack: x1, y1, x0, y0, x1, y1, retdest
    %ec_check
    // stack: isValid(x1, y1), x0, y0, x1, y1, retdest
    PUSH ec_add_valid_points
    // stack: ec_add_valid_points, isValid(x1, y1), x0, y0, x1, y1, retdest
    JUMPI
    // stack: x0, y0, x1, y1, retdest
    POP
    // stack: y0, x1, y1, retdest
    POP
    // stack: x1, y1, retdest
    POP
    // stack: y1, retdest
    POP
    // stack: retdest
    JUMP

ec_add_valid_points:
    JUMPDEST
    // stack: x0, y0, x1, y1, retdest
    DUP3
    // stack: x1, x0, y0, x1, y1, retdest
    DUP2
    // stack: x0, x1, x0, y0, x1, y1, retdest
    EQ
    // stack: x0 == x1, x0, y0, x1, y1, retdest
    PUSH ec_add_equal_first_coord
    // stack: ec_add_equal_first_coord, x0 == x1, x0, y0, x1, y1, retdest
    JUMPI
    // stack: x0, y0, x1, y1, retdest
    PUSH ec_add_valid_points_contd
    // stack: ec_add_valid_points_contd, x0, y0, x1, y1, retdest
    DUP5
    // stack: y1, ec_add_valid_points_contd, x0, y0, x1, y1, retdest
    DUP4
    // stack: y0, y1, ec_add_valid_points_contd, x0, y0, x1, y1, retdest
    PUSH submod
    // stack: submod, y0, y1, ec_add_valid_points_contd, x0, y0, x1, y1, retdest
    JUMP

ec_add_valid_points_contd:
    JUMPDEST
    // stack: (y0 - y1) % N, x0, y0, x1, y1, retdest
    PUSH ec_add_valid_points_contd2
    // stack: ec_add_valid_points_contd2, (y0 - y1) % N, x0, y0, x1, y1, retdest
    DUP5
    // stack: x1, ec_add_valid_points_contd2, (y0 - y1) % N, x0, y0, x1, y1, retdest
    DUP4
    // stack: x0, x1, ec_add_valid_points_contd2, (y0 - y1) % N, x0, y0, x1, y1, retdest
    PUSH submod
    // stack: submod, x0, x1, ec_add_valid_points_contd2, (y0 - y1) % N, x0, y0, x1, y1, retdest
    JUMP

ec_add_valid_points_contd2:
    JUMPDEST
    // stack: (x0 - x1) % N, (y0 - y1) % N, x0, y0, x1, y1, retdest
    //MODDIV // TODO: Implement this
    // stack: lambda, x0, y0, x1, y1, retdest
    PUSH ec_add_valid_points_with_lambda
    // stack: ec_add_valid_points_with_lambda, lambda, x0, y0, x1, y1, retdest
    JUMP

ec_add_valid_points_with_lambda:
    JUMPDEST
    // stack: lambda, x0, y0, x1, y1, retdest
    PUSH ec_add_valid_points_contd4
    // stack: ec_add_valid_points_contd4, lambda, x0, y0, x1, y1, retdest
    DUP3
    // stack: x0, ec_add_valid_points_contd4, lambda, x0, y0, x1, y1, retdest
    PUSH ec_add_valid_points_contd3
    // stack: ec_add_valid_points_contd3, x0, ec_add_valid_points_contd4, lambda, x0, y0, x1, y1, retdest
    DUP7
    // stack: x1, ec_add_valid_points_contd3, x0, ec_add_valid_points_contd4, lambda, x0, y0, x1, y1, retdest
    PUSH 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47
    // stack: N, x1, ec_add_valid_points_contd3, x0, ec_add_valid_points_contd4, lambda, x0, y0, x1, y1, retdest
    DUP6
    // stack: lambda, N, x1, ec_add_valid_points_contd3, x0, ec_add_valid_points_contd4, lambda, x0, y0, x1, y1, retdest
    DUP1
    // stack: lambda, lambda, N, x1, ec_add_valid_points_contd3, x0, ec_add_valid_points_contd4, lambda, x0, y0, x1, y1, retdest
    MULMOD
    // stack: lambda^2, x1, ec_add_valid_points_contd3, x0, ec_add_valid_points_contd4, lambda, x0, y0, x1, y1, retdest
    PUSH submod
    // stack: submod, lambda^2, x1, ec_add_valid_points_contd3, x0, ec_add_valid_points_contd4, lambda, x0, y0, x1, y1, retdest
    JUMP

ec_add_valid_points_contd3:
    JUMPDEST
    // stack: lambda^2 - x1, x0, ec_add_valid_points_contd4, lambda, x0, y0, x1, y1, retdest
    PUSH submod
    // stack: submod, lambda^2 - x1, x0, ec_add_valid_points_contd4, lambda, x0, y0, x1, y1, retdest
    JUMP

ec_add_valid_points_contd4:
    JUMPDEST
    // stack: x2, lambda, x0, y0, x1, y1, retdest
    PUSH ec_add_valid_points_contd6
    // stack: ec_add_valid_points_contd6, x2, lambda, x0, y0, x1, y1, retdest
    PUSH 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47
    // stack: N, ec_add_valid_points_contd6, x2, lambda, x0, y0, x1, y1, retdest
    PUSH ec_add_valid_points_contd5
    // stack: ec_add_valid_points_contd5, N, ec_add_valid_points_contd6, x2, lambda, x0, y0, x1, y1, retdest
    DUP4
    // stack: x2, ec_add_valid_points_contd5, N, ec_add_valid_points_contd6, x2, lambda, x0, y0, x1, y1, retdest
    SWAP8
    // stack: x1, x2, ec_add_valid_points_contd5, N, ec_add_valid_points_contd6, x2, lambda, x0, y0, y1, retdest
    PUSH submod
    // stack: submod, x1, x2, ec_add_valid_points_contd5, N, ec_add_valid_points_contd6, x2, lambda, x0, y0, y1, retdest
    JUMP

ec_add_valid_points_contd5:
    JUMPDEST
    // stack: x1 - x2, N, ec_add_valid_points_contd6, x2, lambda, x0, y0, y1, retdest
    DUP5
    // stack: lambda, x1 - x2, N, ec_add_valid_points_contd6, x2, lambda, x0, y0, y1, retdest
    MULMOD
    // stack: lambda * (x1 - x2), ec_add_valid_points_contd6, x2, lambda, x0, y0, y1, retdest
    DUP7
    // stack: y1, lambda * (x1 - x2), ec_add_valid_points_contd6, x2, lambda, x0, y0, y1, retdest
    SWAP1
    // stack: lambda * (x1 - x2), y1, ec_add_valid_points_contd6, x2, lambda, x0, y0, y1, retdest
    PUSH submod
    // stack: submod, lambda * (x1 - x2), y1, ec_add_valid_points_contd6, x2, lambda, x0, y0, y1, retdest
    JUMP

ec_add_valid_points_contd6:
    JUMPDEST
    // stack: y2, x2, x0, y0, y1, retdest
    SWAP4
    // stack: y1, x2, x0, y0, y2, retdest
    POP
    // stack: x2, x0, y0, y2, retdest
    SWAP2
    // stack: y0, x0, x2, y2, retdest
    POP
    // stack: x0, x2, y2, retdest
    POP
    // stack: x2, y2, retdest
    SWAP1
    // stack: y2, x2, retdest
    SWAP2
    // stack: retdest, x2, y2
    JUMP

ec_add_equal_first_coord:
    JUMPDEST
    // stack: x0, y0, x1, y1, retdest with x0 == x1
    PUSH 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47
    // stack: N, x0, y0, x1, y1, retdest
    DUP3
    // stack: y0, N, x0, y0, x1, y1, retdest
    DUP6
    // stack: y1, y0, N, x0, y0, x1, y1, retdest
    ADDMOD
    // stack: y1 + y0, x0, y0, x1, y1, retdest
    PUSH ec_add_equal_points
    // stack: ec_add_equal_points, y1 + y0, x0, y0, x1, y1, retdest
    JUMPI
    // stack: x0, y0, x1, y1, retdest
    POP
    // stack: y0, x1, y1, retdest
    POP
    // stack: x1, y1, retdest
    POP
    // stack: y1, retdest
    POP
    // stack: retdest
    PUSH 0
    // stack: 0, retdest
    PUSH 0
    // stack: 0, 0, retdest
    SWAP2
    // stack: retdest, 0, 0
    JUMP


// Assumption: x0 == x1 and y0 == y1
ec_add_equal_points:
    JUMPDEST
    // stack: x0, y0, x1, y1, retdest
    PUSH 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47
    // stack: N, x0, y0, x1, y1, retdest
    PUSH 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47
    // stack: N, N, x0, y0, x1, y1, retdest
    DUP3
    // stack: x0, N, N, x0, y0, x1, y1, retdest
    DUP1
    // stack: x0, x0, N, N, x0, y0, x1, y1, retdest
    MULMOD
    // stack: x0^2, N, x0, y0, x1, y1, retdest with
    PUSH 0x183227397098d014dc2822db40c0ac2ecbc0b548b438e5469e10460b6c3e7ea5 // 3/2 in the base field
    // stack: 3/2, x0^2, N, x0, y0, x1, y1, retdest
    MULMOD
    // stack: 3/2 * x0^2, x0, y0, x1, y1, retdest
    DUP3
    // stack: y0, 3/2 * x0^2, x0, y0, x1, y1, retdest
    //MODDIV // TODO: Implement this
    // stack: lambda, x0, y0, x1, y1, retdest
    PUSH ec_add_valid_points_with_lambda
    // stack: ec_add_valid_points_with_lambda, lambda, x0, y0, x1, y1, retdest
    JUMP

global ec_double:
    JUMPDEST
    // stack: x0, y0, retdest
    DUP2
    // stack: y0, x0, y0, retdest
    DUP2
    // stack: x0, y0, x0, y0, retdest
    PUSH ec_add_equal_points
    // stack: ec_add_equal_points, x0, y0, x0, y0, retdest
    JUMP

submod:
    JUMPDEST
    // stack: x, y, retdest
    SWAP1
    // stack: y, x, retdest
    DUP1
    // stack: y, y, x, retdest
    DUP3
    // stack: x, y, y, x, retdest
    LT
    // stack: x < y, y, x, retdest
    PUSH submod
    // stack: submod, x < y, y, x, retdest
    JUMPI
    // stack: y, x, retdest
    PUSH 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47
    // stack: N, y, x, retdest
    SWAP2
    // stack: x, y, N, retdest
    SUB
    // stack: x - y, N, retdest,
    MOD
    // stack: (x - y) % N, retdest
    SWAP1
    // stack: retdest, (x - y) % N
    JUMP

%macro ec_check
    // stack: x0, y0
    PUSH 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47
    // stack: N, x0, y0
    PUSH 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47
    // stack: N, N, x0, y0
    SWAP2
    // stack: x0, N, N, y0
    PUSH 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47
    // stack: N, x0, N, N, y0
    DUP2
    // stack: x0, N, x0, N, N, y0
    DUP1
    // stack: x0, x0, N, x0, N, N, y0
    MULMOD
    // stack: x0^2 % N, x0, N, N, y0
    MULMOD
    // stack: x0^3 % N, N, y0
    PUSH 3
    // stack: 3, x0^3 % N, N, y0
    ADDMOD
    // stack: (x0^3 + 3) % N, y0
    SWAP1
    // stack: y0, (x0^3 + 3) % N
    PUSH 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47
    // stack: N, y0, (x0^3 + 3) % N
    SWAP1
    // stack: y0, N, (x0^3 + 3) % N
    DUP1
    // stack: y0, y0, N, (x0^3 + 3) % N
    MULMOD
    // stack: y0^2 % N, (x0^3 + 3) % N
    EQ
    // stack: y0^2 % N == (x0^3 + 3) % N
%endmacro

