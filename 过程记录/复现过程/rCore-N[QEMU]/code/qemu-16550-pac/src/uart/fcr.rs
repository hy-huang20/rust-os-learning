#[doc = "Register `fcr` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "Field `fifoe` writer - "]
pub type FifoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfifor` writer - "]
pub type RfiforW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xfifor` writer - "]
pub type XfiforW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmam {
    #[doc = "0: `0`"]
    Mode0 = 0,
    #[doc = "1: `1`"]
    Mode1 = 1,
}
impl From<Dmam> for bool {
    #[inline(always)]
    fn from(variant: Dmam) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dmam` writer - "]
pub type DmamW<'a, REG> = crate::BitWriter<'a, REG, Dmam>;
impl<'a, REG> DmamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmam::Mode0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmam::Mode1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tft {
    #[doc = "0: `0`"]
    Empty = 0,
    #[doc = "1: `1`"]
    TwoCharacters = 1,
    #[doc = "2: `10`"]
    QuarterFull = 2,
    #[doc = "3: `11`"]
    HalfFull = 3,
}
impl From<Tft> for u8 {
    #[inline(always)]
    fn from(variant: Tft) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tft {
    type Ux = u8;
}
impl crate::IsEnum for Tft {}
#[doc = "Field `tft` writer - "]
pub type TftW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tft, crate::Safe>;
impl<'a, REG> TftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Tft::Empty)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn two_characters(self) -> &'a mut crate::W<REG> {
        self.variant(Tft::TwoCharacters)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn quarter_full(self) -> &'a mut crate::W<REG> {
        self.variant(Tft::QuarterFull)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn half_full(self) -> &'a mut crate::W<REG> {
        self.variant(Tft::HalfFull)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rt {
    #[doc = "0: `0`"]
    OneCharacter = 0,
    #[doc = "1: `1`"]
    QuarterFull = 1,
    #[doc = "2: `10`"]
    HalfFull = 2,
    #[doc = "3: `11`"]
    TwoLessThanFull = 3,
}
impl From<Rt> for u8 {
    #[inline(always)]
    fn from(variant: Rt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rt {
    type Ux = u8;
}
impl crate::IsEnum for Rt {}
#[doc = "Field `rt` writer - "]
pub type RtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rt, crate::Safe>;
impl<'a, REG> RtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn one_character(self) -> &'a mut crate::W<REG> {
        self.variant(Rt::OneCharacter)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn quarter_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rt::QuarterFull)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn half_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rt::HalfFull)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn two_less_than_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rt::TwoLessThanFull)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifoe(&mut self) -> FifoeW<'_, FcrSpec> {
        FifoeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfifor(&mut self) -> RfiforW<'_, FcrSpec> {
        RfiforW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xfifor(&mut self) -> XfiforW<'_, FcrSpec> {
        XfiforW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dmam(&mut self) -> DmamW<'_, FcrSpec> {
        DmamW::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tft(&mut self) -> TftW<'_, FcrSpec> {
        TftW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn rt(&mut self) -> RtW<'_, FcrSpec> {
        RtW::new(self, 6)
    }
}
#[doc = "UART FIFO Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets fcr to value 0"]
impl crate::Resettable for FcrSpec {}
