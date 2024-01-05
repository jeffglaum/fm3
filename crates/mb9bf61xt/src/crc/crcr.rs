#[doc = "Register `CRCR` reader"]
pub type R = crate::R<CRCR_SPEC>;
#[doc = "Field `D` reader - CRC Data"]
pub type D_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Data"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new(self.bits)
    }
}
#[doc = "CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCR_SPEC;
impl crate::RegisterSpec for CRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcr::R`](R) reader structure"]
impl crate::Readable for CRCR_SPEC {}
#[doc = "`reset()` method sets CRCR to value 0xffff_ffff"]
impl crate::Resettable for CRCR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
