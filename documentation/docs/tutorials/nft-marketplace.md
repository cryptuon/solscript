# Build an NFT Marketplace

Create a marketplace for listing and trading NFTs.

## Prerequisites

- SolScript installed
- Understanding of NFTs and token transfers
- Completed the [Token Tutorial](token.md)

## What We'll Build

A marketplace with:
- List NFTs for sale
- Buy listed NFTs
- Cancel listings
- Platform fees
- Event logging

## Step 1: Data Structures

```solidity
contract NFTMarketplace {
    // Listing structure
    struct Listing {
        address seller;
        address nftContract;
        uint256 tokenId;
        uint256 price;
        bool active;
    }

    // State
    mapping(uint256 => Listing) public listings;
    uint256 public listingCount;
    uint256 public platformFee = 250; // 2.5% in basis points
    address public feeRecipient;
    address public owner;

    // Events
    event Listed(
        uint256 indexed listingId,
        address indexed seller,
        address indexed nftContract,
        uint256 tokenId,
        uint256 price
    );

    event Sold(
        uint256 indexed listingId,
        address indexed buyer,
        uint256 price
    );

    event Cancelled(uint256 indexed listingId);

    event FeeUpdated(uint256 newFee);

    // Errors
    error NotOwner();
    error NotSeller();
    error ListingNotActive();
    error InsufficientPayment(uint256 required, uint256 provided);
    error InvalidPrice();
    error TransferFailed();
}
```

## Step 2: NFT Interface

Define the interface for NFT contracts:

```solidity
interface IERC721 {
    function ownerOf(uint256 tokenId) external view returns (address);
    function transferFrom(address from, address to, uint256 tokenId) external;
    function approve(address to, uint256 tokenId) external;
    function getApproved(uint256 tokenId) external view returns (address);
}
```

## Step 3: Constructor and Modifiers

```solidity
constructor(address _feeRecipient) {
    owner = msg.sender;
    feeRecipient = _feeRecipient;
}

modifier onlyOwner() {
    if (msg.sender != owner) revert NotOwner();
    _;
}

modifier onlySeller(uint256 listingId) {
    if (listings[listingId].seller != msg.sender) revert NotSeller();
    _;
}

modifier isActiveListing(uint256 listingId) {
    if (!listings[listingId].active) revert ListingNotActive();
    _;
}
```

## Step 4: List NFT Function

```solidity
function listNFT(
    address nftContract,
    uint256 tokenId,
    uint256 price
) public returns (uint256) {
    if (price == 0) revert InvalidPrice();

    // Verify caller owns the NFT
    IERC721 nft = IERC721(nftContract);
    require(nft.ownerOf(tokenId) == msg.sender, "Not NFT owner");

    // Verify marketplace is approved
    require(
        nft.getApproved(tokenId) == address(this),
        "Marketplace not approved"
    );

    // Create listing
    listingCount++;
    listings[listingCount] = Listing({
        seller: msg.sender,
        nftContract: nftContract,
        tokenId: tokenId,
        price: price,
        active: true
    });

    emit Listed(listingCount, msg.sender, nftContract, tokenId, price);

    return listingCount;
}
```

## Step 5: Buy NFT Function

```solidity
function buyNFT(uint256 listingId) public payable isActiveListing(listingId) {
    Listing storage listing = listings[listingId];

    if (msg.value < listing.price) {
        revert InsufficientPayment(listing.price, msg.value);
    }

    // Mark as sold
    listing.active = false;

    // Calculate fees
    uint256 fee = (listing.price * platformFee) / 10000;
    uint256 sellerAmount = listing.price - fee;

    // Transfer NFT to buyer
    IERC721(listing.nftContract).transferFrom(
        listing.seller,
        msg.sender,
        listing.tokenId
    );

    // Transfer payment to seller (minus fee)
    // Note: In production, use safe transfer patterns

    // Transfer fee to platform
    // Note: Implement SOL transfers for Solana

    emit Sold(listingId, msg.sender, listing.price);
}
```

## Step 6: Cancel Listing

```solidity
function cancelListing(uint256 listingId)
    public
    onlySeller(listingId)
    isActiveListing(listingId)
{
    listings[listingId].active = false;
    emit Cancelled(listingId);
}
```

## Step 7: Admin Functions

```solidity
function updateFee(uint256 newFee) public onlyOwner {
    require(newFee <= 1000, "Fee too high"); // Max 10%
    platformFee = newFee;
    emit FeeUpdated(newFee);
}

function updateFeeRecipient(address newRecipient) public onlyOwner {
    require(newRecipient != address(0), "Invalid address");
    feeRecipient = newRecipient;
}

function transferOwnership(address newOwner) public onlyOwner {
    require(newOwner != address(0), "Invalid address");
    owner = newOwner;
}
```

## Step 8: View Functions

```solidity
function getListing(uint256 listingId) public view returns (
    address seller,
    address nftContract,
    uint256 tokenId,
    uint256 price,
    bool active
) {
    Listing storage listing = listings[listingId];
    return (
        listing.seller,
        listing.nftContract,
        listing.tokenId,
        listing.price,
        listing.active
    );
}

function calculateFee(uint256 price) public view returns (uint256) {
    return (price * platformFee) / 10000;
}
```

## Complete Contract

```solidity
interface IERC721 {
    function ownerOf(uint256 tokenId) external view returns (address);
    function transferFrom(address from, address to, uint256 tokenId) external;
    function getApproved(uint256 tokenId) external view returns (address);
}

contract NFTMarketplace {
    struct Listing {
        address seller;
        address nftContract;
        uint256 tokenId;
        uint256 price;
        bool active;
    }

    mapping(uint256 => Listing) public listings;
    uint256 public listingCount;
    uint256 public platformFee = 250;
    address public feeRecipient;
    address public owner;

    event Listed(uint256 indexed listingId, address indexed seller, address indexed nftContract, uint256 tokenId, uint256 price);
    event Sold(uint256 indexed listingId, address indexed buyer, uint256 price);
    event Cancelled(uint256 indexed listingId);
    event FeeUpdated(uint256 newFee);

    error NotOwner();
    error NotSeller();
    error ListingNotActive();
    error InsufficientPayment(uint256 required, uint256 provided);
    error InvalidPrice();

    modifier onlyOwner() {
        if (msg.sender != owner) revert NotOwner();
        _;
    }

    modifier onlySeller(uint256 listingId) {
        if (listings[listingId].seller != msg.sender) revert NotSeller();
        _;
    }

    modifier isActiveListing(uint256 listingId) {
        if (!listings[listingId].active) revert ListingNotActive();
        _;
    }

    constructor(address _feeRecipient) {
        owner = msg.sender;
        feeRecipient = _feeRecipient;
    }

    function listNFT(address nftContract, uint256 tokenId, uint256 price) public returns (uint256) {
        if (price == 0) revert InvalidPrice();

        IERC721 nft = IERC721(nftContract);
        require(nft.ownerOf(tokenId) == msg.sender, "Not NFT owner");
        require(nft.getApproved(tokenId) == address(this), "Marketplace not approved");

        listingCount++;
        listings[listingCount] = Listing({
            seller: msg.sender,
            nftContract: nftContract,
            tokenId: tokenId,
            price: price,
            active: true
        });

        emit Listed(listingCount, msg.sender, nftContract, tokenId, price);
        return listingCount;
    }

    function buyNFT(uint256 listingId) public payable isActiveListing(listingId) {
        Listing storage listing = listings[listingId];

        if (msg.value < listing.price) {
            revert InsufficientPayment(listing.price, msg.value);
        }

        listing.active = false;

        IERC721(listing.nftContract).transferFrom(
            listing.seller,
            msg.sender,
            listing.tokenId
        );

        emit Sold(listingId, msg.sender, listing.price);
    }

    function cancelListing(uint256 listingId) public onlySeller(listingId) isActiveListing(listingId) {
        listings[listingId].active = false;
        emit Cancelled(listingId);
    }

    function updateFee(uint256 newFee) public onlyOwner {
        require(newFee <= 1000, "Fee too high");
        platformFee = newFee;
        emit FeeUpdated(newFee);
    }

    function getListing(uint256 listingId) public view returns (
        address seller, address nftContract, uint256 tokenId, uint256 price, bool active
    ) {
        Listing storage listing = listings[listingId];
        return (listing.seller, listing.nftContract, listing.tokenId, listing.price, listing.active);
    }
}
```

## Build and Deploy

```bash
solscript build nft-marketplace.sol -o ./build
solscript deploy nft-marketplace.sol --network devnet
```

## Next Steps

- [Build an Escrow](escrow.md)
- [Token Tutorial](token.md)
