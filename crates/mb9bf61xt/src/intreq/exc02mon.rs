#[doc = "Register `EXC02MON` reader"]
pub type R = crate::R<EXC02MON_SPEC>;
#[doc = "Field `NMI` reader - External NMIX pin interrupt request"]
pub type NMI_R = crate::BitReader;
#[doc = "Field `HWINT` reader - Hardware watchdog timer interrupt request"]
pub type HWINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - External NMIX pin interrupt request"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hardware watchdog timer interrupt request"]
    #[inline(always)]
    pub fn hwint(&self) -> HWINT_R {
        HWINT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "EXC02 batch read register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exc02mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXC02MON_SPEC;
impl crate::RegisterSpec for EXC02MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exc02mon::R`](R) reader structure"]
impl crate::Readable for EXC02MON_SPEC {}
#[doc = "`reset()` method sets EXC02MON to value 0"]
impl crate::Resettable for EXC02MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
