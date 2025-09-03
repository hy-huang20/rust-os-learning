#[doc = "Register `sch` reader"]
pub type R = crate::R<SchSpec>;
#[doc = "Register `sch` writer"]
pub type W = crate::W<SchSpec>;
#[doc = "Field `scratch` reader - "]
pub type ScratchR = crate::FieldReader;
#[doc = "Field `scratch` writer - "]
pub type ScratchW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn scratch(&self) -> ScratchR {
        ScratchR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn scratch(&mut self) -> ScratchW<'_, SchSpec> {
        ScratchW::new(self, 0)
    }
}
#[doc = "UART Scratch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SchSpec;
impl crate::RegisterSpec for SchSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sch::R`](R) reader structure"]
impl crate::Readable for SchSpec {}
#[doc = "`write(|w| ..)` method takes [`sch::W`](W) writer structure"]
impl crate::Writable for SchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sch to value 0"]
impl crate::Resettable for SchSpec {}
