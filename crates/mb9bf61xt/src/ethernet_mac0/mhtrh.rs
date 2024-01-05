#[doc = "Register `MHTRH` reader"]
pub type R = crate::R<MHTRH_SPEC>;
#[doc = "Register `MHTRH` writer"]
pub type W = crate::W<MHTRH_SPEC>;
#[doc = "Field `HTH` reader - the upper 32 bits of the hash table in the HTH"]
pub type HTH_R = crate::FieldReader<u32>;
#[doc = "Field `HTH` writer - the upper 32 bits of the hash table in the HTH"]
pub type HTH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the upper 32 bits of the hash table in the HTH"]
    #[inline(always)]
    pub fn hth(&self) -> HTH_R {
        HTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the upper 32 bits of the hash table in the HTH"]
    #[inline(always)]
    #[must_use]
    pub fn hth(&mut self) -> HTH_W<MHTRH_SPEC> {
        HTH_W::new(self, 0)
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
#[doc = "MAC Hash Table Register (High)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mhtrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mhtrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHTRH_SPEC;
impl crate::RegisterSpec for MHTRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhtrh::R`](R) reader structure"]
impl crate::Readable for MHTRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mhtrh::W`](W) writer structure"]
impl crate::Writable for MHTRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MHTRH to value 0"]
impl crate::Resettable for MHTRH_SPEC {
    const RESET_VALUE: u32 = 0;
}
