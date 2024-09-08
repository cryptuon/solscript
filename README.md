# SolanaScript: High-Level Language for Solana Development

SolanaScript is a high-level language designed to simplify Solana development while providing access to the full power of the Solana blockchain when needed.

## Key Features

1. **Intuitive Contract Structure**
   
   Write Solana programs using a familiar contract-style syntax:

   ```javascript
   contract TokenContract {
     @state totalSupply: u64;
     
     @public
     transfer(to: Address, amount: u64) {
       // Transfer logic here
     }
   }
   ```

2. **Automatic Account Management**
   
   Let SolanaScript handle account creation and management for you:

   ```javascript
   @account
   class TokenAccount {
     balance: u64;
     owner: Address;
   }

   contract TokenContract {
     @public
     createAccount(owner: Address) {
       // Automatically creates and initializes a TokenAccount
       return TokenAccount.create(owner);
     }
   }
   ```

3. **Simplified PDAs**
   
   Work with Program Derived Addresses (PDAs) effortlessly:

   ```javascript
   contract PDAPoweredContract {
     @pda(['user', 'settings'])
     userSettings: UserSettings;

     @public
     updateSettings(user: Address, newSettings: UserSettings) {
       // Automatically handles PDA derivation and bump seeds
       this.userSettings.set(user, newSettings);
     }
   }
   ```

4. **Easy Cross-Program Invocation**
   
   Interact with other Solana programs seamlessly:

   ```javascript
   import { TokenProgram } from '@solana/spl-token';

   contract TokenInteractor {
     @public
     transferTokens(from: Address, to: Address, amount: u64) {
       // Automatically handles CPI to the Token program
       TokenProgram.transfer(from, to, amount);
     }
   }
   ```

5. **Built-in Security Features**
   
   Benefit from automatic security checks:

   ```javascript
   contract SecureContract {
     @state balance: u64;

     @public
     withdraw(amount: u64) {
       // Automatic checks for overflow, underflow, and reentrancy
       this.balance -= amount;
       transfer(tx.sender, amount);
     }
   }
   ```

6. **Simplified Testing**
   
   Test your contracts with an intuitive testing framework:

   ```javascript
   import { assert, test } from '@solana/testing';

   test('TokenContract', (t) => {
     const token = new TokenContract(1000);
     token.transfer(Address('receiver'), 100);
     
     assert.equal(token.balanceOf(Address('receiver')), 100);
   });
   ```

## Core Solana Concepts

While SolanaScript simplifies many aspects of Solana development, understanding these core concepts remains important:

1. **Accounts**: Solana's fundamental storage unit. SolanaScript abstracts much of account handling, but developers should understand account ownership and rent economics.

2. **Instructions**: The basic unit of execution in Solana. SolanaScript's functions translate to instructions, but understanding instruction anatomy can help with advanced use cases.

3. **Transactions**: Groups of instructions. SolanaScript often handles transaction building, but developers should be aware of transaction limits and atomicity.

4. **Programs**: Smart contracts in Solana. SolanaScript contracts compile to Solana programs.

5. **Program Derived Addresses (PDAs)**: Accounts owned by programs. SolanaScript simplifies PDA usage, but understanding their purpose is crucial for advanced patterns.

6. **Rent**: The cost of storing data on Solana. SolanaScript handles many rent calculations, but developers should be aware of its impact on program design.

7. **Signer and Writable Permissions**: Access control for accounts. SolanaScript often infers these, but explicit control is available when needed.

## Advanced Capabilities

For developers who need fine-grained control, SolanaScript provides access to low-level Solana features:

```javascript
import { SolanaLow } from '@solana/low-level';

contract AdvancedContract {
  @public
  complexOperation() {
    // Use high-level abstractions
    this.simpleTransfer(receiver, amount);

    // Drop down to low-level operations when needed
    SolanaLow.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: newAccount.publicKey,
      lamports: rentExemptionAmount,
      space: 1024,
      programId: this.programId
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

1. Install SolanaScript:
   ```
   npm install -g solanascript
   ```

2. Create a new project:
   ```
   solanascript init my-project
   cd my-project
   ```

3. Write your contract in `src/main.sol`:
   ```javascript
   contract HelloWorld {
     @public
     greet(name: string): string {
       return `Hello, ${name}!`;
     }
   }
   ```

4. Compile and deploy:
   ```
   solanascript build
   solanascript deploy
   ```

## Learn More

- [SolanaScript Documentation](https://docs.solscript.xyz)

SolanaScript: Simplifying Solana development without sacrificing power!
