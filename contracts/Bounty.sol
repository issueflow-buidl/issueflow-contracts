// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Bounty {
    address public owner;

    constructor() {
        owner = msg.sender;
    }
}