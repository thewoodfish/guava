# Guava

## Financial Inclusion Through Private Credit Proofs

Guava is built around a simple financial inclusion goal: help creditworthy small businesses access capital even when they do not fit the paperwork-heavy model of traditional finance.

In many emerging markets, the businesses that need working capital most are not always the ones with audited accounts, formal collateral, or a clean paper trail. They may still have real revenue, repeat customers, positive cash flow, and repayment capacity. Guava gives those businesses a way to prove that financial health without surrendering their full financial lives to every lender.

## What We Built

Guava is a privacy-preserving credit product for small businesses. It lets a business prove that it meets a lender's credit criteria without revealing its bank statement, transaction history, customer names, balances, or raw financial data.

Instead of asking borrowers to submit sensitive documents to every lender, Guava turns financial behavior into zero-knowledge proofs. A lender learns only whether the borrower satisfies specific underwriting predicates, such as:

- Monthly revenue is above the lender's minimum threshold
- Average balance is high enough
- Cash flow has been positive for enough months
- Revenue volatility is within range
- Customer concentration is not too high
- Debt ratio is acceptable
- There are no missed repayments
- Account history is long enough

The result is a loan decision backed by cryptographic proof and recorded on Stellar.

## The Problem

Millions of SMEs in Nigeria and other emerging markets are financially active but excluded from formal credit. They are not necessarily excluded because they cannot repay. They are excluded because lending still depends on paperwork: audited accounts, certified statements, collateral, utility bills, business plans, and guarantors.

For small merchants, those documents are often missing or expensive to produce. But the business may still have strong revenue, healthy cash flow, repeat customers, and years of trading activity.

That creates an inclusion gap. Capital goes to businesses that can document themselves in the format banks expect, while many productive informal and semi-formal businesses remain invisible.

The current lending process has three failures:

- It rejects good businesses because they cannot produce the right paperwork.
- It forces borrowers who can apply to expose their entire financial history to every lender.
- It makes lenders over-rely on collateral and documents instead of live financial behavior.

Guava replaces document-based lending with proof-based lending.

## How It Works

1. A lender creates a lending profile and publishes its credit policy.
2. The policy is written to a Soroban smart contract on Stellar testnet.
3. A borrower uploads a bank statement export in XLSX format.
4. The backend parses transactions and computes financial metrics.
5. The borrower applies to a lender.
6. The lender triggers proof generation.
7. A Noir circuit checks the borrower's private metrics against the lender's public thresholds.
8. Barretenberg generates and verifies an UltraHonk zero-knowledge proof.
9. The backend records the decision on Stellar through the Soroban contract.
10. If approved, the contract can atomically disburse XLM to the borrower.

The lender never sees the borrower's raw statement or private financial metrics. The proof is the underwriting evidence.

## What Makes It Different

Guava does not use blockchain as a passive log after the fact. Stellar is part of the underwriting trust model:

- Lenders publish their criteria on-chain before applications are evaluated.
- Proofs are generated against those criteria.
- The loan decision is recorded on-chain with the proof hash and public inputs.
- Approved loans can trigger settlement in the same transaction.

This creates an auditable chain from policy publication to proof verification to loan decision and disbursement.

The broader impact is financial inclusion. Guava lowers the trust cost between lenders and underdocumented borrowers. Borrowers do not need to over-disclose. Lenders do not need to blindly trust self-reported numbers. Both sides can rely on verifiable proofs.

## Zero-Knowledge Proof Layer

The underwriting circuit is written in Noir and lives at:

```text
circuits/lending/src/main.nr
```

Private inputs are the borrower's actual financial values. Public inputs are the lender's thresholds. The circuit enforces eight lending predicates and only allows proof generation if all required constraints pass.

The proving pipeline uses:

- `nargo execute` for witness generation
- `bb write_vk --scheme ultra_honk` for verification key generation
- `bb prove --scheme ultra_honk` for proof generation
- `bb verify --scheme ultra_honk` for off-chain cryptographic verification

The proof package includes the proof bytes, verification key, public inputs, predicate results, circuit ID, and proof metadata. No raw financial values are included.

## Stellar and Soroban Layer

Guava uses a Soroban smart contract deployed on Stellar testnet. The contract supports:

- `publish_policy()` - stores lender underwriting criteria on-chain
- `set_loan_config()` - stores the lender's disbursement amount
- `record_decision()` - records the verified loan decision, proof hash, policy data, and public inputs

If the loan is approved, `record_decision()` can also transfer XLM to the borrower atomically.

Current contract:

```text
CBT2XJCW6BBK3U4GII5ESRQXJ3TPBPVE23F26VMSKHBP4O4S2VAVYKS5
```

## Product Experience

The app has two roles.

Borrowers can:

- Sign up and add a Stellar wallet address
- Upload an XLSX bank statement
- Compute financial metrics
- Browse lenders and their published criteria
- Apply without sharing documents
- Track application status

Lenders can:

- Create a lending profile
- Configure underwriting thresholds
- Publish policy data on Stellar
- View anonymized applications
- Generate and verify ZK proofs
- See predicate-level proof results
- Record approved or rejected decisions on-chain

## Tech Stack

- Frontend: Next.js, TypeScript, Tailwind
- Backend: Rust, Axum, SQLx
- Database: PostgreSQL
- ZK circuits: Noir
- Proving backend: Barretenberg UltraHonk
- Smart contracts: Soroban
- Settlement layer: Stellar testnet
- Statement parsing: XLSX ingestion with Rust
- Auth: JWT and bcrypt

## What Is Real in the Demo

This is not a mock approval screen. The demo performs the actual pipeline:

- Parses borrower statement data
- Computes financial metrics
- Builds a Noir `Prover.toml`
- Runs the Noir and Barretenberg toolchain
- Generates a real UltraHonk proof
- Verifies the proof off-chain
- Publishes lender criteria to Stellar
- Records loan decisions on Stellar testnet
- Displays proof metadata and transaction hashes in the UI

## Current Limitations

The current build uses manual XLSX upload as the data source. That is intentional for the hackathon MVP: it proves the lending workflow, ZK circuit, proof generation, verification, and Stellar settlement path end to end.

In production, Guava should connect directly to trusted financial data sources such as open banking APIs, POS providers, accounting systems, or bank feeds. That would prevent borrowers from submitting edited files while preserving the same proof-based lending flow.

## Financial Inclusion Impact

Guava expands access to credit by changing what counts as proof of creditworthiness.

For borrowers, it means:

- Less dependence on collateral, audited accounts, and formal paperwork
- More control over sensitive financial data
- A reusable proof that can be shared with multiple lenders
- Faster access to lenders who publish clear criteria

For lenders, it means:

- Stronger underwriting signals without handling raw statements
- Lower data custody and privacy risk
- Transparent, on-chain policy commitments
- Verifiable decisions that can be audited later

For the ecosystem, it means small businesses can be evaluated by actual financial behavior instead of only by formal documentation. That is especially important in markets where many SMEs are productive, digitally active, and underserved, but still considered too opaque by traditional lenders.

## Why It Matters

Guava gives SMEs a way to prove financial health without surrendering financial privacy. It gives lenders verifiable underwriting signals without forcing them to custody sensitive borrower data.

That matters because financial inclusion is not only about opening accounts or moving money. It is also about access to fair credit. If a business can prove it has the cash flow to repay, it should not be rejected simply because it lacks the right documents or collateral.

For markets where small businesses are productive but underdocumented, this changes the lending question from:

> "Can you give me all your financial documents?"

to:

> "Can you prove you meet my lending criteria?"

That is the core idea: prove financial health, not financial history.
