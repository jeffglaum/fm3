#[doc = "Register `SFNS` reader"]
pub type R = crate::R<SFNS_SPEC>;
#[doc = "Register `SFNS` writer"]
pub type W = crate::W<SFNS_SPEC>;
#[doc = "Field `SFS` reader - Scan conversion FIFO stage count setting bit"]
pub type SFS_R = crate::FieldReader;
#[doc = "Field `SFS` writer - Scan conversion FIFO stage count setting bit"]
pub type SFS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Scan conversion FIFO stage count setting bit"]
    #[inline(always)]
    pub fn sfs(&self) -> SFS_R {
        SFS_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Scan conversion FIFO stage count setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn sfs(&mut self) -> SFS_W<SFNS_SPEC> {
        SFS_W::new(self, 0)
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
#[doc = "Scan Conversion FIFO Stage Count Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfns::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfns::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFNS_SPEC;
impl crate::RegisterSpec for SFNS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sfns::R`](R) reader structure"]
impl crate::Readable for SFNS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sfns::W`](W) writer structure"]
impl crate::Writable for SFNS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SFNS to value 0"]
impl crate::Resettable for SFNS_SPEC {
    const RESET_VALUE: u8 = 0;
}
