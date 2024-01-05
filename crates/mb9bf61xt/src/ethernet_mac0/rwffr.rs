#[doc = "Register `RWFFR` reader"]
pub type R = crate::R<RWFFR_SPEC>;
#[doc = "Register `RWFFR` writer"]
pub type W = crate::W<RWFFR_SPEC>;
#[doc = "Field `RWFFR31` reader - Remote Wake-up Frame Filter Register"]
pub type RWFFR31_R = crate::FieldReader<u32>;
#[doc = "Field `RWFFR31` writer - Remote Wake-up Frame Filter Register"]
pub type RWFFR31_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Remote Wake-up Frame Filter Register"]
    #[inline(always)]
    pub fn rwffr31(&self) -> RWFFR31_R {
        RWFFR31_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remote Wake-up Frame Filter Register"]
    #[inline(always)]
    #[must_use]
    pub fn rwffr31(&mut self) -> RWFFR31_W<RWFFR_SPEC> {
        RWFFR31_W::new(self, 0)
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
#[doc = "Remote Wake-up Frame Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwffr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwffr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RWFFR_SPEC;
impl crate::RegisterSpec for RWFFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rwffr::R`](R) reader structure"]
impl crate::Readable for RWFFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rwffr::W`](W) writer structure"]
impl crate::Writable for RWFFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RWFFR to value 0"]
impl crate::Resettable for RWFFR_SPEC {
    const RESET_VALUE: u32 = 0;
}
