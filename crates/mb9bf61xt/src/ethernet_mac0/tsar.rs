#[doc = "Register `TSAR` reader"]
pub type R = crate::R<TSAR_SPEC>;
#[doc = "Register `TSAR` writer"]
pub type W = crate::W<TSAR_SPEC>;
#[doc = "Field `TSAR` reader - Time Stamp Addend Register"]
pub type TSAR_R = crate::FieldReader<u32>;
#[doc = "Field `TSAR` writer - Time Stamp Addend Register"]
pub type TSAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Time Stamp Addend Register"]
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time Stamp Addend Register"]
    #[inline(always)]
    #[must_use]
    pub fn tsar(&mut self) -> TSAR_W<TSAR_SPEC> {
        TSAR_W::new(self, 0)
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
#[doc = "Time Stamp Addend Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSAR_SPEC;
impl crate::RegisterSpec for TSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsar::R`](R) reader structure"]
impl crate::Readable for TSAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsar::W`](W) writer structure"]
impl crate::Writable for TSAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSAR to value 0"]
impl crate::Resettable for TSAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
