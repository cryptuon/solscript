// Automated Market Maker (AMM)
//
// A simple constant product AMM (x * y = k) for token swaps.
// Demonstrates DeFi primitives: liquidity pools, swaps, and LP tokens.

contract SimpleAMM {
    // Pool State
    address public tokenA;
    address public tokenB;
    uint64 public reserveA;
    uint64 public reserveB;

    // LP Token tracking
    uint64 public totalLPSupply;
    mapping(address => uint64) public lpBalances;

    // Configuration
    address public owner;
    uint64 public swapFee;      // Fee in basis points (e.g., 30 = 0.3%)

    bool public initialized;

    // Events
    event PoolInitialized(address indexed tokenA, address indexed tokenB);
    event LiquidityAdded(address indexed provider, uint64 amountA, uint64 amountB, uint64 lpTokens);
    event LiquidityRemoved(address indexed provider, uint64 amountA, uint64 amountB, uint64 lpTokens);
    event Swap(address indexed user, address tokenIn, uint64 amountIn, address tokenOut, uint64 amountOut);
    event FeeUpdated(uint64 oldFee, uint64 newFee);

    // Constructor
    constructor(uint64 _swapFee) {
        owner = msg.sender;
        swapFee = _swapFee;
        initialized = false;
        totalLPSupply = 0;
        reserveA = 0;
        reserveB = 0;
    }

    // Initialize the pool with initial liquidity
    function initialize(
        address _tokenA,
        address _tokenB,
        uint64 amountA,
        uint64 amountB
    ) public {
        require(msg.sender == owner, "Not owner");
        require(!initialized, "Already initialized");
        require(amountA > 1000, "Insufficient initial liquidity A");
        require(amountB > 1000, "Insufficient initial liquidity B");

        tokenA = _tokenA;
        tokenB = _tokenB;
        reserveA = amountA;
        reserveB = amountB;

        // Calculate initial LP tokens using geometric mean approximation
        uint64 lpTokens = sqrt(amountA * amountB);

        // Lock minimum liquidity to prevent manipulation
        lpBalances[address(0)] = 1000;
        lpBalances[msg.sender] = lpTokens - 1000;
        totalLPSupply = lpTokens;

        initialized = true;

        emit PoolInitialized(_tokenA, _tokenB);
        emit LiquidityAdded(msg.sender, amountA, amountB, lpTokens - 1000);
    }

    // Add liquidity to the pool
    function addLiquidity(
        uint64 amountA,
        uint64 amountB,
        uint64 minLPTokens
    ) public returns (uint64) {
        require(initialized, "Not initialized");
        require(amountA > 0 && amountB > 0, "Zero amount");

        // Calculate optimal amounts based on current ratio
        uint64 optimalB = (amountA * reserveB) / reserveA;
        uint64 actualA = amountA;
        uint64 actualB = amountB;

        if (optimalB <= amountB) {
            actualB = optimalB;
        } else {
            uint64 optimalA = (amountB * reserveA) / reserveB;
            actualA = optimalA;
        }

        // Calculate LP tokens to mint
        uint64 lpFromA = (actualA * totalLPSupply) / reserveA;
        uint64 lpFromB = (actualB * totalLPSupply) / reserveB;
        uint64 lpTokens = lpFromA;
        if (lpFromB < lpFromA) {
            lpTokens = lpFromB;
        }

        require(lpTokens >= minLPTokens, "Slippage exceeded");

        // Update state
        reserveA = reserveA + actualA;
        reserveB = reserveB + actualB;
        lpBalances[msg.sender] = lpBalances[msg.sender] + lpTokens;
        totalLPSupply = totalLPSupply + lpTokens;

        emit LiquidityAdded(msg.sender, actualA, actualB, lpTokens);

        return lpTokens;
    }

    // Remove liquidity from the pool
    function removeLiquidity(
        uint64 lpTokens,
        uint64 minAmountA,
        uint64 minAmountB
    ) public returns (uint64, uint64) {
        require(initialized, "Not initialized");
        require(lpTokens > 0, "Zero amount");
        require(lpBalances[msg.sender] >= lpTokens, "Insufficient LP balance");

        // Calculate token amounts to return
        uint64 amountA = (lpTokens * reserveA) / totalLPSupply;
        uint64 amountB = (lpTokens * reserveB) / totalLPSupply;

        require(amountA >= minAmountA && amountB >= minAmountB, "Slippage exceeded");

        // Update state
        lpBalances[msg.sender] = lpBalances[msg.sender] - lpTokens;
        totalLPSupply = totalLPSupply - lpTokens;
        reserveA = reserveA - amountA;
        reserveB = reserveB - amountB;

        emit LiquidityRemoved(msg.sender, amountA, amountB, lpTokens);

        return (amountA, amountB);
    }

    // Swap exact amount of tokenA for tokenB
    function swapAForB(uint64 amountIn, uint64 minAmountOut) public returns (uint64) {
        require(initialized, "Not initialized");
        require(amountIn > 0, "Zero amount");

        // Apply fee (10000 basis points = 100%)
        uint64 amountInWithFee = amountIn * (10000 - swapFee);

        // Constant product formula
        uint64 numerator = amountInWithFee * reserveB;
        uint64 denominator = (reserveA * 10000) + amountInWithFee;
        uint64 amountOut = numerator / denominator;

        require(amountOut >= minAmountOut, "Slippage exceeded");
        require(amountOut < reserveB, "Insufficient liquidity");

        // Update reserves
        reserveA = reserveA + amountIn;
        reserveB = reserveB - amountOut;

        emit Swap(msg.sender, tokenA, amountIn, tokenB, amountOut);

        return amountOut;
    }

    // Swap exact amount of tokenB for tokenA
    function swapBForA(uint64 amountIn, uint64 minAmountOut) public returns (uint64) {
        require(initialized, "Not initialized");
        require(amountIn > 0, "Zero amount");

        // Apply fee
        uint64 amountInWithFee = amountIn * (10000 - swapFee);

        // Constant product formula
        uint64 numerator = amountInWithFee * reserveA;
        uint64 denominator = (reserveB * 10000) + amountInWithFee;
        uint64 amountOut = numerator / denominator;

        require(amountOut >= minAmountOut, "Slippage exceeded");
        require(amountOut < reserveA, "Insufficient liquidity");

        // Update reserves
        reserveB = reserveB + amountIn;
        reserveA = reserveA - amountOut;

        emit Swap(msg.sender, tokenB, amountIn, tokenA, amountOut);

        return amountOut;
    }

    // Get current price of tokenA in terms of tokenB (6 decimal precision)
    function getPriceAtoB() public view returns (uint64) {
        if (reserveA == 0) {
            return 0;
        }
        return (reserveB * 1000000) / reserveA;
    }

    // Get current price of tokenB in terms of tokenA (6 decimal precision)
    function getPriceBtoA() public view returns (uint64) {
        if (reserveB == 0) {
            return 0;
        }
        return (reserveA * 1000000) / reserveB;
    }

    // Calculate expected output for a given input
    function getAmountOut(uint64 amountIn, bool aToB) public view returns (uint64) {
        uint64 reserveIn = reserveA;
        uint64 reserveOut = reserveB;
        if (!aToB) {
            reserveIn = reserveB;
            reserveOut = reserveA;
        }

        uint64 amountInWithFee = amountIn * (10000 - swapFee);
        uint64 numerator = amountInWithFee * reserveOut;
        uint64 denominator = (reserveIn * 10000) + amountInWithFee;

        return numerator / denominator;
    }

    // Get pool reserves
    function getReserves() public view returns (uint64, uint64) {
        return (reserveA, reserveB);
    }

    // Get LP token balance
    function getLPBalance(address account) public view returns (uint64) {
        return lpBalances[account];
    }

    // Admin: Set swap fee
    function setSwapFee(uint64 newFee) public {
        require(msg.sender == owner, "Not owner");
        require(newFee <= 1000, "Fee too high");  // Max 10%
        uint64 oldFee = swapFee;
        swapFee = newFee;
        emit FeeUpdated(oldFee, newFee);
    }

    // Integer square root (Babylonian method)
    function sqrt(uint64 x) internal pure returns (uint64) {
        if (x == 0) {
            return 0;
        }

        uint64 z = (x + 1) / 2;
        uint64 y = x;

        while (z < y) {
            y = z;
            z = (x / z + z) / 2;
        }

        return y;
    }
}
