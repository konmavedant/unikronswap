# unikronswap

📦 Project Root: unikron-sol-poc/

├── programs/                           # Anchor-based Solana smart contracts
│   └── unikron/                        # Main Anchor program
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs                  # Anchor entrypoint
│           ├── instructions/          # Instruction modules
│           │   ├── commit_trade.rs
│           │   ├── reveal_trade.rs
│           │   └── settle_trade.rs
│           ├── state/                 # Account data structures
│           │   ├── trade_intent.rs
│           │   ├── config.rs
│           │   └── fee_pools.rs
│           ├── constants.rs
│           └── errors.rs
├── migrations/
│   └── deploy.ts                      # Optional Anchor deployment scripts
├── tests/                             # Anchor unit/integration tests
│   └── trade_flow.ts
├── scripts/                           # Utility or helper scripts
│   ├── simulate_crosschain.ts
│   └── generate_test_intent.ts
├── client/                            # React frontend (Next.js)
│   ├── pages/
│   │   ├── index.tsx                  # Trade submission UI
│   │   └── confirm.tsx                # Reveal/settle step
│   ├── components/
│   │   ├── TradeForm.tsx
│   │   ├── CommitPhase.tsx
│   │   ├── RevealPhase.tsx
│   │   └── WalletConnect.tsx
│   ├── hooks/
│   │   ├── useWallet.ts               # Wallet adapter logic
│   │   └── useJupiterQuote.ts        # Fetch route data
│   ├── lib/
│   │   ├── intentSigner.ts           # Sign TradeIntent (with route hash)
│   │   ├── anchorProvider.ts         # Solana Anchor setup
│   │   └── hashUtils.ts              # Route hash generation
│   ├── constants/
│   │   └── tokens.ts                  # Supported tokens
│   ├── public/
│   ├── styles/
│   ├── package.json
│   └── README.md
├── .anchor/                           # Anchor-specific configs
│   └── Anchor.toml
├── .gitignore
├── README.md
└── Cargo.toml                         # Root Cargo file (if using multiple programs)
