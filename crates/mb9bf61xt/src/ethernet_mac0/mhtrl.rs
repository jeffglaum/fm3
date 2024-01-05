#[doc = "Register `MHTRL` reader"]
pub type R = crate::R<MHTRL_SPEC>;
#[doc = "Register `MHTRL` writer"]
pub type W = crate::W<MHTRL_SPEC>;
#[doc = "Field `HTL` reader - the lower 32 bits of the hash table in the HTL"]
pub type HTL_R = crate::FieldReader<u32>;
#[doc = "Field `HTL` writer - the lower 32 bits of the hash table in the HTL"]
pub type HTL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the lower 32 bits of the hash table in the HTL"]
    #[inline(always)]
    pub fn htl(&self) -> HTL_R {
        HTL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the lower 32 bits of the hash table in the HTL"]
    #[inline(always)]
    #[must_use]
    pub fn htl(&mut self) -> HTL_W<MHTRL_SPEC> {
        HTL_W::new(self, 0)
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
#[doc = "MAC Hash Table Register (Low)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mhtrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mhtrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHTRL_SPEC;
impl crate::RegisterSpec for MHTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhtrl::R`](R) reader structure"]
impl crate::Readable for MHTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mhtrl::W`](W) writer structure"]
impl crate::Writable for MHTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MHTRL to value 0"]
impl crate::Resettable for MHTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
