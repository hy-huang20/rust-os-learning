#[doc = "Register `mcr` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `mcr` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Data Terminal Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtr {
    #[doc = "0: `0`"]
    Deasserted = 0,
    #[doc = "1: `1`"]
    Asserted = 1,
}
impl From<Dtr> for bool {
    #[inline(always)]
    fn from(variant: Dtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dtr` reader - Data Terminal Ready"]
pub type DtrR = crate::BitReader<Dtr>;
impl DtrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtr {
        match self.bits {
            false => Dtr::Deasserted,
            true => Dtr::Asserted,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == Dtr::Deasserted
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Dtr::Asserted
    }
}
#[doc = "Field `dtr` writer - Data Terminal Ready"]
pub type DtrW<'a, REG> = crate::BitWriter<'a, REG, Dtr>;
impl<'a, REG> DtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut crate::W<REG> {
        self.variant(Dtr::Deasserted)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Dtr::Asserted)
    }
}
#[doc = "Request to Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rts {
    #[doc = "0: `0`"]
    Deasserted = 0,
    #[doc = "1: `1`"]
    Asserted = 1,
}
impl From<Rts> for bool {
    #[inline(always)]
    fn from(variant: Rts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rts` reader - Request to Send"]
pub type RtsR = crate::BitReader<Rts>;
impl RtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rts {
        match self.bits {
            false => Rts::Deasserted,
            true => Rts::Asserted,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == Rts::Deasserted
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Rts::Asserted
    }
}
#[doc = "Field `rts` writer - Request to Send"]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG, Rts>;
impl<'a, REG> RtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut crate::W<REG> {
        self.variant(Rts::Deasserted)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Rts::Asserted)
    }
}
#[doc = "Loop Back Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loop {
    #[doc = "0: `0`"]
    Normal = 0,
    #[doc = "1: `1`"]
    LoopBack = 1,
}
impl From<Loop> for bool {
    #[inline(always)]
    fn from(variant: Loop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `loop` reader - Loop Back Mode"]
pub type LoopR = crate::BitReader<Loop>;
impl LoopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loop {
        match self.bits {
            false => Loop::Normal,
            true => Loop::LoopBack,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Loop::Normal
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_loop_back(&self) -> bool {
        *self == Loop::LoopBack
    }
}
#[doc = "Field `loop` writer - Loop Back Mode"]
pub type LoopW<'a, REG> = crate::BitWriter<'a, REG, Loop>;
impl<'a, REG> LoopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Normal)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn loop_back(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::LoopBack)
    }
}
#[doc = "Auto Flow Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Afce {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Afce> for bool {
    #[inline(always)]
    fn from(variant: Afce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `afce` reader - Auto Flow Control Enable"]
pub type AfceR = crate::BitReader<Afce>;
impl AfceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afce {
        match self.bits {
            false => Afce::Disabled,
            true => Afce::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Afce::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Afce::Enabled
    }
}
#[doc = "Field `afce` writer - Auto Flow Control Enable"]
pub type AfceW<'a, REG> = crate::BitWriter<'a, REG, Afce>;
impl<'a, REG> AfceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Afce::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Afce::Enabled)
    }
}
#[doc = "UART Function: Select IrDA or RS485\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Function {
    #[doc = "0: `0`"]
    Uart = 0,
    #[doc = "1: `1`"]
    IrDaSir = 1,
    #[doc = "2: `10`"]
    Rs485 = 2,
}
impl From<Function> for u8 {
    #[inline(always)]
    fn from(variant: Function) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Function {
    type Ux = u8;
}
impl crate::IsEnum for Function {}
#[doc = "Field `function` reader - UART Function: Select IrDA or RS485"]
pub type FunctionR = crate::FieldReader<Function>;
impl FunctionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Function {
        match self.bits {
            0 => Function::Uart,
            1 => Function::IrDaSir,
            2 => Function::Rs485,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == Function::Uart
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ir_da_sir(&self) -> bool {
        *self == Function::IrDaSir
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_rs485(&self) -> bool {
        *self == Function::Rs485
    }
}
#[doc = "Field `function` writer - UART Function: Select IrDA or RS485"]
pub type FunctionW<'a, REG> = crate::FieldWriter<'a, REG, 2, Function>;
impl<'a, REG> FunctionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn uart(self) -> &'a mut crate::W<REG> {
        self.variant(Function::Uart)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ir_da_sir(self) -> &'a mut crate::W<REG> {
        self.variant(Function::IrDaSir)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rs485(self) -> &'a mut crate::W<REG> {
        self.variant(Function::Rs485)
    }
}
impl R {
    #[doc = "Bit 0 - Data Terminal Ready"]
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request to Send"]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    pub fn loop_(&self) -> LoopR {
        LoopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto Flow Control Enable"]
    #[inline(always)]
    pub fn afce(&self) -> AfceR {
        AfceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - UART Function: Select IrDA or RS485"]
    #[inline(always)]
    pub fn function(&self) -> FunctionR {
        FunctionR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Data Terminal Ready"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DtrW<'_, McrSpec> {
        DtrW::new(self, 0)
    }
    #[doc = "Bit 1 - Request to Send"]
    #[inline(always)]
    pub fn rts(&mut self) -> RtsW<'_, McrSpec> {
        RtsW::new(self, 1)
    }
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    pub fn loop_(&mut self) -> LoopW<'_, McrSpec> {
        LoopW::new(self, 4)
    }
    #[doc = "Bit 5 - Auto Flow Control Enable"]
    #[inline(always)]
    pub fn afce(&mut self) -> AfceW<'_, McrSpec> {
        AfceW::new(self, 5)
    }
    #[doc = "Bits 6:7 - UART Function: Select IrDA or RS485"]
    #[inline(always)]
    pub fn function(&mut self) -> FunctionW<'_, McrSpec> {
        FunctionW::new(self, 6)
    }
}
#[doc = "UART Modem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets mcr to value 0"]
impl crate::Resettable for McrSpec {}
