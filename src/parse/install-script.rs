//Try to parse commands from install script:
//DLL functions called and args
//Registry keys and values
//INI file keys and values
//Files and directories
//Icons and shortcuts
//Commandline commands and parameters

//Maybe hook EXPANDCONSTANT and other functions to get the values of the constants,
//Then stub the rest.
//Maybe do this for all commands within CURSTEPCHANGED, so we can fetch all values.