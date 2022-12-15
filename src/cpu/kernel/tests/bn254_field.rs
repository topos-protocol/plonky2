use std::str::FromStr;

use anyhow::Result;
use ethereum_types::U256;
use rand::{thread_rng, Rng};

use crate::cpu::kernel::aggregator::KERNEL;
use crate::cpu::kernel::interpreter::{run_interpreter, BN_BASE};

type Fp2 = [U256; 2];
type Fp6 = [Fp2; 3];
type Fp12 = [Fp6; 2];

fn add_fp(x: U256, y: U256) -> U256 {
    (x + y) % BN_BASE
}

fn add3_fp(x: U256, y: U256, z: U256) -> U256 {
    (x + y + z) % BN_BASE
}

fn mul_fp(x: U256, y: U256) -> U256 {
    U256::try_from(x.full_mul(y) % BN_BASE).unwrap()
}

fn sub_fp(x: U256, y: U256) -> U256 {
    (BN_BASE + x - y) % BN_BASE
}

fn neg_fp(x: U256) -> U256 {
    (BN_BASE - x) % BN_BASE
}

fn conj_fp2(a: Fp2) -> Fp2 {
    let [a, a_] = a;
    [a, neg_fp(a_)]
}

fn add_fp2(a: Fp2, b: Fp2) -> Fp2 {
    let [a, a_] = a;
    let [b, b_] = b;
    [add_fp(a, b), add_fp(a_, b_)]
}

fn add3_fp2(a: Fp2, b: Fp2, c: Fp2) -> Fp2 {
    let [a, a_] = a;
    let [b, b_] = b;
    let [c, c_] = c;
    [add3_fp(a, b, c), add3_fp(a_, b_, c_)]
}

fn sub_fp2(a: Fp2, b: Fp2) -> Fp2 {
    let [a, a_] = a;
    let [b, b_] = b;
    [sub_fp(a, b), sub_fp(a_, b_)]
}

fn mul_fp2(a: Fp2, b: Fp2) -> Fp2 {
    let [a, a_] = a;
    let [b, b_] = b;
    [
        sub_fp(mul_fp(a, b), mul_fp(a_, b_)),
        add_fp(mul_fp(a, b_), mul_fp(a_, b)),
    ]
}

fn i9(a: Fp2) -> Fp2 {
    let [a, a_] = a;
    let nine = U256::from(9);
    [sub_fp(mul_fp(nine, a), a_), add_fp(a, mul_fp(nine, a_))]
}

fn add_fp6(c: Fp6, d: Fp6) -> Fp6 {
    let [c0, c1, c2] = c;
    let [d0, d1, d2] = d;

    let e0 = add_fp2(c0, d0);
    let e1 = add_fp2(c1, d1);
    let e2 = add_fp2(c2, d2);
    [e0, e1, e2]
}

fn sub_fp6(c: Fp6, d: Fp6) -> Fp6 {
    let [c0, c1, c2] = c;
    let [d0, d1, d2] = d;

    let e0 = sub_fp2(c0, d0);
    let e1 = sub_fp2(c1, d1);
    let e2 = sub_fp2(c2, d2);
    [e0, e1, e2]
}

fn mul_fp6(c: Fp6, d: Fp6) -> Fp6 {
    let [c0, c1, c2] = c;
    let [d0, d1, d2] = d;

    let c0d0 = mul_fp2(c0, d0);
    let c0d1 = mul_fp2(c0, d1);
    let c0d2 = mul_fp2(c0, d2);
    let c1d0 = mul_fp2(c1, d0);
    let c1d1 = mul_fp2(c1, d1);
    let c1d2 = mul_fp2(c1, d2);
    let c2d0 = mul_fp2(c2, d0);
    let c2d1 = mul_fp2(c2, d1);
    let c2d2 = mul_fp2(c2, d2);
    let cd12 = add_fp2(c1d2, c2d1);

    [
        add_fp2(c0d0, i9(cd12)),
        add3_fp2(c0d1, c1d0, i9(c2d2)),
        add3_fp2(c0d2, c1d1, c2d0),
    ]
}

fn sh(c: Fp6) -> Fp6 {
    let [c0, c1, c2] = c;
    [i9(c2), c0, c1]
}

fn sparse_embed(x: [U256; 5]) -> Fp12 {
    let [g0, g1, g1_, g2, g2_] = x;
    let zero = U256::from(0);
    [
        [[g0, zero], [g1, g1_], [zero, zero]],
        [[zero, zero], [g2, g2_], [zero, zero]],
    ]
}

fn mul_fp12(f: Fp12, g: Fp12) -> Fp12 {
    let [f0, f1] = f;
    let [g0, g1] = g;

    let h0 = mul_fp6(f0, g0);
    let h1 = mul_fp6(f1, g1);
    let h01 = mul_fp6(add_fp6(f0, f1), add_fp6(g0, g1));
    [add_fp6(h0, sh(h1)), sub_fp6(h01, add_fp6(h0, h1))]
}

fn gen_fp() -> U256 {
    let mut rng = thread_rng();
    let x64 = rng.gen::<u64>();
    U256([x64, x64, x64, x64]) % BN_BASE
}

fn gen_fp6() -> Fp6 {
    [
        [gen_fp(), gen_fp()],
        [gen_fp(), gen_fp()],
        [gen_fp(), gen_fp()],
    ]
}

fn gen_fp12_sparse() -> Fp12 {
    sparse_embed([gen_fp(), gen_fp(), gen_fp(), gen_fp(), gen_fp()])
}

fn frob_t1(n: usize) -> Fp2 {
    match n {
        0 => [
            U256::from_str("0x1").unwrap(),
            U256::from_str("0x0").unwrap(),
        ],
        1 => [
            U256::from_str("0x2fb347984f7911f74c0bec3cf559b143b78cc310c2c3330c99e39557176f553d")
                .unwrap(),
            U256::from_str("0x16c9e55061ebae204ba4cc8bd75a079432ae2a1d0b7c9dce1665d51c640fcba2")
                .unwrap(),
        ],
        2 => [
            U256::from_str("0x30644e72e131a0295e6dd9e7e0acccb0c28f069fbb966e3de4bd44e5607cfd48")
                .unwrap(),
            U256::from_str("0x0").unwrap(),
        ],
        3 => [
            U256::from_str("0x856e078b755ef0abaff1c77959f25ac805ffd3d5d6942d37b746ee87bdcfb6d")
                .unwrap(),
            U256::from_str("0x4f1de41b3d1766fa9f30e6dec26094f0fdf31bf98ff2631380cab2baaa586de")
                .unwrap(),
        ],
        4 => [
            U256::from_str("0x59e26bcea0d48bacd4f263f1acdb5c4f5763473177fffffe").unwrap(),
            U256::from_str("0x0").unwrap(),
        ],
        5 => [
            U256::from_str("0x28be74d4bb943f51699582b87809d9caf71614d4b0b71f3a62e913ee1dada9e4")
                .unwrap(),
            U256::from_str("0x14a88ae0cb747b99c2b86abcbe01477a54f40eb4c3f6068dedae0bcec9c7aac7")
                .unwrap(),
        ],
        _ => panic!(),
    }
}

fn frob_t2(n: usize) -> Fp2 {
    match n {
        0 => [
            U256::from_str("0x1").unwrap(),
            U256::from_str("0x0").unwrap(),
        ],
        1 => [
            U256::from_str("0x5b54f5e64eea80180f3c0b75a181e84d33365f7be94ec72848a1f55921ea762")
                .unwrap(),
            U256::from_str("0x2c145edbe7fd8aee9f3a80b03b0b1c923685d2ea1bdec763c13b4711cd2b8126")
                .unwrap(),
        ],
        2 => [
            U256::from_str("0x59e26bcea0d48bacd4f263f1acdb5c4f5763473177fffffe").unwrap(),
            U256::from_str("0x0").unwrap(),
        ],
        3 => [
            U256::from_str("0xbc58c6611c08dab19bee0f7b5b2444ee633094575b06bcb0e1a92bc3ccbf066")
                .unwrap(),
            U256::from_str("0x23d5e999e1910a12feb0f6ef0cd21d04a44a9e08737f96e55fe3ed9d730c239f")
                .unwrap(),
        ],
        4 => [
            U256::from_str("0x30644e72e131a0295e6dd9e7e0acccb0c28f069fbb966e3de4bd44e5607cfd48")
                .unwrap(),
            U256::from_str("0x0").unwrap(),
        ],
        5 => [
            U256::from_str("0x1ee972ae6a826a7d1d9da40771b6f589de1afb54342c724fa97bda050992657f")
                .unwrap(),
            U256::from_str("0x10de546ff8d4ab51d2b513cdbb25772454326430418536d15721e37e70c255c9")
                .unwrap(),
        ],
        _ => panic!(),
    }
}

fn frob_z(n: usize) -> Fp2 {
    match n {
        0 => [
            U256::from_str("0x1").unwrap(),
            U256::from_str("0x0").unwrap(),
        ],
        1 => [
            U256::from_str("0x1284b71c2865a7dfe8b99fdd76e68b605c521e08292f2176d60b35dadcc9e470")
                .unwrap(),
            U256::from_str("0x246996f3b4fae7e6a6327cfe12150b8e747992778eeec7e5ca5cf05f80f362ac")
                .unwrap(),
        ],
        2 => [
            U256::from_str("0x30644e72e131a0295e6dd9e7e0acccb0c28f069fbb966e3de4bd44e5607cfd49")
                .unwrap(),
            U256::from_str("0x0").unwrap(),
        ],
        3 => [
            U256::from_str("0x19dc81cfcc82e4bbefe9608cd0acaa90894cb38dbe55d24ae86f7d391ed4a67f")
                .unwrap(),
            U256::from_str("0xabf8b60be77d7306cbeee33576139d7f03a5e397d439ec7694aa2bf4c0c101")
                .unwrap(),
        ],
        4 => [
            U256::from_str("0x30644e72e131a0295e6dd9e7e0acccb0c28f069fbb966e3de4bd44e5607cfd48")
                .unwrap(),
            U256::from_str("0x0").unwrap(),
        ],
        5 => [
            U256::from_str("0x757cab3a41d3cdc072fc0af59c61f302cfa95859526b0d41264475e420ac20f")
                .unwrap(),
            U256::from_str("0xca6b035381e35b618e9b79ba4e2606ca20b7dfd71573c93e85845e34c4a5b9c")
                .unwrap(),
        ],
        6 => [
            U256::from_str("0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd46")
                .unwrap(),
            U256::from_str("0x0").unwrap(),
        ],
        7 => [
            U256::from_str("0x1ddf9756b8cbf849cf96a5d90a9accfd3b2f4c893f42a9166615563bfbb318d7")
                .unwrap(),
            U256::from_str("0xbfab77f2c36b843121dc8b86f6c4ccf2307d819d98302a771c39bb757899a9b")
                .unwrap(),
        ],
        8 => [
            U256::from_str("0x59e26bcea0d48bacd4f263f1acdb5c4f5763473177fffffe").unwrap(),
            U256::from_str("0x0").unwrap(),
        ],
        9 => [
            U256::from_str("0x1687cca314aebb6dc866e529b0d4adcd0e34b703aa1bf84253b10eddb9a856c8")
                .unwrap(),
            U256::from_str("0x2fb855bcd54a22b6b18456d34c0b44c0187dc4add09d90a0c58be1eae3bc3c46")
                .unwrap(),
        ],
        10 => [
            U256::from_str("0x59e26bcea0d48bacd4f263f1acdb5c4f5763473177ffffff").unwrap(),
            U256::from_str("0x0").unwrap(),
        ],
        11 => [
            U256::from_str("0x290c83bf3d14634db120850727bb392d6a86d50bd34b19b929bc44b896723b38")
                .unwrap(),
            U256::from_str("0x23bd9e3da9136a739f668e1adc9ef7f0f575ec93f71a8df953c846338c32a1ab")
                .unwrap(),
        ],
        _ => panic!(),
    }
}

fn frob_fp6(n: usize, c: Fp6) -> Fp6 {
    let [c0, c1, c2] = c;
    let _c0 = conj_fp2(c0);
    let _c1 = conj_fp2(c1);
    let _c2 = conj_fp2(c2);

    if n % 2 != 0 {
        [_c0, mul_fp2(frob_t1(n), _c1), mul_fp2(frob_t2(n), _c2)]
    } else {
        [c0, mul_fp2(frob_t1(n), c1), mul_fp2(frob_t2(n), c2)]
    }
}
fn frob_fp12(n: usize, f: Fp12) -> Fp12 {
    let [f0, f1] = f;
    let zero = U256::from(0);
    let scale = [frob_z(n), [zero, zero], [zero, zero]];
    [frob_fp6(n, f0), mul_fp6(scale, frob_fp6(n, f1))]
}

fn make_mul_stack(
    in0: usize,
    in1: usize,
    out: usize,
    f0: Fp6,
    f1: Fp6,
    g0: Fp6,
    g1: Fp6,
    mul_label: &str,
) -> Vec<U256> {
    // stack: in0, f, f', in1, g, g', mul_dest, in0, in1, out, ret_stack, out

    let in0 = U256::from(in0);
    let in1 = U256::from(in1);
    let out = U256::from(out);

    let f0: Vec<U256> = f0.into_iter().flatten().collect();
    let f1: Vec<U256> = f1.into_iter().flatten().collect();
    let g0: Vec<U256> = g0.into_iter().flatten().collect();
    let g1: Vec<U256> = g1.into_iter().flatten().collect();

    let ret_stack = U256::from(KERNEL.global_labels["ret_stack"]);
    let mul_dest = U256::from(KERNEL.global_labels[mul_label]);

    let mut input = vec![in0];
    input.extend(f0);
    input.extend(f1);
    input.extend(vec![in1]);
    input.extend(g0);
    input.extend(g1);
    input.extend(vec![mul_dest, in0, in1, out, ret_stack, out]);
    input.reverse();

    input
}

fn make_mul_expected(f: Fp12, g: Fp12) -> Vec<U256> {
    mul_fp12(f, g)
        .into_iter()
        .flatten()
        .flatten()
        .rev()
        .collect()
}

#[test]
fn test_mul_fp12() -> Result<()> {
    let in0 = 64;
    let in1 = 76;
    let out = 88;

    let f0 = gen_fp6();
    let f1 = gen_fp6();
    let g0 = gen_fp6();
    let g1 = gen_fp6();
    let [h0, h1] = gen_fp12_sparse();

    let test_mul = KERNEL.global_labels["test_mul_fp12"];

    let normal: Vec<U256> = make_mul_stack(in0, in1, out, f0, f1, g0, g1, "mul_fp12");
    let sparse: Vec<U256> = make_mul_stack(in0, in1, out, f0, f1, h0, h1, "mul_fp12_sparse");
    let square: Vec<U256> = make_mul_stack(in0, in1, out, f0, f1, f0, f1, "square_fp12_test");

    let out_normal: Vec<U256> = run_interpreter(test_mul, normal)?.stack().to_vec();
    let out_sparse: Vec<U256> = run_interpreter(test_mul, sparse)?.stack().to_vec();
    let out_square: Vec<U256> = run_interpreter(test_mul, square)?.stack().to_vec();

    let exp_normal: Vec<U256> = make_mul_expected([f0, f1], [g0, g1]);
    let exp_sparse: Vec<U256> = make_mul_expected([f0, f1], [h0, h1]);
    let exp_square: Vec<U256> = make_mul_expected([f0, f1], [f0, f1]);

    assert_eq!(out_normal, exp_normal);
    assert_eq!(out_sparse, exp_sparse);
    assert_eq!(out_square, exp_square);

    Ok(())
}
