# Project Title

Tax Filing – A Soroban Smart Contract for Tax Submission Receipts on Stellar

## Project Vision

This project provides a **tax filing smart contract on the Stellar blockchain** using Soroban. It enables tax authorities to create tax years with deadlines and allows citizens to submit tax filings with on-chain verification and approval workflows.

---

## Description

A Soroban smart contract dApp that manages tax filing submissions on Stellar Testnet. Citizens submit filings with amount and document hash, which the tax authority can then approve or reject. All filing statuses are stored permanently on-chain for transparency and verification.

---

## Features

### 1. Tax Year Management
- Tax authority creates tax years with submission deadlines
- Each year is uniquely identified by its year number
- Deadline is stored as a Unix timestamp

### 2. Filing Submission
- Citizens submit tax filings with amount and document hash
- Each citizen can submit one filing per tax year
- Filing is stored on-chain with "pending" status

### 3. Approval Workflow
- Tax authority reviews and approves filings
- Tax authority can reject filings
- Status changes to "approved" or "rejected"

### 4. On-chain Transparency
- All filings stored permanently on Stellar blockchain
- Anyone can query filing status and details
- Complete audit trail of filing decisions

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CAPDJNF6WBP4GAPAKEB335ZZQ7ZB2GJAKTTQGKK62I7QF25HKBWSFBGS](https://stellar.expert/explorer/testnet/tx/e44f4c2f52cb4ebb69ae519d404a0259b5b0f993188b07ac093091caaa6785d6)

![screenshot](https://i.ibb.co/xqB3bCGM/image.png)

---

## Future Scopes

### 1. Automated Deadline Enforcement
- Prevent submissions after deadline passes
- Emit events for deadline approaching

### 2. Multi-year Archive
- Support historical filing retrieval
- Archive old tax years

### 3. Document Verification
- Integrate with off-chain document verification oracles
- Store IPFS hashes for document retrieval

### 4. Penalty Calculation
- Auto-calculate penalties for late filings
- Support penalty waivers

### 5. Frontend Dashboard
- Build a web interface for citizens to submit filings
- Admin dashboard for tax authority to review and manage filings

### 6. Audit Trail
- Emit events for all filing state changes
- Generate compliance reports

---

## Profile

- **Name:** chinhnghia812
