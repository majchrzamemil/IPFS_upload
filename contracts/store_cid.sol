pragma solidity 0.5.17;

// Simplest possible contract for storring multiple CIDs
contract StoreCID {
   
    // Dynamic array storring strings with CID
    // Left public for test purpouses, in general
    // it should be private.
    string[] public CIDs;

    // Adds new CID to contract
    function addCID(string memory cid) public {
        CIDs.push(cid);
    }
    
}
