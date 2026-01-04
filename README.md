# SolScript: High-Level Language for Solana Development

SolScript is a high-level language designed to simplify Solana development while providing access to the full power of the Solana blockchain when needed.

## Key Features

1. **Intuitive Contract Structure**
   
   Write Solana programs using a familiar contract-style syntax:

   ```solscript
   contract TokenContract {
     @state totalSupply: u64;

     @public
     fn transfer(to: Address, amount: u64) {
       // Transfer logic here
     }
   }
   ```

2. **Automatic Account Management**

   Let SolScript handle account creation and management for you:

   ```solscript
   @account
   struct TokenAccount {
     balance: u64;
     owner: Address;
   }

   contract TokenContract {
     @public
     fn createAccount(owner: Address) {
       // Automatically creates and initializes a TokenAccount
       return TokenAccount.create(owner);
     }
   }
   ```

3. **Simplified PDAs**

   Work with Program Derived Addresses (PDAs) effortlessly:

   ```solscript
   contract PDAPoweredContract {
     @pda(["user", "settings"])
     userSettings: UserSettings;

     @public
     fn updateSettings(user: Address, newSettings: UserSettings) {
       // Automatically handles PDA derivation and bump seeds
       self.userSettings.set(user, newSettings);
     }
   }
   ```

4. **Easy Cross-Program Invocation**

   Interact with other Solana programs seamlessly:

   ```solscript
   import { Token } from "@solana/token";

   contract TokenInteractor {
     @public
     fn transferTokens(from: Address, to: Address, amount: u64) {
       // Automatically handles CPI to the Token program
       Token.transfer(from, to, amount);
     }
   }
   ```

5. **Built-in Security Features**

   Benefit from automatic security checks:

   ```solscript
   contract SecureContract {
     @state balance: u64;

     @public
     fn withdraw(amount: u64) {
       // Automatic checks for overflow, underflow, and reentrancy
       self.balance -= amount;
       transfer(tx.sender, amount);
     }
   }
   ```

6. **Simplified Testing**

   Test your contracts with an intuitive testing framework:

   ```solscript
   import { Test, assert } from "@solana/testing";

   #[test]
   fn test_token_transfer() {
     let token = TokenContract.new(1000);
     token.transfer(Address("receiver"), 100)?;

     assert.eq(token.balanceOf(Address("receiver")), 100);
   }
   ```

## Core Solana Concepts

While SolScript simplifies many aspects of Solana development, understanding these core concepts remains important:

1. **Accounts**: Solana's fundamental storage unit. SolScript abstracts much of account handling, but developers should understand account ownership and rent economics.

2. **Instructions**: The basic unit of execution in Solana. SolScript's functions translate to instructions, but understanding instruction anatomy can help with advanced use cases.

3. **Transactions**: Groups of instructions. SolScript often handles transaction building, but developers should be aware of transaction limits and atomicity.

4. **Programs**: Smart contracts in Solana. SolScript contracts compile to Solana programs.

5. **Program Derived Addresses (PDAs)**: Accounts owned by programs. SolScript simplifies PDA usage, but understanding their purpose is crucial for advanced patterns.

6. **Rent**: The cost of storing data on Solana. SolScript handles many rent calculations, but developers should be aware of its impact on program design.

7. **Signer and Writable Permissions**: Access control for accounts. SolScript often infers these, but explicit control is available when needed.

## Advanced Capabilities

For developers who need fine-grained control, SolScript provides access to low-level Solana features:

```solscript
import { LowLevel } from "@solana/low-level";

contract AdvancedContract {
  @public
  fn complexOperation() {
    // Use high-level abstractions
    self.simpleTransfer(receiver, amount);

    // Drop down to low-level operations when needed
    LowLevel.createAccount({
      fromPubkey: payer.address,
      newAccountPubkey: newAccount.address,
      lamports: rentExemptionAmount,
      space: 1024,
      programId: self.programId
    });
  }
}
```

## Best Suited For

- Rapid prototyping and development of Solana programs
- Developers new to blockchain or Solana
- Complex projects requiring both high-level abstractions and low-level control
- Teams looking for a language that scales with their expertise

## Getting Started

1. Install SolScript:
   ```
   cargo install solscript
   ```

2. Create a new project:
   ```
   solscript init my-project
   cd my-project
   ```

3. Write your contract in `src/main.ss`:
   ```solscript
   contract HelloWorld {
     @public
     fn greet(name: string): string {
       return `Hello, ${name}!`;
     }
   }
   ```

4. Compile and deploy:
   ```
   solscript build
   solscript deploy
   ```

## Learn More

- [SolScript Documentation](https://docs.solscript.xyz)

SolScript: Simplifying Solana development without sacrificing power!
