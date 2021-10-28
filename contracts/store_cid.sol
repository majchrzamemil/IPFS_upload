pragma solidity 0.5.17;

contract StoreCID {
   
    // Dynamic array storring strings with CID
    string[] public CIDs;

    // Adds new CID to contract
    function addCID(string memory cid) public {
        CIDs.push(cid);
    }
    
}
