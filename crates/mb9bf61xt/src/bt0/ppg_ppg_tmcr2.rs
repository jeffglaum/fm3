#[doc = "Register `PPG_TMCR2` reader"]
pub type R = crate::R<PPG_PPG_TMCR2_SPEC>;
#[doc = "Register `PPG_TMCR2` writer"]
pub type W = crate::W<PPG_PPG_TMCR2_SPEC>;
#[doc = "Field `CKS3` reader - Count clock selection bit"]
pub type CKS3_R = crate::BitReader;
#[doc = "Field `CKS3` writer - Count clock selection bit"]
pub type CKS3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Count clock selection bit"]
    #[inline(always)]
    pub fn cks3(&self) -> CKS3_R {
        CKS3_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Count clock selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn cks3(&mut self) -> CKS3_W<PPG_PPG_TMCR2_SPEC> {
        CKS3_W::new(self, 0)
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
#[doc = "Timer Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppg_ppg_tmcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppg_ppg_tmcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPG_PPG_TMCR2_SPEC;
impl crate::RegisterSpec for PPG_PPG_TMCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ppg_ppg_tmcr2::R`](R) reader structure"]
impl crate::Readable for PPG_PPG_TMCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppg_ppg_tmcr2::W`](W) writer structure"]
impl crate::Writable for PPG_PPG_TMCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PPG_TMCR2 to value 0"]
impl crate::Resettable for PPG_PPG_TMCR2_SPEC {
    const RESET_VALUE: u8 = 0;
}
