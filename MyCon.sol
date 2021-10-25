// SPDX-License-Identifier: MIT
pragma solidity 0.6.0;

import "@chainlink/contracts/src/v0.6/ChainlinkClient.sol";
import "@chainlink/contracts/src/v0.6/vendor/Ownable.sol";

contract MyCon is ChainlinkClient, Ownable {

        event Transaction(
    
        address indexed to ,
        uint256 amount);

  
  constructor() public Ownable() {
    setPublicChainlinkToken();
  }

  function getChainlinkToken() public view returns (address) {
    return chainlinkTokenAddress();
  }

  function withdrawLink() public onlyOwner {
    LinkTokenInterface link = LinkTokenInterface(chainlinkTokenAddress());
    require(link.transfer(msg.sender, link.balanceOf(address(this))), "Unable to transfer");
  }
  
  function sendLink(address to , uint amount) external
{
  require(amount >1);
    require(to == 0x16E560640683638fAeEE86e916C2C663228560a9)  ; 
    LinkTokenInterface link = LinkTokenInterface(chainlinkTokenAddress());
    link.transfer(msg.sender, amount);
    emit Transaction(to,amount);
    
}  
}
