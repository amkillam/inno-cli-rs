#![allow(non_camel_case_types)]
//Camelcase is used sparingly for setup steps, to match the original Inno Setup PascalScript

#[repr(C)]
pub enum TSetupStep {
    ssPreInstall,
    ssInstall,
    ssPostInstall,
    ssDone,
}

impl TryFrom<i32> for TSetupStep {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TSetupStep::ssPreInstall),
            1 => Ok(TSetupStep::ssInstall),
            2 => Ok(TSetupStep::ssPostInstall),
            3 => Ok(TSetupStep::ssDone),
            _ => Err(()),
        }
    }
}

impl TryInto<i32> for TSetupStep {
    type Error = ();

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            TSetupStep::ssPreInstall => Ok(0),
            TSetupStep::ssInstall => Ok(1),
            TSetupStep::ssPostInstall => Ok(2),
            TSetupStep::ssDone => Ok(3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_i32() {
        assert_eq!(TSetupStep::ssPreInstall, 0.try_into().unwrap());
        assert_eq!(TSetupStep::ssInstall, 1.try_into().unwrap());
        assert_eq!(TSetupStep::ssPostInstall, 2.try_into().unwrap());
        assert_eq!(TSetupStep::ssDone, 3.try_into().unwrap());
    }

    #[test]
    fn test_try_into_i32() {
        assert_eq!(0, TSetupStep::ssPreInstall.try_into().unwrap());
        assert_eq!(1, TSetupStep::ssInstall.try_into().unwrap());
        assert_eq!(2, TSetupStep::ssPostInstall.try_into().unwrap());
        assert_eq!(3, TSetupStep::ssDone.try_into().unwrap());
    }
}

