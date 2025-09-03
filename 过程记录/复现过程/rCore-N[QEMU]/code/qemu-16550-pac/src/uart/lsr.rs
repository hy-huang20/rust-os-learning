#[doc = "Register `lsr` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Data Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dr {
    #[doc = "1: `1`"]
    Ready = 1,
}
impl From<Dr> for bool {
    #[inline(always)]
    fn from(variant: Dr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dr` reader - Data Ready"]
pub type DrR = crate::BitReader<Dr>;
impl DrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dr> {
        match self.bits {
            true => Some(Dr::Ready),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Dr::Ready
    }
}
#[doc = "Overrun Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oe {
    #[doc = "1: `1`"]
    Error = 1,
}
impl From<Oe> for bool {
    #[inline(always)]
    fn from(variant: Oe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `oe` reader - Overrun Error"]
pub type OeR = crate::BitReader<Oe>;
impl OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Oe> {
        match self.bits {
            true => Some(Oe::Error),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Oe::Error
    }
}
#[doc = "Parity Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "1: `1`"]
    Error = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pe` reader - Parity Error"]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pe> {
        match self.bits {
            true => Some(Pe::Error),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Pe::Error
    }
}
#[doc = "Framing Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "1: `1`"]
    Error = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `fe` reader - Framing Error"]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fe> {
        match self.bits {
            true => Some(Fe::Error),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Fe::Error
    }
}
#[doc = "Field `bi` reader - Break Interrupt"]
pub type BiR = crate::BitReader;
#[doc = "TX Holding Register Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thre {
    #[doc = "1: `1`"]
    Empty = 1,
}
impl From<Thre> for bool {
    #[inline(always)]
    fn from(variant: Thre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `thre` reader - TX Holding Register Empty"]
pub type ThreR = crate::BitReader<Thre>;
impl ThreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Thre> {
        match self.bits {
            true => Some(Thre::Empty),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Thre::Empty
    }
}
#[doc = "Transmitter Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Temt {
    #[doc = "1: `1`"]
    Empty = 1,
}
impl From<Temt> for bool {
    #[inline(always)]
    fn from(variant: Temt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `temt` reader - Transmitter Empty"]
pub type TemtR = crate::BitReader<Temt>;
impl TemtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Temt> {
        match self.bits {
            true => Some(Temt::Empty),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Temt::Empty
    }
}
#[doc = "RX Data Error in FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoerr {
    #[doc = "1: `1`"]
    Error = 1,
}
impl From<Fifoerr> for bool {
    #[inline(always)]
    fn from(variant: Fifoerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `fifoerr` reader - RX Data Error in FIFO"]
pub type FifoerrR = crate::BitReader<Fifoerr>;
impl FifoerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fifoerr> {
        match self.bits {
            true => Some(Fifoerr::Error),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Fifoerr::Error
    }
}
impl R {
    #[doc = "Bit 0 - Data Ready"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt"]
    #[inline(always)]
    pub fn bi(&self) -> BiR {
        BiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX Holding Register Empty"]
    #[inline(always)]
    pub fn thre(&self) -> ThreR {
        ThreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty"]
    #[inline(always)]
    pub fn temt(&self) -> TemtR {
        TemtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Data Error in FIFO"]
    #[inline(always)]
    pub fn fifoerr(&self) -> FifoerrR {
        FifoerrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`reset()` method sets lsr to value 0"]
impl crate::Resettable for LsrSpec {}
