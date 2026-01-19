// NFT - Non-Fungible Token collection
// Demonstrates ERC721-style NFT with metadata

contract NFT {
    // Token metadata
    string public name;
    string public symbol;
    string public baseURI;

    // Token state
    uint256 public totalSupply;
    uint256 public maxSupply;
    uint256 public mintPrice;

    mapping(uint256 => address) public ownerOf;
    mapping(address => uint256) public balanceOf;
    mapping(uint256 => address) public getApproved;
    mapping(address => mapping(address => bool)) public isApprovedForAll;
    mapping(uint256 => string) private _tokenURIs;

    // Ownership
    address public owner;
    bool public mintingEnabled;

    // Events
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);
    event Minted(address indexed to, uint256 indexed tokenId);

    // Errors
    error Unauthorized();
    error TokenNotFound();
    error MaxSupplyReached();
    error MintingDisabled();
    error InsufficientPayment();
    error InvalidAddress();
    error NotApproved();

    // Modifiers
    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }

    modifier tokenExists(uint256 tokenId) {
        if (ownerOf[tokenId] == address(0)) revert TokenNotFound();
        _;
    }

    // Constructor
    constructor(
        string memory _name,
        string memory _symbol,
        uint256 _maxSupply,
        uint256 _mintPrice
    ) {
        name = _name;
        symbol = _symbol;
        maxSupply = _maxSupply;
        mintPrice = _mintPrice;
        owner = msg.sender;
        mintingEnabled = true;
    }

    // Mint a new NFT
    function mint() public payable returns (uint256) {
        if (!mintingEnabled) revert MintingDisabled();
        if (totalSupply >= maxSupply) revert MaxSupplyReached();
        if (msg.value < mintPrice) revert InsufficientPayment();

        totalSupply++;
        uint256 tokenId = totalSupply;

        ownerOf[tokenId] = msg.sender;
        balanceOf[msg.sender]++;

        emit Transfer(address(0), msg.sender, tokenId);
        emit Minted(msg.sender, tokenId);

        return tokenId;
    }

    // Owner can mint for free
    function ownerMint(address to) public onlyOwner returns (uint256) {
        if (to == address(0)) revert InvalidAddress();
        if (totalSupply >= maxSupply) revert MaxSupplyReached();

        totalSupply++;
        uint256 tokenId = totalSupply;

        ownerOf[tokenId] = to;
        balanceOf[to]++;

        emit Transfer(address(0), to, tokenId);
        emit Minted(to, tokenId);

        return tokenId;
    }

    // Transfer NFT
    function transferFrom(address from, address to, uint256 tokenId)
        public
        tokenExists(tokenId)
    {
        if (to == address(0)) revert InvalidAddress();

        address tokenOwner = ownerOf[tokenId];
        if (from != tokenOwner) revert Unauthorized();

        // Check authorization
        if (msg.sender != tokenOwner &&
            getApproved[tokenId] != msg.sender &&
            !isApprovedForAll[tokenOwner][msg.sender]) {
            revert NotApproved();
        }

        // Clear approval
        getApproved[tokenId] = address(0);

        // Transfer
        balanceOf[from]--;
        balanceOf[to]++;
        ownerOf[tokenId] = to;

        emit Transfer(from, to, tokenId);
    }

    // Approve single token
    function approve(address to, uint256 tokenId) public tokenExists(tokenId) {
        address tokenOwner = ownerOf[tokenId];
        if (msg.sender != tokenOwner && !isApprovedForAll[tokenOwner][msg.sender]) {
            revert Unauthorized();
        }

        getApproved[tokenId] = to;
        emit Approval(tokenOwner, to, tokenId);
    }

    // Approve all tokens
    function setApprovalForAll(address operator, bool approved) public {
        if (operator == address(0)) revert InvalidAddress();
        isApprovedForAll[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }

    // Toggle minting
    function setMintingEnabled(bool enabled) public onlyOwner {
        mintingEnabled = enabled;
    }

    // Update mint price
    function setMintPrice(uint256 _price) public onlyOwner {
        mintPrice = _price;
    }

    // Set base URI
    function setBaseURI(string memory _baseURI) public onlyOwner {
        baseURI = _baseURI;
    }
}
