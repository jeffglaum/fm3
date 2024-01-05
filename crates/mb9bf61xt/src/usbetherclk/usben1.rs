#[doc = "Register `USBEN1` reader"]
pub type R = crate::R<USBEN1_SPEC>;
#[doc = "Register `USBEN1` writer"]
pub type W = crate::W<USBEN1_SPEC>;
#[doc = "Field `USBEN1` reader - USB1 enable bit"]
pub type USBEN1_R = crate::BitReader;
#[doc = "Field `USBEN1` writer - USB1 enable bit"]
pub type USBEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB1 enable bit"]
    #[inline(always)]
    pub fn usben1(&self) -> USBEN1_R {
        USBEN1_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB1 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn usben1(&mut self) -> USBEN1_W<USBEN1_SPEC> {
        USBEN1_W::new(self, 0)
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
#[doc = "USB1 Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usben1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usben1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBEN1_SPEC;
impl crate::RegisterSpec for USBEN1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usben1::R`](R) reader structure"]
impl crate::Readable for USBEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usben1::W`](W) writer structure"]
impl crate::Writable for USBEN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USBEN1 to value 0"]
impl crate::Resettable for USBEN1_SPEC {
    const RESET_VALUE: u8 = 0;
}
