#[doc = "Register `rbr` reader"]
pub type R = crate::R<RbrSpec>;
#[doc = "Field `rbr` reader - "]
pub type RbrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rbr(&self) -> RbrR {
        RbrR::new(self.bits)
    }
}
#[doc = "UART Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbrSpec;
impl crate::RegisterSpec for RbrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rbr::R`](R) reader structure"]
impl crate::Readable for RbrSpec {}
#[doc = "`reset()` method sets rbr to value 0"]
impl crate::Resettable for RbrSpec {}
