#[doc = "Register `PCFD` reader"]
pub type R = crate::R<PCFD_SPEC>;
#[doc = "Field `PC` reader - Conversion input channel bits"]
pub type PC_R = crate::FieldReader;
#[doc = "Field `RS` reader - Scan conversion start factor"]
pub type RS_R = crate::FieldReader;
#[doc = "Field `INVL` reader - A/D conversion result disable bit"]
pub type INVL_R = crate::BitReader;
#[doc = "Field `PD` reader - Priority conversion result"]
pub type PD_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:4 - Conversion input channel bits"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Scan conversion start factor"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - A/D conversion result disable bit"]
    #[inline(always)]
    pub fn invl(&self) -> INVL_R {
        INVL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 20:31 - Priority conversion result"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "Priority Conversion FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCFD_SPEC;
impl crate::RegisterSpec for PCFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcfd::R`](R) reader structure"]
impl crate::Readable for PCFD_SPEC {}
#[doc = "`reset()` method sets PCFD to value 0"]
impl crate::Resettable for PCFD_SPEC {
    const RESET_VALUE: u32 = 0;
}
