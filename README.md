# Solana NFT Royalty Pool

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Solana](https://img.shields.io/badge/Solana-1.16.0-blue)](https://solana.com)
[![Anchor](https://img.shields.io/badge/Anchor-0.28.0-purple)](https://www.anchor-lang.com)
[![Rust](https://img.shields.io/badge/Rust-1.70.0-orange)](https://www.rust-lang.org)

> A decentralized NFT royalty management system on Solana that enables fractional ownership and automated distribution of NFT royalties through a pool-based mechanism.

## Features

- **NFT Creation & Minting**: Create and mint NFTs with built-in royalty support
- **Fractional Royalty Ownership**: Buy and sell shares of NFT royalties
- **Automated Distribution**: Pool-based royalty distribution system
- **Solana Integration**: Built on Solana blockchain for fast and cost-effective transactions
- **Smart Contract Security**: Developed using Anchor framework for secure and reliable contracts

## Prerequisites

- Node.js (v16 or higher)
- Rust (latest stable)
- Solana CLI tools
- Anchor Framework
- Yarn package manager

## Quick Start

1. **Clone the repository**
```bash
git clone https://github.com/yourusername/solana-nft-royalty-pool.git
cd solana-nft-royalty-pool
```

2. **Install dependencies**
```bash
yarn install
```

3. **Build the program**
```bash
anchor build
```

4. **Deploy to localnet**
```bash
anchor deploy
```

5. **Run tests**
```bash
anchor test
```

### Testing

For detailed testing with transaction inspection:
```bash
anchor test --detach --skip-deploy
```
This keeps the validator running, allowing you to inspect transaction signatures on the explorer.

## Documentation

### NFT Creation vs Minting

| Aspect                | Creating an NFT                          | Minting an NFT                           |
| --------------------- | ---------------------------------------- | ---------------------------------------- |
| **Focus**             | Defining metadata and initializing mint  | Generating and assigning the token       |
| **Ownership**         | No ownership established yet             | Token ownership is assigned              |
| **Blockchain Action** | Setting up metadata and the mint account | Minting tokens and sending to an address |
| **State**             | Conceptual existence                     | Tangible existence as a token            |

### Core Functions

#### `initialize_contract(ctx)`

Initializes the contract state with:
- `total_items`: `u64` - Total number of NFTs
- `bump`: `u8` - PDA bump seed

#### `create_nft(ctx, symbol, name, uri)`

Creates a new NFT with metadata and royalty support.

**Parameters**:
- `ctx`: `Context<CreateNft>` - Transaction context
- `name`: `String` - NFT name
- `symbol`: `String` - NFT symbol
- `uri`: `String` - Metadata URI (IPFS)

**Accounts Required**:
- `payer`: Transaction initiator
- `mint`: NFT mint account
- `metadata`: NFT metadata account
- `edition_account`: Master edition account
- `token_account`: NFT holder account
- `royalty_pool_account`: Royalty management account
- `contract_state`: Global state account

#### `mint_nft`

Mints an existing NFT to a specified user.

**Parameters**:
- `ctx`: `Context<MintNft>` - Transaction context
- `amount`: `u64` - Number of tokens (always 1 for NFTs)

#### `buy_shares`

Enables fractional ownership of NFT royalties.

**Parameters**:
- `ctx`: `Context<BuyShares>` - Transaction context
- `share_amount`: `u64` - Number of shares to purchase

#### `add_royalties_to_pool`

Adds SOL to the royalty pool for distribution.

**Parameters**:
- `ctx`: `Context<AddRoyaltiesToPool>` - Transaction context
- `amount`: `u64` - Amount of SOL to add

## 🔒 Security

This project follows Solana's best practices for smart contract development:
- Comprehensive test coverage
- Anchor framework security features
- PDA-based account management
- Proper access control

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- [GitHub Issues](https://github.com/defaimaxi/solana-nft-royalty-pool/issues)
- [Twitter](https://twitter.com/defai_maxi)

## 🔗 Links

- [Solana Documentation](https://docs.solana.com)
- [Anchor Framework](https://www.anchor-lang.com)
- [Metaplex Documentation](https://docs.metaplex.com)

---

Built with ❤️ by [defaimaxi]
