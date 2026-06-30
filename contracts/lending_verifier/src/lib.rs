//! LedgerProof Lending Verifier — Soroban Smart Contract
//!
//! Records cryptographically-verified loan decisions on Stellar.
//!
//! The UltraHonk proof is verified off-chain by the Barretenberg `bb verify`
//! tool (computationally infeasible within Soroban's instruction budget).
//! This contract receives the verified result and records it immutably:
//!   - Proof hash (first 32 bytes of the UltraHonk proof)
//!   - Proven public inputs (the lender's committed thresholds)
//!   - Loan decision (APPROVED / REJECTED)
//!   - Lender address
//!   - Timestamp (ledger)
//!
//! Anyone can look up a proof_id to confirm the on-chain record matches
//! the proof package they received off-chain.

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    symbol_short, Address, Bytes, BytesN, Env, Symbol,
    log, panic_with_error,
};

// ── Error codes ────────────────────────────────────────────────────────────

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    PolicyNotSatisfied = 1,
    InvalidInput = 2,
    AlreadyRecorded = 3,
}

// ── Storage types ──────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug)]
pub struct LendingPolicy {
    pub required_monthly_revenue: u64,
    pub required_avg_balance: u64,
    pub required_positive_cf_months: u64,
    pub max_revenue_volatility_bps: u64,
    pub max_customer_concentration_bps: u64,
    pub max_debt_ratio_bps: u64,
    pub require_no_missed_repayments: u64,
    pub required_account_age_months: u64,
}

/// Public inputs committed into the ZK proof (the lender's thresholds).
#[contracttype]
#[derive(Clone, Debug)]
pub struct PublicInputs {
    pub required_monthly_revenue: u64,
    pub required_avg_balance: u64,
    pub required_positive_cf_months: u64,
    pub max_revenue_volatility_bps: u64,
    pub max_customer_concentration_bps: u64,
    pub max_debt_ratio_bps: u64,
    pub require_no_missed_repayments: u64,
    pub required_account_age_months: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct LoanRecord {
    /// First 32 bytes of the UltraHonk proof (fingerprint)
    pub proof_hash: BytesN<32>,
    pub lender: Address,
    pub decision: Symbol,
    pub public_inputs: PublicInputs,
    pub verified_at: u64,
}

// ── Contract ───────────────────────────────────────────────────────────────

#[contract]
pub struct LendingVerifier;

#[contractimpl]
impl LendingVerifier {
    /// Record a verified loan decision on-chain.
    ///
    /// Called by the LedgerProof backend after `bb verify` confirms the
    /// UltraHonk proof is valid. The lender must sign this transaction.
    ///
    /// proof_id        — UUID v4 bytes (16 bytes) used as the storage key
    /// proof_hash      — first 32 bytes of the proof hex (fingerprint)
    /// public_inputs   — 8 × u64 ABI-encoded (big-endian, 8 bytes each = 64 bytes)
    /// policy          — the lender's on-chain policy (verified against public inputs)
    /// decision        — "APPROVED" or "REJECTED"
    pub fn record_decision(
        env: Env,
        lender: Address,
        proof_id: BytesN<16>,
        proof_hash: BytesN<32>,
        public_inputs_bytes: Bytes,
        policy: LendingPolicy,
        decision: Symbol,
    ) -> Symbol {
        lender.require_auth();

        // Reject duplicate recordings for the same proof
        if env.storage().persistent().has(&proof_id) {
            panic_with_error!(&env, Error::AlreadyRecorded);
        }

        // Decode and verify that the proven thresholds satisfy the policy
        let inputs = Self::decode_public_inputs(&env, &public_inputs_bytes);
        Self::assert_policy_satisfied(&env, &inputs, &policy);

        let record = LoanRecord {
            proof_hash,
            lender: lender.clone(),
            decision: decision.clone(),
            public_inputs: inputs,
            verified_at: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&proof_id, &record);

        log!(&env, "LedgerProof: {} decision recorded for proof {:?}", decision, proof_id);

        decision
    }

    /// Retrieve a recorded loan decision by proof_id.
    pub fn get_record(env: Env, proof_id: BytesN<16>) -> Option<LoanRecord> {
        env.storage().persistent().get(&proof_id)
    }

    // ── Internal helpers ───────────────────────────────────────────────────

    fn decode_public_inputs(env: &Env, bytes: &Bytes) -> PublicInputs {
        if bytes.len() < 64 {
            panic_with_error!(env, Error::InvalidInput);
        }

        let read_u64 = |offset: u32| -> u64 {
            let mut buf = [0u8; 8];
            for i in 0..8u32 {
                buf[i as usize] = bytes.get(offset + i).unwrap_or(0);
            }
            u64::from_be_bytes(buf)
        };

        PublicInputs {
            required_monthly_revenue:          read_u64(0),
            required_avg_balance:              read_u64(8),
            required_positive_cf_months:       read_u64(16),
            max_revenue_volatility_bps:        read_u64(24),
            max_customer_concentration_bps:    read_u64(32),
            max_debt_ratio_bps:                read_u64(40),
            require_no_missed_repayments:      read_u64(48),
            required_account_age_months:       read_u64(56),
        }
    }

    fn assert_policy_satisfied(env: &Env, inputs: &PublicInputs, policy: &LendingPolicy) {
        // The proven thresholds in the proof must be at least as strict as the lender's policy.
        // This prevents reusing a proof generated under a laxer policy.
        let ok = inputs.required_monthly_revenue >= policy.required_monthly_revenue
            && inputs.required_avg_balance >= policy.required_avg_balance
            && inputs.required_positive_cf_months >= policy.required_positive_cf_months
            && inputs.max_revenue_volatility_bps <= policy.max_revenue_volatility_bps
            && inputs.max_customer_concentration_bps <= policy.max_customer_concentration_bps
            && inputs.max_debt_ratio_bps <= policy.max_debt_ratio_bps
            && (policy.require_no_missed_repayments == 0
                || inputs.require_no_missed_repayments == 1)
            && inputs.required_account_age_months >= policy.required_account_age_months;

        if !ok {
            panic_with_error!(env, Error::PolicyNotSatisfied);
        }
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn make_public_inputs_bytes(env: &Env, inputs: &[u64; 8]) -> Bytes {
        let mut buf = [0u8; 64];
        let mut i = 0;
        for val in inputs {
            for b in val.to_be_bytes() {
                buf[i] = b;
                i += 1;
            }
        }
        Bytes::from_slice(env, &buf)
    }

    #[test]
    fn test_record_and_retrieve() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(LendingVerifier, ());
        let client = LendingVerifierClient::new(&env, &contract_id);

        let lender = Address::generate(&env);
        let proof_id = BytesN::from_array(&env, &[1u8; 16]);
        let proof_hash = BytesN::from_array(&env, &[2u8; 32]);

        let inputs_raw: [u64; 8] = [
            600_000_000, 60_000_000, 5, 1200, 2000, 2000, 1, 24,
        ];
        let public_inputs_bytes = make_public_inputs_bytes(&env, &inputs_raw);

        let policy = LendingPolicy {
            required_monthly_revenue: 500_000_000,
            required_avg_balance: 50_000_000,
            required_positive_cf_months: 4,
            max_revenue_volatility_bps: 1500,
            max_customer_concentration_bps: 2500,
            max_debt_ratio_bps: 2500,
            require_no_missed_repayments: 1,
            required_account_age_months: 12,
        };

        let decision = client.record_decision(
            &lender,
            &proof_id,
            &proof_hash,
            &public_inputs_bytes,
            &policy,
            &symbol_short!("APPROVED"),
        );

        assert_eq!(decision, symbol_short!("APPROVED"));

        let record = client.get_record(&proof_id).unwrap();
        assert_eq!(record.decision, symbol_short!("APPROVED"));
        assert_eq!(record.proof_hash, proof_hash);
    }

    #[test]
    #[should_panic]
    fn test_policy_not_satisfied() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(LendingVerifier, ());
        let client = LendingVerifierClient::new(&env, &contract_id);

        let lender = Address::generate(&env);
        let proof_id = BytesN::from_array(&env, &[3u8; 16]);
        let proof_hash = BytesN::from_array(&env, &[4u8; 32]);

        // Revenue too low — 300M vs 500M required
        let inputs_raw: [u64; 8] = [300_000_000, 60_000_000, 5, 1200, 2000, 2000, 1, 24];
        let public_inputs_bytes = make_public_inputs_bytes(&env, &inputs_raw);

        let policy = LendingPolicy {
            required_monthly_revenue: 500_000_000,
            required_avg_balance: 50_000_000,
            required_positive_cf_months: 4,
            max_revenue_volatility_bps: 1500,
            max_customer_concentration_bps: 2500,
            max_debt_ratio_bps: 2500,
            require_no_missed_repayments: 1,
            required_account_age_months: 12,
        };

        client.record_decision(
            &lender,
            &proof_id,
            &proof_hash,
            &public_inputs_bytes,
            &policy,
            &symbol_short!("APPROVED"),
        );
    }
}
