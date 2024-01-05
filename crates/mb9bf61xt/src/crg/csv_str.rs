#[doc = "Register `CSV_STR` reader"]
pub type R = crate::R<CSV_STR_SPEC>;
#[doc = "Field `MCMF` reader - Main clock failure detection flag"]
pub type MCMF_R = crate::BitReader;
#[doc = "Field `SCMF` reader - Sub clock failure detection flag"]
pub type SCMF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Main clock failure detection flag"]
    #[inline(always)]
    pub fn mcmf(&self) -> MCMF_R {
        MCMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sub clock failure detection flag"]
    #[inline(always)]
    pub fn scmf(&self) -> SCMF_R {
        SCMF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "CSV status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csv_str::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSV_STR_SPEC;
impl crate::RegisterSpec for CSV_STR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csv_str::R`](R) reader structure"]
impl crate::Readable for CSV_STR_SPEC {}
#[doc = "`reset()` method sets CSV_STR to value 0"]
impl crate::Resettable for CSV_STR_SPEC {
    const RESET_VALUE: u8 = 0;
}
