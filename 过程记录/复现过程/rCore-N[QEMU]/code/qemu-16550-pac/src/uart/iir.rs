#[doc = "Register `iir` reader"]
pub type R = crate::R<IirSpec>;
#[doc = "Interrupt ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iid {
    #[doc = "0: `0`"]
    ModemStatus = 0,
    #[doc = "1: `1`"]
    NoInterruptPending = 1,
    #[doc = "2: `10`"]
    ThrEmpty = 2,
    #[doc = "3: `11`"]
    Rs485Interrupt = 3,
    #[doc = "4: `100`"]
    ReceivedDataAvailable = 4,
    #[doc = "6: `110`"]
    ReceiverLineStatus = 6,
    #[doc = "7: `111`"]
    BusyDetect = 7,
    #[doc = "12: `1100`"]
    CharacterTimeout = 12,
}
impl From<Iid> for u8 {
    #[inline(always)]
    fn from(variant: Iid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iid {
    type Ux = u8;
}
impl crate::IsEnum for Iid {}
#[doc = "Field `iid` reader - Interrupt ID"]
pub type IidR = crate::FieldReader<Iid>;
impl IidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Iid> {
        match self.bits {
            0 => Some(Iid::ModemStatus),
            1 => Some(Iid::NoInterruptPending),
            2 => Some(Iid::ThrEmpty),
            3 => Some(Iid::Rs485Interrupt),
            4 => Some(Iid::ReceivedDataAvailable),
            6 => Some(Iid::ReceiverLineStatus),
            7 => Some(Iid::BusyDetect),
            12 => Some(Iid::CharacterTimeout),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_modem_status(&self) -> bool {
        *self == Iid::ModemStatus
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == Iid::NoInterruptPending
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_thr_empty(&self) -> bool {
        *self == Iid::ThrEmpty
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_rs485_interrupt(&self) -> bool {
        *self == Iid::Rs485Interrupt
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_received_data_available(&self) -> bool {
        *self == Iid::ReceivedDataAvailable
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_receiver_line_status(&self) -> bool {
        *self == Iid::ReceiverLineStatus
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_busy_detect(&self) -> bool {
        *self == Iid::BusyDetect
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_character_timeout(&self) -> bool {
        *self == Iid::CharacterTimeout
    }
}
#[doc = "FIFOs Enable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Feflag {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "3: `11`"]
    Enable = 3,
}
impl From<Feflag> for u8 {
    #[inline(always)]
    fn from(variant: Feflag) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Feflag {
    type Ux = u8;
}
impl crate::IsEnum for Feflag {}
#[doc = "Field `feflag` reader - FIFOs Enable Flag"]
pub type FeflagR = crate::FieldReader<Feflag>;
impl FeflagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Feflag> {
        match self.bits {
            0 => Some(Feflag::Disable),
            3 => Some(Feflag::Enable),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Feflag::Disable
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Feflag::Enable
    }
}
impl R {
    #[doc = "Bits 0:3 - Interrupt ID"]
    #[inline(always)]
    pub fn iid(&self) -> IidR {
        IidR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 6:7 - FIFOs Enable Flag"]
    #[inline(always)]
    pub fn feflag(&self) -> FeflagR {
        FeflagR::new((self.bits >> 6) & 3)
    }
}
#[doc = "UART Interrupt Identity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IirSpec;
impl crate::RegisterSpec for IirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IirSpec {}
#[doc = "`reset()` method sets iir to value 0"]
impl crate::Resettable for IirSpec {}
