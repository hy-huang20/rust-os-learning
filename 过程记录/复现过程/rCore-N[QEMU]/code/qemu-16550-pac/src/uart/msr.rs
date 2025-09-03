#[doc = "Register `msr` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Delta Clear to Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcts {
    #[doc = "0: `0`"]
    NoChange = 0,
    #[doc = "1: `1`"]
    Change = 1,
}
impl From<Dcts> for bool {
    #[inline(always)]
    fn from(variant: Dcts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dcts` reader - Delta Clear to Send"]
pub type DctsR = crate::BitReader<Dcts>;
impl DctsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcts {
        match self.bits {
            false => Dcts::NoChange,
            true => Dcts::Change,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == Dcts::NoChange
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == Dcts::Change
    }
}
#[doc = "Delta Data Set Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddsr {
    #[doc = "0: `0`"]
    NoChange = 0,
    #[doc = "1: `1`"]
    Change = 1,
}
impl From<Ddsr> for bool {
    #[inline(always)]
    fn from(variant: Ddsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ddsr` reader - Delta Data Set Ready"]
pub type DdsrR = crate::BitReader<Ddsr>;
impl DdsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddsr {
        match self.bits {
            false => Ddsr::NoChange,
            true => Ddsr::Change,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == Ddsr::NoChange
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == Ddsr::Change
    }
}
#[doc = "Trailing Edge Ring Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teri {
    #[doc = "0: `0`"]
    NoChange = 0,
    #[doc = "1: `1`"]
    Change = 1,
}
impl From<Teri> for bool {
    #[inline(always)]
    fn from(variant: Teri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `teri` reader - Trailing Edge Ring Indicator"]
pub type TeriR = crate::BitReader<Teri>;
impl TeriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teri {
        match self.bits {
            false => Teri::NoChange,
            true => Teri::Change,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == Teri::NoChange
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == Teri::Change
    }
}
#[doc = "Delta Data Carrier Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddcd {
    #[doc = "0: `0`"]
    NoChange = 0,
    #[doc = "1: `1`"]
    Change = 1,
}
impl From<Ddcd> for bool {
    #[inline(always)]
    fn from(variant: Ddcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ddcd` reader - Delta Data Carrier Detect"]
pub type DdcdR = crate::BitReader<Ddcd>;
impl DdcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddcd {
        match self.bits {
            false => Ddcd::NoChange,
            true => Ddcd::Change,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == Ddcd::NoChange
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == Ddcd::Change
    }
}
#[doc = "Line State of Clear To Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cts {
    #[doc = "0: `0`"]
    Deasserted = 0,
    #[doc = "1: `1`"]
    Asserted = 1,
}
impl From<Cts> for bool {
    #[inline(always)]
    fn from(variant: Cts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cts` reader - Line State of Clear To Send"]
pub type CtsR = crate::BitReader<Cts>;
impl CtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cts {
        match self.bits {
            false => Cts::Deasserted,
            true => Cts::Asserted,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == Cts::Deasserted
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Cts::Asserted
    }
}
#[doc = "Line State of Data Set Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsr {
    #[doc = "0: `0`"]
    Deasserted = 0,
    #[doc = "1: `1`"]
    Asserted = 1,
}
impl From<Dsr> for bool {
    #[inline(always)]
    fn from(variant: Dsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dsr` reader - Line State of Data Set Ready"]
pub type DsrR = crate::BitReader<Dsr>;
impl DsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsr {
        match self.bits {
            false => Dsr::Deasserted,
            true => Dsr::Asserted,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == Dsr::Deasserted
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Dsr::Asserted
    }
}
#[doc = "Line State of Ring Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ri {
    #[doc = "0: `0`"]
    Deasserted = 0,
    #[doc = "1: `1`"]
    Asserted = 1,
}
impl From<Ri> for bool {
    #[inline(always)]
    fn from(variant: Ri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ri` reader - Line State of Ring Indicator"]
pub type RiR = crate::BitReader<Ri>;
impl RiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ri {
        match self.bits {
            false => Ri::Deasserted,
            true => Ri::Asserted,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == Ri::Deasserted
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Ri::Asserted
    }
}
#[doc = "Line State of Data Carrier Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcd {
    #[doc = "0: `0`"]
    Deasserted = 0,
    #[doc = "1: `1`"]
    Asserted = 1,
}
impl From<Dcd> for bool {
    #[inline(always)]
    fn from(variant: Dcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dcd` reader - Line State of Data Carrier Detect"]
pub type DcdR = crate::BitReader<Dcd>;
impl DcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcd {
        match self.bits {
            false => Dcd::Deasserted,
            true => Dcd::Asserted,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == Dcd::Deasserted
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Dcd::Asserted
    }
}
impl R {
    #[doc = "Bit 0 - Delta Clear to Send"]
    #[inline(always)]
    pub fn dcts(&self) -> DctsR {
        DctsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Delta Data Set Ready"]
    #[inline(always)]
    pub fn ddsr(&self) -> DdsrR {
        DdsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge Ring Indicator"]
    #[inline(always)]
    pub fn teri(&self) -> TeriR {
        TeriR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Delta Data Carrier Detect"]
    #[inline(always)]
    pub fn ddcd(&self) -> DdcdR {
        DdcdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line State of Clear To Send"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Line State of Data Set Ready"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Line State of Ring Indicator"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Line State of Data Carrier Detect"]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART Modem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`reset()` method sets msr to value 0"]
impl crate::Resettable for MsrSpec {}
