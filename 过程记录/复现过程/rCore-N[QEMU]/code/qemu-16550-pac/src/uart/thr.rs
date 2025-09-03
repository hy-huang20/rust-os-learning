#[doc = "Register `thr` writer"]
pub type W = crate::W<ThrSpec>;
#[doc = "Field `thr` writer - "]
pub type ThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn thr(&mut self) -> ThrW<'_, ThrSpec> {
        ThrW::new(self, 0)
    }
}
#[doc = "UART Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThrSpec;
impl crate::RegisterSpec for ThrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for ThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets thr to value 0"]
impl crate::Resettable for ThrSpec {}
