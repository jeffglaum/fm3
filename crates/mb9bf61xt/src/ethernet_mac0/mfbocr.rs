#[doc = "Register `MFBOCR` reader"]
pub type R = crate::R<MFBOCR_SPEC>;
#[doc = "Field `NMFH` reader - Number of Missed frame by HOST"]
pub type NMFH_R = crate::FieldReader<u16>;
#[doc = "Field `ONMFH` reader - Overflow NMFH"]
pub type ONMFH_R = crate::BitReader;
#[doc = "Field `NMFF` reader - Number of Missed frame by Ethernet-MAC"]
pub type NMFF_R = crate::FieldReader<u16>;
#[doc = "Field `ONMFF` reader - Overflow NMFF"]
pub type ONMFF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Number of Missed frame by HOST"]
    #[inline(always)]
    pub fn nmfh(&self) -> NMFH_R {
        NMFH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow NMFH"]
    #[inline(always)]
    pub fn onmfh(&self) -> ONMFH_R {
        ONMFH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Number of Missed frame by Ethernet-MAC"]
    #[inline(always)]
    pub fn nmff(&self) -> NMFF_R {
        NMFF_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow NMFF"]
    #[inline(always)]
    pub fn onmff(&self) -> ONMFF_R {
        ONMFF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Missed Frame and Buffer Overflow Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfbocr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MFBOCR_SPEC;
impl crate::RegisterSpec for MFBOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfbocr::R`](R) reader structure"]
impl crate::Readable for MFBOCR_SPEC {}
#[doc = "`reset()` method sets MFBOCR to value 0"]
impl crate::Resettable for MFBOCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
