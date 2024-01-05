#[doc = "Register `SSIR` reader"]
pub type R = crate::R<SSIR_SPEC>;
#[doc = "Register `SSIR` writer"]
pub type W = crate::W<SSIR_SPEC>;
#[doc = "Field `SSINC` reader - Sub-Second Increment Value"]
pub type SSINC_R = crate::FieldReader;
#[doc = "Field `SSINC` writer - Sub-Second Increment Value"]
pub type SSINC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sub-Second Increment Value"]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sub-Second Increment Value"]
    #[inline(always)]
    #[must_use]
    pub fn ssinc(&mut self) -> SSINC_W<SSIR_SPEC> {
        SSINC_W::new(self, 0)
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
#[doc = "Sub-Second Increment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSIR_SPEC;
impl crate::RegisterSpec for SSIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssir::R`](R) reader structure"]
impl crate::Readable for SSIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssir::W`](W) writer structure"]
impl crate::Writable for SSIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSIR to value 0"]
impl crate::Resettable for SSIR_SPEC {
    const RESET_VALUE: u32 = 0;
}
