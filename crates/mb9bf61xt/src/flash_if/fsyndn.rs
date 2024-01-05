#[doc = "Register `FSYNDN` reader"]
pub type R = crate::R<FSYNDN_SPEC>;
#[doc = "Register `FSYNDN` writer"]
pub type W = crate::W<FSYNDN_SPEC>;
#[doc = "Field `SD` reader - Flash Sync"]
pub type SD_R = crate::FieldReader;
#[doc = "Field `SD` writer - Flash Sync"]
pub type SD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Flash Sync"]
    #[inline(always)]
    pub fn sd(&self) -> SD_R {
        SD_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Flash Sync"]
    #[inline(always)]
    #[must_use]
    pub fn sd(&mut self) -> SD_W<FSYNDN_SPEC> {
        SD_W::new(self, 0)
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
#[doc = "Flash Sync Down Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsyndn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsyndn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSYNDN_SPEC;
impl crate::RegisterSpec for FSYNDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsyndn::R`](R) reader structure"]
impl crate::Readable for FSYNDN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsyndn::W`](W) writer structure"]
impl crate::Writable for FSYNDN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSYNDN to value 0"]
impl crate::Resettable for FSYNDN_SPEC {
    const RESET_VALUE: u32 = 0;
}
