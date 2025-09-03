#[doc = "Register `dll` reader"]
pub type R = crate::R<DllSpec>;
#[doc = "Register `dll` writer"]
pub type W = crate::W<DllSpec>;
#[doc = "Field `dll` reader - "]
pub type DllR = crate::FieldReader;
#[doc = "Field `dll` writer - "]
pub type DllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dll(&self) -> DllR {
        DllR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dll(&mut self) -> DllW<'_, DllSpec> {
        DllW::new(self, 0)
    }
}
#[doc = "UART Divisor Latch Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllSpec;
impl crate::RegisterSpec for DllSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dll::R`](R) reader structure"]
impl crate::Readable for DllSpec {}
#[doc = "`write(|w| ..)` method takes [`dll::W`](W) writer structure"]
impl crate::Writable for DllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dll to value 0"]
impl crate::Resettable for DllSpec {}
