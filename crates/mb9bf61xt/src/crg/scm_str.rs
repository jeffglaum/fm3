#[doc = "Register `SCM_STR` reader"]
pub type R = crate::R<SCM_STR_SPEC>;
#[doc = "Field `MORDY` reader - Main clock oscillation stable bit"]
pub type MORDY_R = crate::BitReader;
#[doc = "Field `SORDY` reader - Sub clock oscillation stable bit"]
pub type SORDY_R = crate::BitReader;
#[doc = "Field `PLRDY` reader - PLL oscillation stable bit"]
pub type PLRDY_R = crate::BitReader;
#[doc = "Field `RCM` reader - Master clock selection bits"]
pub type RCM_R = crate::FieldReader;
impl R {
    #[doc = "Bit 1 - Main clock oscillation stable bit"]
    #[inline(always)]
    pub fn mordy(&self) -> MORDY_R {
        MORDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Sub clock oscillation stable bit"]
    #[inline(always)]
    pub fn sordy(&self) -> SORDY_R {
        SORDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL oscillation stable bit"]
    #[inline(always)]
    pub fn plrdy(&self) -> PLRDY_R {
        PLRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Master clock selection bits"]
    #[inline(always)]
    pub fn rcm(&self) -> RCM_R {
        RCM_R::new((self.bits >> 5) & 7)
    }
}
#[doc = "System Clock Mode Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scm_str::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCM_STR_SPEC;
impl crate::RegisterSpec for SCM_STR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scm_str::R`](R) reader structure"]
impl crate::Readable for SCM_STR_SPEC {}
#[doc = "`reset()` method sets SCM_STR to value 0"]
impl crate::Resettable for SCM_STR_SPEC {
    const RESET_VALUE: u8 = 0;
}
