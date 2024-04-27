use crate::pascal_types::{PascalAnsiString, PascalDynamicArray, PascalTPSExec, TSetupStep};

#[link(name = "inno-cli", kind = "static")]
extern "C" {
    fn GenerateExec(byte_code: PascalAnsiString) -> PascalTPSExec;
    fn TPSExecRunProcPN(
        exec: PascalTPSExec,
        //Should be PascalDynamicArray<PascalVariant> but Rust can't handle PascalVariant's size being unknown
        //at compile time
        params: PascalDynamicArray<TSetupStep>,
        proc_name: PascalAnsiString,
    ); //This should return a PascalVariant but Rust can't handle PascalVariant's size being unknown at compile time
}

pub fn generate_exec(byte_code: Vec<u8>) -> PascalTPSExec {
    let byte_code: PascalAnsiString = PascalAnsiString::try_from(byte_code).unwrap();
    println!("Generating exec");
    unsafe { GenerateExec(byte_code) }
}

pub fn run_installer(byte_code: Vec<u8>) {
    let setup_step: &mut [TSetupStep] = &mut [TSetupStep::ssInstall];
    let params: PascalDynamicArray<TSetupStep> =
        PascalDynamicArray::<TSetupStep>::try_from(setup_step).unwrap();
    let proc_name: PascalAnsiString = PascalAnsiString::try_from("CURSTEPCHANGED").unwrap();

    let exec: PascalTPSExec = generate_exec(byte_code);
    println!("Running installer");
    unsafe { TPSExecRunProcPN(exec, params, proc_name) };
}
