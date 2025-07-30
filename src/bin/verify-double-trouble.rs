#![allow(unused, unreachable_code)]
use ark_ed_on_bls12_381::Fr;
use ark_ff::Field;
use double_trouble::data::puzzle_data;
use double_trouble::inner_product_argument::utils::challenge;
use double_trouble::verify;
use double_trouble::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (ck, [instance_and_proof_1, instance_and_proof_2]) = puzzle_data();
    let (instance1, proof1) = instance_and_proof_1;
    let (instance2, proof2) = instance_and_proof_2;
    assert!(verify(&ck, &instance1, &proof1));
    assert!(verify(&ck, &instance2, &proof2));

    let (a, comm_a_rand): (Vec<Fr>, Fr) = {
        
        // knowns
        let s1 = &proof1.response.s;
        let s2 = &proof2.response.s;
        let gamma1: Fr = challenge(&ck, &instance1, &proof1.commitment);
        let gamma2: Fr = challenge(&ck, &instance2, &proof2.commitment);

        // compute r₁ = (s₁ − s₂) / (γ₁ − 2·γ₂)
        let gamma_difference = gamma1 - gamma2.double();
        let gamma_difference_inv = gamma_difference.inverse().unwrap();
        let mut r1 = Vec::with_capacity(s1.len());
        for (si1, si2) in s1.iter().zip(s2.iter()) {
            r1.push((*si1 - *si2) * gamma_difference_inv);
        }

        // compute a = s₁ − γ₁·r₁
        let mut a = Vec::with_capacity(s1.len());
        for (si1, ri1) in s1.iter().zip(r1.iter()) {
            a.push(*si1 - gamma1 * *ri1);
        }

        // compute u values: α + γ·ρ
        let u1 = proof1.response.u;
        let u2 = proof2.response.u;

        // recover ρ₁ = (u₁ − u₂) / (γ₁ − 2·γ₂)
        let rho1 = (u1 - u2) * gamma_difference_inv;

        // recover α = u₁ − γ₁·ρ₁
        let alpha = u1 - gamma1 * rho1;

        (a, alpha)
    };
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance1.comm_a
    );
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance2.comm_a
    );
}