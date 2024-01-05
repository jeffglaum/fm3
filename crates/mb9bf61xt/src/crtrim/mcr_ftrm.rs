#[doc = "Register `MCR_FTRM` reader"]
pub type R = crate::R<MCR_FTRM_SPEC>;
#[doc = "Register `MCR_FTRM` writer"]
pub type W = crate::W<MCR_FTRM_SPEC>;
#[doc = "Field `TRD` reader - Frequency trimming setup bits"]
pub type TRD_R = crate::FieldReader;
#[doc = "Field `TRD` writer - Frequency trimming setup bits"]
pub type TRD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frequency trimming setup bits"]
    #[inline(always)]
    pub fn trd(&self) -> TRD_R {
        TRD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frequency trimming setup bits"]
    #[inline(always)]
    #[must_use]
    pub fn trd(&mut self) -> TRD_W<MCR_FTRM_SPEC> {
        TRD_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "High-speed CR oscillation Frequency Trimming Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr_ftrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr_ftrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCR_FTRM_SPEC;
impl crate::RegisterSpec for MCR_FTRM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mcr_ftrm::R`](R) reader structure"]
impl crate::Readable for MCR_FTRM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcr_ftrm::W`](W) writer structure"]
impl crate::Writable for MCR_FTRM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets MCR_FTRM to value 0x7f"]
impl crate::Resettable for MCR_FTRM_SPEC {
    const RESET_VALUE: u16 = 0x7f;
}
