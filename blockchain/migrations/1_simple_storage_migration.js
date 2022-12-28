var MyContract = artifacts.require("SimpleStorage");

module.exports = function(deployer) {
    // deployment steps
    deployer.deploy(MyContract);
};