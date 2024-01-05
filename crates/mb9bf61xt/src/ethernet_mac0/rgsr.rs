#[doc = "Register `RGSR` reader"]
pub type R = crate::R<RGSR_SPEC>;
#[doc = "Field `LM` reader - Link Mode"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LSP` reader - Link Speed"]
pub type LSP_R = crate::FieldReader;
#[doc = "Field `LS` reader - Link Status"]
pub type LS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Link Mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Link Speed"]
    #[inline(always)]
    pub fn lsp(&self) -> LSP_R {
        LSP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Link Status"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "RGMII Status Register)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGSR_SPEC;
impl crate::RegisterSpec for RGSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgsr::R`](R) reader structure"]
impl crate::Readable for RGSR_SPEC {}
#[doc = "`reset()` method sets RGSR to value 0"]
impl crate::Resettable for RGSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
