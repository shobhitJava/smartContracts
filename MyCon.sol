pragma solidity 0.4.24;

contract MyCon {
       uint256 private balance;
    event Transaction(
    
        address indexed to ,
        uint256 amount);
         
        function sendLink(address to , uint256 amount) external{
            require(amount>1);
            Transaction(to, amount);
            
        }
        
        function deposit() public payable {
            
            balance+= msg.value;
        }
}
