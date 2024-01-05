#[doc = "Register `SCFD` reader"]
pub type R = crate::R<SCFD_SPEC>;
#[doc = "Field `SC` reader - Conversion input channel bits"]
pub type SC_R = crate::FieldReader;
#[doc = "Field `RS` reader - Scan conversion start factor"]
pub type RS_R = crate::FieldReader;
#[doc = "Field `INVL` reader - A/D conversion result disable bit"]
pub type INVL_R = crate::BitReader;
#[doc = "Field `SD` reader - Scan conversion result"]
pub type SD_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:4 - Conversion input channel bits"]
    #[inline(always)]
    pub fn sc(&self) -> SC_R {
        SC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Scan conversion start factor"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - A/D conversion result disable bit"]
    #[inline(always)]
    pub fn invl(&self) -> INVL_R {
        INVL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 20:31 - Scan conversion result"]
    #[inline(always)]
    pub fn sd(&self) -> SD_R {
        SD_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "Scan Conversion FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCFD_SPEC;
impl crate::RegisterSpec for SCFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scfd::R`](R) reader structure"]
impl crate::Readable for SCFD_SPEC {}
#[doc = "`reset()` method sets SCFD to value 0"]
impl crate::Resettable for SCFD_SPEC {
    const RESET_VALUE: u32 = 0;
}
