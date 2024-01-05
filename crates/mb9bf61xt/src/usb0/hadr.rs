#[doc = "Register `HADR` reader"]
pub type R = crate::R<HADR_SPEC>;
#[doc = "Register `HADR` writer"]
pub type W = crate::W<HADR_SPEC>;
#[doc = "Field `ADDRESS` reader - Host Address"]
pub type ADDRESS_R = crate::FieldReader;
#[doc = "Field `ADDRESS` writer - Host Address"]
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Host Address"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<HADR_SPEC> {
        ADDRESS_W::new(self, 0)
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
#[doc = "Host Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hadr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hadr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HADR_SPEC;
impl crate::RegisterSpec for HADR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hadr::R`](R) reader structure"]
impl crate::Readable for HADR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hadr::W`](W) writer structure"]
impl crate::Writable for HADR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HADR to value 0"]
impl crate::Resettable for HADR_SPEC {
    const RESET_VALUE: u8 = 0;
}
