#[doc = "Register `QICRH` reader"]
pub type R = crate::R<QICRH_SPEC>;
#[doc = "Register `QICRH` writer"]
pub type W = crate::W<QICRH_SPEC>;
#[doc = "Field `CDCIE` reader - Count inversion interrupt enable bit"]
pub type CDCIE_R = crate::BitReader;
#[doc = "Field `CDCIE` writer - Count inversion interrupt enable bit"]
pub type CDCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDCF` reader - Count inversion interrupt request flag bit"]
pub type CDCF_R = crate::BitReader;
#[doc = "Field `CDCF` writer - Count inversion interrupt request flag bit"]
pub type CDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRPC` reader - Last position counter direction bit"]
pub type DIRPC_R = crate::BitReader;
#[doc = "Field `DIROU` reader - Last position counter flow direction bit"]
pub type DIROU_R = crate::BitReader;
#[doc = "Field `QPCNRCMIE` reader - PC match and RC match interrupt enable bit"]
pub type QPCNRCMIE_R = crate::BitReader;
#[doc = "Field `QPCNRCMIE` writer - PC match and RC match interrupt enable bit"]
pub type QPCNRCMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QPCNRCMF` reader - PC match and RC match interrupt request flag bit"]
pub type QPCNRCMF_R = crate::BitReader;
#[doc = "Field `QPCNRCMF` writer - PC match and RC match interrupt request flag bit"]
pub type QPCNRCMF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Count inversion interrupt enable bit"]
    #[inline(always)]
    pub fn cdcie(&self) -> CDCIE_R {
        CDCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Count inversion interrupt request flag bit"]
    #[inline(always)]
    pub fn cdcf(&self) -> CDCF_R {
        CDCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Last position counter direction bit"]
    #[inline(always)]
    pub fn dirpc(&self) -> DIRPC_R {
        DIRPC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Last position counter flow direction bit"]
    #[inline(always)]
    pub fn dirou(&self) -> DIROU_R {
        DIROU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PC match and RC match interrupt enable bit"]
    #[inline(always)]
    pub fn qpcnrcmie(&self) -> QPCNRCMIE_R {
        QPCNRCMIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PC match and RC match interrupt request flag bit"]
    #[inline(always)]
    pub fn qpcnrcmf(&self) -> QPCNRCMF_R {
        QPCNRCMF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Count inversion interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cdcie(&mut self) -> CDCIE_W<QICRH_SPEC> {
        CDCIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Count inversion interrupt request flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn cdcf(&mut self) -> CDCF_W<QICRH_SPEC> {
        CDCF_W::new(self, 1)
    }
    #[doc = "Bit 4 - PC match and RC match interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn qpcnrcmie(&mut self) -> QPCNRCMIE_W<QICRH_SPEC> {
        QPCNRCMIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - PC match and RC match interrupt request flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn qpcnrcmf(&mut self) -> QPCNRCMF_W<QICRH_SPEC> {
        QPCNRCMF_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "High-Order Bytes of QPRC Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qicrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qicrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QICRH_SPEC;
impl crate::RegisterSpec for QICRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`qicrh::R`](R) reader structure"]
impl crate::Readable for QICRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qicrh::W`](W) writer structure"]
impl crate::Writable for QICRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets QICRH to value 0"]
impl crate::Resettable for QICRH_SPEC {
    const RESET_VALUE: u8 = 0;
}
