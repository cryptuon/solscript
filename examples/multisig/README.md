# Multi-Signature Wallet

A wallet that requires multiple owners to approve transactions before execution.

## Features Demonstrated

- Multi-party coordination
- Struct storage with mappings
- Owner management
- Transaction lifecycle
- Custom errors
- Modifiers for access control
- Array manipulation

## How It Works

1. **Setup**: Initialize with a list of owners and required approval count
2. **Submit**: Any owner can submit a transaction (auto-approves)
3. **Approve**: Other owners approve the transaction
4. **Execute**: Once enough approvals, any owner can execute

## Contract Interface

### State
- `owners` - List of wallet owners
- `requiredApprovals` - Number of approvals needed
- `transactions` - Mapping of transaction ID to Transaction struct

### Transaction Management
- `submitTransaction(destination, value, data)` - Submit new transaction
- `approve(txId)` - Approve a transaction
- `revokeApproval(txId)` - Revoke your approval
- `executeTransaction(txId)` - Execute approved transaction

### Owner Management
- `addOwner(address)` - Add new owner
- `removeOwner(address)` - Remove owner
- `changeRequirement(required)` - Change approval threshold

### View Functions
- `getOwners()` - List all owners
- `getTransaction(txId)` - Get transaction details
- `hasApproved(txId, owner)` - Check if owner approved
- `isConfirmed(txId)` - Check if transaction has enough approvals

## Example Usage

```solidity
// Deploy with 3 owners, requiring 2 approvals
constructor([owner1, owner2, owner3], 2)

// Owner 1 submits transaction (auto-approves)
submitTransaction(recipient, 1000, "")  // Returns txId = 0

// Owner 2 approves
approve(0)  // Now has 2/2 approvals

// Any owner can execute
executeTransaction(0)  // Executes the transaction
```

## Build & Deploy

```bash
solscript check multisig.sol
solscript build multisig.sol
```

## Security Considerations

- Ensure enough owners to prevent single point of failure
- Set appropriate approval threshold
- Consider time-locks for high-value transactions
