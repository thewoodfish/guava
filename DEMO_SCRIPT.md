# Guava — Video Script

**Target length:** 3 minutes.

---

## User Story (0:00-0:50)

*Show: landing page, then borrower dashboard.*

> "This is Guava, a privacy-preserving credit product for small businesses."

> "Imagine a shop owner in Nigeria who needs working capital. Her business has revenue, repeat customers, and cash flow. But when she applies for a loan, the lender asks for audited accounts, certified bank statements, collateral, and guarantors."

> "She may be creditworthy, but she is underdocumented. And if she does apply, she has to hand over her full financial history to every lender."

> "Guava changes the question from: can you give me all your financial documents? to: can you prove you meet my lending criteria?"

> "The borrower should not have to expose customer names, balances, and every transaction just to access fair credit."

---

## How It Works (0:50-2:05)

*Show: lender signup, lender dashboard, lending criteria form.*

> "First, a lender publishes their criteria: minimum revenue, average balance, volatility limits, debt ratio, and account age."

*Click Save & Publish.*

> "Guava writes that policy to a Soroban smart contract on Stellar before any borrower is evaluated."

*Switch to borrower dashboard.*

> "The borrower uploads a bank statement export. In this hackathon build, the source is XLSX. In production, this connects to open banking, POS, or accounting feeds."

*Upload statement and compute metrics.*

> "Guava parses the transactions and computes financial metrics privately. The borrower sees their own summary, but the lender never receives the raw statement."

*Show Browse Lenders and apply.*

> "The borrower sees available lenders and applies with one click."

*Switch to lender applications tab.*

> "On the lender side, the application is anonymized. No bank statement, no customer list, no transaction history."

*Click Generate Proof.*

> "The lender triggers proof generation. Guava checks whether the borrower's private metrics satisfy the lender's public thresholds."

*Show proof result and Stellar section.*

> "The result is a loan decision backed by cryptographic proof. The lender sees pass or fail results, proof metadata, and a Stellar transaction hash. If approved, the decision is recorded on-chain and the contract can disburse XLM."

---

## The Tech (2:05-3:00)

*Show proof pipeline animation or proof result panel.*

> "Under the hood, Guava uses zero-knowledge proofs. The circuit is written in Noir. The private inputs are the borrower's real financial metrics. The public inputs are the lender's published thresholds."

> "The circuit checks eight underwriting predicates: revenue, balance, cash flow, volatility, concentration, debt ratio, missed repayments, and account age."

> "If the constraints pass, Barretenberg generates an UltraHonk proof, and the backend verifies it off-chain using `bb verify`."

*Show Stellar transaction / contract section.*

> "Stellar is not just a log. It is part of the trust model. The lender's policy is published before evaluation, and the final decision is recorded with the proof hash and public inputs."

> "That gives both sides an auditable trail: policy published, proof generated, decision recorded, and settlement handled."

*Show final approved result or landing page.*

> "Guava expands financial inclusion by letting small businesses prove financial health without giving up financial privacy."

> "Guava. Prove financial health, not financial history."

---

## Recording Notes

- Use two browser windows: one lender, one borrower.
- Let the proof generation animation run; it demonstrates the real pipeline.
- Zoom in on the predicate results, proof hash, and Stellar transaction hash.
- Keep the narration focused on financial inclusion first, then privacy, then the tech.
