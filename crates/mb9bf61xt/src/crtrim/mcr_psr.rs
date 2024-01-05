#[doc = "Register `MCR_PSR` reader"]
pub type R = crate::R<MCR_PSR_SPEC>;
#[doc = "Register `MCR_PSR` writer"]
pub type W = crate::W<MCR_PSR_SPEC>;
#[doc = "Field `CSR` reader - High-speed CR oscillation frequency division ratio setting bits"]
pub type CSR_R = crate::FieldReader;
#[doc = "Field `CSR` writer - High-speed CR oscillation frequency division ratio setting bits"]
pub type CSR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - High-speed CR oscillation frequency division ratio setting bits"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - High-speed CR oscillation frequency division ratio setting bits"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<MCR_PSR_SPEC> {
        CSR_W::new(self, 0)
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
#[doc = "High-speed CR oscillation Frequency Division Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr_psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr_psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCR_PSR_SPEC;
impl crate::RegisterSpec for MCR_PSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mcr_psr::R`](R) reader structure"]
impl crate::Readable for MCR_PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcr_psr::W`](W) writer structure"]
impl crate::Writable for MCR_PSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MCR_PSR to value 0x01"]
impl crate::Resettable for MCR_PSR_SPEC {
    const RESET_VALUE: u8 = 0x01;
}
