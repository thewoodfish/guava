use crate::models::{
    metrics::LendingPolicy,
    proof::{LoanDecision, ProofPackage},
};
use crate::services::{proof_gen, soroban};
use anyhow::Result;
use uuid::Uuid;

pub struct SorobanConfig<'a> {
    pub contract_id: &'a str,
    pub identity: &'a str,
    pub network: &'a str,
}

/// Verify a proof package, evaluate the loan decision, and record it on Stellar.
pub fn evaluate(
    application_id: Uuid,
    package: &ProofPackage,
    policy: &LendingPolicy,
    circuits_dir: &str,
    soroban: &SorobanConfig,
) -> Result<LoanDecision> {
    // 1. Cryptographic proof verification (off-chain, bb verify)
    let proof_verified = match proof_gen::verify(package, circuits_dir) {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!("Proof verification error: {}", e);
            false
        }
    };

    if !proof_verified {
        return Ok(LoanDecision {
            application_id,
            decision: "rejected".to_string(),
            reason: "Cryptographic proof verification failed.".to_string(),
            proof_verified: false,
            policy_met: false,
            failed_predicates: vec!["proof_invalid".to_string()],
            stellar_tx_hash: None,
        });
    }

    // 2. Policy check against proven predicates
    let failed: Vec<String> = package
        .predicates
        .iter()
        .filter(|p| !p.satisfied)
        .map(|p| p.name.clone())
        .collect();

    let policy_met = failed.is_empty();

    let (decision, reason) = if policy_met {
        (
            "approved".to_string(),
            "All lending criteria verified by ZK proofs.".to_string(),
        )
    } else {
        (
            "rejected".to_string(),
            format!("The following criteria were not met: {}", failed.join(", ")),
        )
    };

    // 3. Record decision on Stellar testnet (non-blocking on failure)
    let stellar_tx_hash = match soroban::record_decision_on_chain(
        package.proof_id,
        &package.proof_hex,
        policy,
        &decision,
        soroban.contract_id,
        soroban.identity,
        soroban.network,
    ) {
        Ok(hash) => {
            tracing::info!("Decision recorded on Stellar: {}", hash);
            Some(hash)
        }
        Err(e) => {
            tracing::error!("Failed to record on Stellar (non-fatal): {}", e);
            None
        }
    };

    Ok(LoanDecision {
        application_id,
        decision,
        reason,
        proof_verified,
        policy_met,
        failed_predicates: failed,
        stellar_tx_hash,
    })
}
