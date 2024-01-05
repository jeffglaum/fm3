#[doc = "Register `HFCOMP` reader"]
pub type R = crate::R<HFCOMP_SPEC>;
#[doc = "Register `HFCOMP` writer"]
pub type W = crate::W<HFCOMP_SPEC>;
#[doc = "Field `FRAMECOMP` reader - frame compare data"]
pub type FRAMECOMP_R = crate::FieldReader;
#[doc = "Field `FRAMECOMP` writer - frame compare data"]
pub type FRAMECOMP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - frame compare data"]
    #[inline(always)]
    pub fn framecomp(&self) -> FRAMECOMP_R {
        FRAMECOMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - frame compare data"]
    #[inline(always)]
    #[must_use]
    pub fn framecomp(&mut self) -> FRAMECOMP_W<HFCOMP_SPEC> {
        FRAMECOMP_W::new(self, 0)
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
#[doc = "SOF Interrupt Frame Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfcomp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfcomp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFCOMP_SPEC;
impl crate::RegisterSpec for HFCOMP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcomp::R`](R) reader structure"]
impl crate::Readable for HFCOMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfcomp::W`](W) writer structure"]
impl crate::Writable for HFCOMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCOMP to value 0"]
impl crate::Resettable for HFCOMP_SPEC {
    const RESET_VALUE: u8 = 0;
}
