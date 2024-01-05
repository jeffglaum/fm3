#[doc = "Register `RPDR` reader"]
pub type R = crate::R<RPDR_SPEC>;
#[doc = "Register `RPDR` writer"]
pub type W = crate::W<RPDR_SPEC>;
#[doc = "Field `RPD` reader - Receive Poll Demand"]
pub type RPD_R = crate::FieldReader<u32>;
#[doc = "Field `RPD` writer - Receive Poll Demand"]
pub type RPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Poll Demand"]
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Poll Demand"]
    #[inline(always)]
    #[must_use]
    pub fn rpd(&mut self) -> RPD_W<RPDR_SPEC> {
        RPD_W::new(self, 0)
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
#[doc = "Receive Poll Demand Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPDR_SPEC;
impl crate::RegisterSpec for RPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpdr::R`](R) reader structure"]
impl crate::Readable for RPDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rpdr::W`](W) writer structure"]
impl crate::Writable for RPDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RPDR to value 0"]
impl crate::Resettable for RPDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
