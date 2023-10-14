use num_bigint::BigUint;

/// alpha^x mod p
/// output = n^exp mod p
pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint){
    n.modpow(exponent,modulus)
}
    
/// output = s = k-c*x mod q
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint{
    if *k >= c*x {
        return (k - c * x).modpow(exponent: &BigUint::from(1u32), modulus:q);
    }
    return q- (c * x-k).modpow(exponent: &BigUint::from(1u32), modulus:q);
}

///r1= alpha^s*y1^c
///r2=beta^s*y2^c
pub fn verify(r1:&BigUint,r2:&BigUint,y1:&BigUint,y2:&BigUint,alpha:&BigUint,beta:&BigUint,c:&BigUint,s:&BigUint,p:&BigUint)->bool{
    let cond1: bool =*r1==(alpha.modpow(exponent:s,modulus:p)*y1.modpow(exponent:c,p)).modpow(exponent:&BigUint::from(1u32),modulus:&p);
    let cond2: bool =*r2==(beta.modpow(exponent:s,modulus:p)*y2.modpow(exponent:c,p)).modpow(exponent:&BigUint::from(1u32),modulus:&p);
    cond1 && cond2
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_toy_example(){
        let alpha: BigUint = BigUint:: from(4u32);
        let beta: BigUint = BigUint:: from(9u32);
        let p: BigUint=BigUint::from(23u32);
        let q: BigUint=BigUint::from(11u32);

        let x: BigUint=BigUint::from(6u32);
        let k: BigUint=BigUint::from(7u32);

        let c: BigUint=BigUint::from(4u32);

        
        let y1: BigUint= exponentiate(n:&alpha, exponent:&x, modulus:&p);
        let y2: BigUint= exponentiate(n:&beta, exponent:&x, modulus:&p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1: BigUint= exponentiate(n:&alpha, exponent:&k, modulus:&p);
        let r2: BigUint= exponentiate(n:&beta, exponent:&k, modulus:&p);
        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        lets s: BigUint = solve(&k,&c,&x,&q);
        assert_eq!(s, BigUint::from(5u32));

        let result: bool = verify(&r1, &r2, &y1, &y2, &alpha, &beta, &c, &s. &p);
        assert!(result);
        
    }



}