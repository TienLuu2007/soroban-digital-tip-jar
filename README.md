# Title
Soroban Digital Tip Jar

# Description
The Soroban Digital Tip Jar is a decentralized web3 application designed to allow content creators to receive direct financial support from their global audience. By leveraging the Stellar blockchain's Soroban smart contract platform, the project eliminates costly traditional payment processors and intermediaries. It provides a transparent, secure, and instant tipping mechanism where fans can fund a creator's public goal using stablecoins like USDC.

# Features
* **One-Click Tipping:** Fans can securely sign a transaction to send micro-payments or larger tips instantly.
* **Direct Ledger-to-Ledger Transfers:** Tips move directly from the fan's token balance to the creator's wallet without being permanently held by the platform.
* **Cryptographic Authorization:** Built-in security using Soroban's native account authentication (`require_auth`) ensuring funds can only be moved by the valid key holder.
* **Gas-Free Progress Tracking:** Includes an optimized, read-only balance function allowing frontends to dynamically update the total funding status without charging users network fees.
* **Single-Asset Initialization:** Safe configuration design that locks the contract to a single specific token asset (e.g., USDC) upon initial deployment to prevent errors.

# Contract
Contract link: [Contract CDLTWAARWFKCSPDWM6X2DB7VZMQWOXKIHZLK7V4RJZTW56C4IQIWJQ7C](https://stellar.expert/explorer/testnet/contract/CDLTWAARWFKCSPDWM6X2DB7VZMQWOXKIHZLK7V4RJZTW56C4IQIWJQ7C)

### Contract Deployment Confirmation
<img width="2772" height="1522" alt="image" src="https://github.com/user-attachments/assets/8c9e838d-ed1c-433f-b249-2664df2d99aa" />

# Future scopes
* **Multi-Asset Support:** Update the smart contract logic to accept a wider range of Stellar-native tokens and fiat-backed stablecoins.
* **Milestone & Reward Tiers:** Implement automated on-chain milestones that unlock specific tiers or metadata rewards (like a unique digital badge) for top tiers of contributors.
* **Time-Bound Campaigns:** Introduce decentralized crowd-funding campaign limits with self-enforcing deadlines and automated refund logic if goals are not met.

# Profile
* **Name:** [Tien Luu]
* **Skills:** Rust, Soroban Smart Contracts, Stellar Blockchain Development, C++, Python
* **GitHub:** [https://github.com/TienLuu2007]

## Project Vision
The vision behind the Digital Tip Jar is to champion the principle of **radical simplicity** in decentralized application design. 
In a web3 ecosystem often bogged down by overengineering and excessive state complexity, this project serves as a proof-of-concept that a highly functional, secure MVP requires only **one core on-chain transaction**. 
By keeping the smart contract lean and handling secondary presentation logic off-chain, we aim to lower the barrier to entry for both developers looking to build on Stellar and creators seeking frictionless, censorship-resistant monetization channels.
