#[doc = "Register `ier` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `ier` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Enable Received Data Available Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erbfi {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Erbfi> for bool {
    #[inline(always)]
    fn from(variant: Erbfi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `erbfi` reader - Enable Received Data Available Interrupt"]
pub type ErbfiR = crate::BitReader<Erbfi>;
impl ErbfiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erbfi {
        match self.bits {
            false => Erbfi::Disable,
            true => Erbfi::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Erbfi::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Erbfi::Enable
    }
}
#[doc = "Field `erbfi` writer - Enable Received Data Available Interrupt"]
pub type ErbfiW<'a, REG> = crate::BitWriter<'a, REG, Erbfi>;
impl<'a, REG> ErbfiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Erbfi::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Erbfi::Enable)
    }
}
#[doc = "Enable Transmit Holding Register Empty Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etbei {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Etbei> for bool {
    #[inline(always)]
    fn from(variant: Etbei) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `etbei` reader - Enable Transmit Holding Register Empty Interrupt"]
pub type EtbeiR = crate::BitReader<Etbei>;
impl EtbeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etbei {
        match self.bits {
            false => Etbei::Disable,
            true => Etbei::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Etbei::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Etbei::Enable
    }
}
#[doc = "Field `etbei` writer - Enable Transmit Holding Register Empty Interrupt"]
pub type EtbeiW<'a, REG> = crate::BitWriter<'a, REG, Etbei>;
impl<'a, REG> EtbeiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Etbei::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Etbei::Enable)
    }
}
#[doc = "Enable Receiver Line Status Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elsi {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Elsi> for bool {
    #[inline(always)]
    fn from(variant: Elsi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `elsi` reader - Enable Receiver Line Status Interrupt"]
pub type ElsiR = crate::BitReader<Elsi>;
impl ElsiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Elsi {
        match self.bits {
            false => Elsi::Disable,
            true => Elsi::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Elsi::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Elsi::Enable
    }
}
#[doc = "Field `elsi` writer - Enable Receiver Line Status Interrupt"]
pub type ElsiW<'a, REG> = crate::BitWriter<'a, REG, Elsi>;
impl<'a, REG> ElsiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Elsi::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Elsi::Enable)
    }
}
#[doc = "Enable Modem Status Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edssi {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Edssi> for bool {
    #[inline(always)]
    fn from(variant: Edssi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `edssi` reader - Enable Modem Status Interrupt"]
pub type EdssiR = crate::BitReader<Edssi>;
impl EdssiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edssi {
        match self.bits {
            false => Edssi::Disable,
            true => Edssi::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Edssi::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Edssi::Enable
    }
}
#[doc = "Field `edssi` writer - Enable Modem Status Interrupt"]
pub type EdssiW<'a, REG> = crate::BitWriter<'a, REG, Edssi>;
impl<'a, REG> EdssiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Edssi::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Edssi::Enable)
    }
}
#[doc = "RS485 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rs485IntEn {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Rs485IntEn> for bool {
    #[inline(always)]
    fn from(variant: Rs485IntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rs485_int_en` reader - RS485 Interrupt Enable"]
pub type Rs485IntEnR = crate::BitReader<Rs485IntEn>;
impl Rs485IntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rs485IntEn {
        match self.bits {
            false => Rs485IntEn::Disable,
            true => Rs485IntEn::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rs485IntEn::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rs485IntEn::Enable
    }
}
#[doc = "Field `rs485_int_en` writer - RS485 Interrupt Enable"]
pub type Rs485IntEnW<'a, REG> = crate::BitWriter<'a, REG, Rs485IntEn>;
impl<'a, REG> Rs485IntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rs485IntEn::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rs485IntEn::Enable)
    }
}
#[doc = "Programmable THRE Interrupt Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ptime {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Ptime> for bool {
    #[inline(always)]
    fn from(variant: Ptime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ptime` reader - Programmable THRE Interrupt Mode Enable"]
pub type PtimeR = crate::BitReader<Ptime>;
impl PtimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ptime {
        match self.bits {
            false => Ptime::Disable,
            true => Ptime::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ptime::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ptime::Enable
    }
}
#[doc = "Field `ptime` writer - Programmable THRE Interrupt Mode Enable"]
pub type PtimeW<'a, REG> = crate::BitWriter<'a, REG, Ptime>;
impl<'a, REG> PtimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ptime::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ptime::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Received Data Available Interrupt"]
    #[inline(always)]
    pub fn erbfi(&self) -> ErbfiR {
        ErbfiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn etbei(&self) -> EtbeiR {
        EtbeiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn elsi(&self) -> ElsiR {
        ElsiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt"]
    #[inline(always)]
    pub fn edssi(&self) -> EdssiR {
        EdssiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RS485 Interrupt Enable"]
    #[inline(always)]
    pub fn rs485_int_en(&self) -> Rs485IntEnR {
        Rs485IntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Programmable THRE Interrupt Mode Enable"]
    #[inline(always)]
    pub fn ptime(&self) -> PtimeR {
        PtimeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Received Data Available Interrupt"]
    #[inline(always)]
    pub fn erbfi(&mut self) -> ErbfiW<'_, IerSpec> {
        ErbfiW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn etbei(&mut self) -> EtbeiW<'_, IerSpec> {
        EtbeiW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn elsi(&mut self) -> ElsiW<'_, IerSpec> {
        ElsiW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt"]
    #[inline(always)]
    pub fn edssi(&mut self) -> EdssiW<'_, IerSpec> {
        EdssiW::new(self, 3)
    }
    #[doc = "Bit 4 - RS485 Interrupt Enable"]
    #[inline(always)]
    pub fn rs485_int_en(&mut self) -> Rs485IntEnW<'_, IerSpec> {
        Rs485IntEnW::new(self, 4)
    }
    #[doc = "Bit 7 - Programmable THRE Interrupt Mode Enable"]
    #[inline(always)]
    pub fn ptime(&mut self) -> PtimeW<'_, IerSpec> {
        PtimeW::new(self, 7)
    }
}
#[doc = "UART Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ier to value 0"]
impl crate::Resettable for IerSpec {}
