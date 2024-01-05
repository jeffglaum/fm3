#[doc = "Register `FSTR` reader"]
pub type R = crate::R<FSTR_SPEC>;
#[doc = "Field `RDY` reader - Flash Rdy"]
pub type RDY_R = crate::BitReader;
#[doc = "Field `HNG` reader - Flash Hang flag"]
pub type HNG_R = crate::BitReader;
#[doc = "Field `ERR` reader - Flash ECC Error"]
pub type ERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Flash Rdy"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Hang flag"]
    #[inline(always)]
    pub fn hng(&self) -> HNG_R {
        HNG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash ECC Error"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Flash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fstr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSTR_SPEC;
impl crate::RegisterSpec for FSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fstr::R`](R) reader structure"]
impl crate::Readable for FSTR_SPEC {}
#[doc = "`reset()` method sets FSTR to value 0"]
impl crate::Resettable for FSTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
