# Staking Pool

A DeFi staking contract where users deposit tokens to earn rewards over time.

## Features Demonstrated

- Time-based reward calculations
- Reward per token accounting
- Lock duration enforcement
- Compound rewards
- APR calculation
- Admin controls with pause functionality
- Modifier chaining

## How It Works

1. **Stake**: Users deposit tokens into the pool
2. **Accrue**: Rewards accumulate based on stake amount and time
3. **Claim**: Users claim earned rewards at any time
4. **Unstake**: After lock period, users can withdraw principal

## Reward Mechanism

Rewards are distributed per second based on:
- `rewardRate`: Tokens distributed per second
- `stakedBalance`: User's share of the pool
- `totalStaked`: Total tokens in the pool

```
userReward = (stakedBalance / totalStaked) * rewardRate * time
```

## Contract Interface

### Core Functions
- `stake(amount)` - Deposit tokens (minimum required)
- `unstake(amount)` - Withdraw after lock period
- `claimRewards()` - Claim accumulated rewards
- `compoundRewards()` - Add rewards to stake

### View Functions
- `earned(account)` - Calculate pending rewards
- `rewardPerToken()` - Current reward rate per token
- `timeUntilUnlock(account)` - Seconds until unstake allowed
- `getAPR()` - Current annual percentage rate
- `getPoolStats()` - Pool configuration
- `getUserInfo(user)` - User's staking details

### Admin Functions
- `setRewardRate(rate)` - Adjust reward distribution
- `setMinimumStake(amount)` - Set minimum stake
- `setLockDuration(duration)` - Set lock period
- `setPaused(paused)` - Pause/unpause pool

## Example Usage

```solidity
// Deploy pool
constructor(stakingToken, rewardToken, 100)  // 100 tokens/second

// User stakes 1000 tokens
stake(1000)

// After 1 day, check rewards
earned(userAddress)  // Returns accumulated rewards

// Claim rewards
claimRewards()

// After lock period, unstake
unstake(1000)
```

## Build & Deploy

```bash
solscript check staking.sol
solscript build staking.sol
```

## Security Considerations

- Ensure reward token supply covers emissions
- Set appropriate lock durations
- Monitor total staked vs rewards balance
- Consider emergency withdrawal mechanism
