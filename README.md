# 🌉 Solana Stablecoin Project

Welcome to the Solana Stablecoin project! This decentralized application allows users to mint stablecoins by depositing USDC, leveraging the Pyth Oracle for real-time price updates. Built on Solana for high performance and scalability, this project empowers users to access stable assets seamlessly.

---

## 🛠 Features

- **Mint Stablecoins:** Users can deposit USDC and mint stablecoins pegged 1:1 to the deposited value.
- **Real-Time Price Feeds:** Utilizes the **Pyth Oracle** for accurate and up-to-date price data.
- **Fast and Low-Cost Transactions:** Powered by the Solana blockchain for high-speed and cost-efficient operations.
- **Transparent and Secure:** Built with Anchor framework and audited smart contract logic.

---

## 🔧 Technology Stack

- **Blockchain:** Solana
- **Smart Contracts:** Anchor Framework (Rust)
- **Price Feeds:** Pyth Oracle
- **Frontend:** NextJS, Aceternity UI, Tailwind CSS
- **Wallet Integration:** Solana Wallet Adapter

---

## 🚀 How It Works

1. **Deposit USDC:**
   - Users provide USDC to the program.
   - The program validates the deposit.

2. **Fetch Price Data:**
   - The program queries the **Pyth Oracle** for the latest price data.
   - The price feed ensures accurate valuation for minting.

3. **Mint Stablecoins:**
   - The program mints stablecoins equivalent to the value of the deposited USDC.
   - Stablecoins are transferred to the user's wallet.

---

## 📂 Folder Structure

```plaintext
📦solana-stablecoin
 ┣ 📂app                  # Frontend application
 ┃ ┣ 📂components         # React components
 ┃ ┗ 📂pages              # Next.js pages
 ┣ 📂programs             # Smart contracts
 ┃ ┣ 📂src                # Rust source code
 ┃ ┗ 📜Cargo.toml         # Rust dependencies
 ┣ 📂tests                # End-to-end tests
 ┗ 📜README.md            # Project documentation


```
## 👷 Scope for Improvement

- Add support for multiple collateral types.
- Enable multi-currency stablecoin minting.
- Implement advanced liquidation mechanisms.

## 🤝 Contributing

Please follow these steps to contribute

- Fork the repository.
- Create a feature branch.
- Submit a pull request.
