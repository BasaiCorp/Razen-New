// src/backend/execution/raze/optimization/peephole.rs
//! Peephole optimizations
//! 
//! Local optimizations on small instruction windows.

use crate::backend::execution::raze::mir::MIR;

pub fn optimize(mir: Vec<MIR>) -> Vec<MIR> {
    let mut result = Vec::new();
    let mut i = 0;
    
    while i < mir.len() {
        // Pattern: mov r1, r2; mov r3, r1 => mov r3, r2
        if i + 1 < mir.len() {
            if let (MIR::Move { dest: d1, src: s1, .. }, MIR::Move { dest: d2, src: s2, .. }) 
                = (&mir[i], &mir[i + 1]) {
                if d1 == s2 {
                    result.push(MIR::Move { dest: *d2, src: *s1, ty: crate::backend::execution::raze::mir::MIRType::Int64 });
                    i += 2;
                    continue;
                }
            }
        }
        
        result.push(mir[i].clone());
        i += 1;
    }
    
    result
}
