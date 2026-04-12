// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract Bounty is ReentrancyGuard, Ownable {
    struct BountyInfo {
        uint256 id;
        address creator;
        uint256 amount;
        address token;
        string description;
        bool isActive;
        address claimant;
    }
    
    uint256 private _bountyCounter;
    mapping(uint256 => BountyInfo) public bounties;
    
    event BountyCreated(
        uint256 indexed bountyId,
        address indexed creator,
        uint256 amount,
        address token,
        string description
    );
    
    event BountyClaimed(
        uint256 indexed bountyId,
        address indexed claimant
    );
    
    event BountyCancelled(
        uint256 indexed bountyId
    );
    
    function createBounty(
        uint256 amount,
        address token,
        string memory description
    ) external nonReentrant returns (uint256) {
        require(amount > 0, "Amount must be greater than 0");
        require(token != address(0), "Invalid token address");
        require(bytes(description).length > 0, "Description cannot be empty");
        
        _bountyCounter++;
        uint256 bountyId = _bountyCounter;
        
        // Transfer tokens from creator to contract
        IERC20(token).transferFrom(msg.sender, address(this), amount);
        
        // Create bounty
        bounties[bountyId] = BountyInfo({
            id: bountyId,
            creator: msg.sender,
            amount: amount,
            token: token,
            description: description,
            isActive: true,
            claimant: address(0)
        });
        
        emit BountyCreated(bountyId, msg.sender, amount, token, description);
        
        return bountyId;
    }
    
    function getBounty(uint256 bountyId) external view returns (BountyInfo memory) {
        require(bounties[bountyId].id != 0, "Bounty does not exist");
        return bounties[bountyId];
    }
    
    function getBountyCounter() external view returns (uint256) {
        return _bountyCounter;
    }
}