use std::convert::{TryFrom, TryInto};
use crate::pascal_types::PascalAnsiString;

//Various installation step types
// {$define ssPreInstall := 0}
// {$define ssInstall := 1}
// {$define ssPostInstall := 2}
// {$define ssDone := 3}


// type TSetupStep = (ssPreInstall, ssInstall, ssPostInstall, ssDone);



// function RunInstaller(ByteCode: TbtString);
// begin
	
// 	const Params: array of Variant := [ssInstall];
// 	const procName: TbtString := 'CURSTEPCHANGED';

// 	return Exec.RunProcPN(Params, ProcName);
// end;

fn run_installer(byte_code: String) {
    const params = vec![TSetupStep::ssInstall];
    const proc_name = "CURSTEPCHANGED".to_string();

    // Exec.RunProcPN(Params, ProcName);
}

