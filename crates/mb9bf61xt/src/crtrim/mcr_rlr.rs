#[doc = "Register `MCR_RLR` reader"]
pub type R = crate::R<MCR_RLR_SPEC>;
#[doc = "Register `MCR_RLR` writer"]
pub type W = crate::W<MCR_RLR_SPEC>;
#[doc = "Field `TRMLCK` reader - Register write-protect bits"]
pub type TRMLCK_R = crate::FieldReader<u32>;
#[doc = "Field `TRMLCK` writer - Register write-protect bits"]
pub type TRMLCK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register write-protect bits"]
    #[inline(always)]
    pub fn trmlck(&self) -> TRMLCK_R {
        TRMLCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register write-protect bits"]
    #[inline(always)]
    #[must_use]
    pub fn trmlck(&mut self) -> TRMLCK_W<MCR_RLR_SPEC> {
        TRMLCK_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "High-Speed CR Oscillation Register Write-Protect Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr_rlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr_rlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCR_RLR_SPEC;
impl crate::RegisterSpec for MCR_RLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr_rlr::R`](R) reader structure"]
impl crate::Readable for MCR_RLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcr_rlr::W`](W) writer structure"]
impl crate::Writable for MCR_RLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR_RLR to value 0x01"]
impl crate::Resettable for MCR_RLR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
