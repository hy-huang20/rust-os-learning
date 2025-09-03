#[doc = "Register `lcr` reader"]
pub type R = crate::R<LcrSpec>;
#[doc = "Register `lcr` writer"]
pub type W = crate::W<LcrSpec>;
#[doc = "Data Length Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dls {
    #[doc = "0: 5 bits"]
    Five = 0,
    #[doc = "1: 6 bits"]
    Six = 1,
    #[doc = "2: 7 bits"]
    Seven = 2,
    #[doc = "3: 8 bits"]
    Eight = 3,
}
impl From<Dls> for u8 {
    #[inline(always)]
    fn from(variant: Dls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dls {
    type Ux = u8;
}
impl crate::IsEnum for Dls {}
#[doc = "Field `dls` reader - Data Length Select"]
pub type DlsR = crate::FieldReader<Dls>;
impl DlsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dls {
        match self.bits {
            0 => Dls::Five,
            1 => Dls::Six,
            2 => Dls::Seven,
            3 => Dls::Eight,
            _ => unreachable!(),
        }
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == Dls::Five
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == Dls::Six
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == Dls::Seven
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == Dls::Eight
    }
}
#[doc = "Field `dls` writer - Data Length Select"]
pub type DlsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dls, crate::Safe>;
impl<'a, REG> DlsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn five(self) -> &'a mut crate::W<REG> {
        self.variant(Dls::Five)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn six(self) -> &'a mut crate::W<REG> {
        self.variant(Dls::Six)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(Dls::Seven)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(Dls::Eight)
    }
}
#[doc = "Number of stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: 1 stop bit"]
    One = 0,
    #[doc = "1: 1.5 stop bits when DLS(LCR\\[1:0\\]) is zero, else 2 stop bits"]
    Two = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `stop` reader - Number of stop bits"]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::One,
            true => Stop::Two,
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Stop::One
    }
    #[doc = "1.5 stop bits when DLS(LCR\\[1:0\\]) is zero, else 2 stop bits"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Stop::Two
    }
}
#[doc = "Field `stop` writer - Number of stop bits"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::One)
    }
    #[doc = "1.5 stop bits when DLS(LCR\\[1:0\\]) is zero, else 2 stop bits"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Two)
    }
}
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pen {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Pen> for bool {
    #[inline(always)]
    fn from(variant: Pen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pen` reader - Parity Enable"]
pub type PenR = crate::BitReader<Pen>;
impl PenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pen {
        match self.bits {
            false => Pen::Disabled,
            true => Pen::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pen::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pen::Enabled
    }
}
#[doc = "Field `pen` writer - Parity Enable"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG, Pen>;
impl<'a, REG> PenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Enabled)
    }
}
#[doc = "Even Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eps {
    #[doc = "0: `0`"]
    Odd = 0,
    #[doc = "1: `1`"]
    Even = 1,
    #[doc = "2: `10`"]
    Rs485Data = 2,
    #[doc = "3: `11`"]
    Rs485Addr = 3,
}
impl From<Eps> for u8 {
    #[inline(always)]
    fn from(variant: Eps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eps {
    type Ux = u8;
}
impl crate::IsEnum for Eps {}
#[doc = "Field `eps` reader - Even Parity Select"]
pub type EpsR = crate::FieldReader<Eps>;
impl EpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eps {
        match self.bits {
            0 => Eps::Odd,
            1 => Eps::Even,
            2 => Eps::Rs485Data,
            3 => Eps::Rs485Addr,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Eps::Odd
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Eps::Even
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_rs485_data(&self) -> bool {
        *self == Eps::Rs485Data
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_rs485_addr(&self) -> bool {
        *self == Eps::Rs485Addr
    }
}
#[doc = "Field `eps` writer - Even Parity Select"]
pub type EpsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eps, crate::Safe>;
impl<'a, REG> EpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Eps::Odd)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Eps::Even)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rs485_data(self) -> &'a mut crate::W<REG> {
        self.variant(Eps::Rs485Data)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rs485_addr(self) -> &'a mut crate::W<REG> {
        self.variant(Eps::Rs485Addr)
    }
}
#[doc = "Field `bc` reader - Break Control Bit"]
pub type BcR = crate::BitReader;
#[doc = "Field `bc` writer - Break Control Bit"]
pub type BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Divisor Latch Access Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlab {
    #[doc = "0: `0`"]
    RxBuffer = 0,
    #[doc = "1: `1`"]
    DivisorLatch = 1,
}
impl From<Dlab> for bool {
    #[inline(always)]
    fn from(variant: Dlab) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dlab` reader - Divisor Latch Access Bit"]
pub type DlabR = crate::BitReader<Dlab>;
impl DlabR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlab {
        match self.bits {
            false => Dlab::RxBuffer,
            true => Dlab::DivisorLatch,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_rx_buffer(&self) -> bool {
        *self == Dlab::RxBuffer
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_divisor_latch(&self) -> bool {
        *self == Dlab::DivisorLatch
    }
}
#[doc = "Field `dlab` writer - Divisor Latch Access Bit"]
pub type DlabW<'a, REG> = crate::BitWriter<'a, REG, Dlab>;
impl<'a, REG> DlabW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(Dlab::RxBuffer)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn divisor_latch(self) -> &'a mut crate::W<REG> {
        self.variant(Dlab::DivisorLatch)
    }
}
impl R {
    #[doc = "Bits 0:1 - Data Length Select"]
    #[inline(always)]
    pub fn dls(&self) -> DlsR {
        DlsR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Number of stop bits"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Even Parity Select"]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Break Control Bit"]
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DlabR {
        DlabR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Length Select"]
    #[inline(always)]
    pub fn dls(&mut self) -> DlsW<'_, LcrSpec> {
        DlsW::new(self, 0)
    }
    #[doc = "Bit 2 - Number of stop bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, LcrSpec> {
        StopW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<'_, LcrSpec> {
        PenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Even Parity Select"]
    #[inline(always)]
    pub fn eps(&mut self) -> EpsW<'_, LcrSpec> {
        EpsW::new(self, 4)
    }
    #[doc = "Bit 6 - Break Control Bit"]
    #[inline(always)]
    pub fn bc(&mut self) -> BcW<'_, LcrSpec> {
        BcW::new(self, 6)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&mut self) -> DlabW<'_, LcrSpec> {
        DlabW::new(self, 7)
    }
}
#[doc = "UART Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrSpec;
impl crate::RegisterSpec for LcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lcr to value 0"]
impl crate::Resettable for LcrSpec {}
