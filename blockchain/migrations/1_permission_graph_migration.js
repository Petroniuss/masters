const MyContract = artifacts.require("PermissionGraph");

module.exports = function(deployer) {
    deployer.deploy(MyContract);
};