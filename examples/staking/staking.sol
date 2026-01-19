// Staking Pool
//
// A DeFi staking contract where users deposit tokens to earn rewards
// over time. Demonstrates time-based calculations and reward distribution.

contract StakingPool {
    // Configuration
    address public owner;
    uint64 public rewardRate;           // Rewards per second per staked token
    uint64 public lastUpdateTime;       // Last time rewards were calculated
    uint64 public rewardPerTokenStored; // Accumulated rewards per token

    uint64 public totalStaked;          // Total tokens staked in the pool
    uint64 public minimumStake;         // Minimum stake amount
    uint64 public lockDuration;         // Time tokens must be locked

    bool public paused;

    // User State
    mapping(address => uint64) public stakedBalance;
    mapping(address => uint64) public stakingTimestamp;
    mapping(address => uint64) public userRewardPerTokenPaid;
    mapping(address => uint64) public pendingRewards;

    // Events
    event Staked(address indexed user, uint64 amount, uint64 timestamp);
    event Unstaked(address indexed user, uint64 amount);
    event RewardsClaimed(address indexed user, uint64 amount);
    event RewardRateUpdated(uint64 oldRate, uint64 newRate);
    event PoolPaused(bool isPaused);

    // Constructor
    constructor(uint64 _rewardRate) {
        owner = msg.sender;
        rewardRate = _rewardRate;
        lastUpdateTime = block.timestamp;
        minimumStake = 100;      // Minimum 100 tokens
        lockDuration = 86400;    // 1 day in seconds
        paused = false;
        totalStaked = 0;
        rewardPerTokenStored = 0;
    }

    // Stake tokens in the pool
    function stake(uint64 amount) public {
        require(!paused, "Pool is paused");
        require(amount >= minimumStake, "Below minimum stake");

        // Update rewards before changing balances
        _updateReward(msg.sender);

        stakedBalance[msg.sender] = stakedBalance[msg.sender] + amount;
        stakingTimestamp[msg.sender] = block.timestamp;
        totalStaked = totalStaked + amount;

        emit Staked(msg.sender, amount, block.timestamp);
    }

    // Unstake tokens from the pool
    function unstake(uint64 amount) public {
        require(stakedBalance[msg.sender] >= amount, "Insufficient balance");

        // Check lock duration
        uint64 unlockTime = stakingTimestamp[msg.sender] + lockDuration;
        require(block.timestamp >= unlockTime, "Still locked");

        // Update rewards before changing balances
        _updateReward(msg.sender);

        stakedBalance[msg.sender] = stakedBalance[msg.sender] - amount;
        totalStaked = totalStaked - amount;

        emit Unstaked(msg.sender, amount);
    }

    // Claim accumulated rewards
    function claimRewards() public {
        _updateReward(msg.sender);

        uint64 reward = pendingRewards[msg.sender];
        require(reward > 0, "No rewards to claim");

        pendingRewards[msg.sender] = 0;

        emit RewardsClaimed(msg.sender, reward);
    }

    // Compound rewards - add to staked balance
    function compoundRewards() public {
        require(!paused, "Pool is paused");

        _updateReward(msg.sender);

        uint64 reward = pendingRewards[msg.sender];
        if (reward > 0) {
            pendingRewards[msg.sender] = 0;

            stakedBalance[msg.sender] = stakedBalance[msg.sender] + reward;
            totalStaked = totalStaked + reward;

            emit Staked(msg.sender, reward, block.timestamp);
        }
    }

    // Internal reward update
    function _updateReward(address account) internal {
        rewardPerTokenStored = rewardPerToken();
        lastUpdateTime = block.timestamp;

        if (account != address(0)) {
            pendingRewards[account] = earned(account);
            userRewardPerTokenPaid[account] = rewardPerTokenStored;
        }
    }

    // Calculate current reward per token
    function rewardPerToken() public view returns (uint64) {
        if (totalStaked == 0) {
            return rewardPerTokenStored;
        }

        uint64 timeDelta = block.timestamp - lastUpdateTime;
        uint64 rewardAccrued = timeDelta * rewardRate;

        return rewardPerTokenStored + (rewardAccrued * 1000000 / totalStaked);
    }

    // Calculate rewards earned by an account
    function earned(address account) public view returns (uint64) {
        uint64 balance = stakedBalance[account];
        uint64 rewardDelta = rewardPerToken() - userRewardPerTokenPaid[account];

        return (balance * rewardDelta / 1000000) + pendingRewards[account];
    }

    // Get time until tokens can be unstaked
    function timeUntilUnlock(address account) public view returns (uint64) {
        if (stakedBalance[account] == 0) {
            return 0;
        }

        uint64 unlockTime = stakingTimestamp[account] + lockDuration;

        if (block.timestamp >= unlockTime) {
            return 0;
        }

        return unlockTime - block.timestamp;
    }

    // Admin: Set reward rate
    function setRewardRate(uint64 newRate) public {
        require(msg.sender == owner, "Not owner");
        _updateReward(address(0));
        uint64 oldRate = rewardRate;
        rewardRate = newRate;
        emit RewardRateUpdated(oldRate, newRate);
    }

    // Admin: Set minimum stake
    function setMinimumStake(uint64 amount) public {
        require(msg.sender == owner, "Not owner");
        minimumStake = amount;
    }

    // Admin: Set lock duration
    function setLockDuration(uint64 duration) public {
        require(msg.sender == owner, "Not owner");
        lockDuration = duration;
    }

    // Admin: Pause/unpause pool
    function setPaused(bool _paused) public {
        require(msg.sender == owner, "Not owner");
        paused = _paused;
        emit PoolPaused(_paused);
    }

    // View: Get pool stats
    function getPoolStats() public view returns (uint64, uint64, uint64, uint64) {
        return (totalStaked, rewardRate, minimumStake, lockDuration);
    }

    // View: Get user info
    function getUserInfo(address user) public view returns (uint64, uint64, uint64, uint64) {
        return (
            stakedBalance[user],
            earned(user),
            stakingTimestamp[user],
            timeUntilUnlock(user)
        );
    }
}
