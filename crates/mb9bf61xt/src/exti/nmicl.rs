#[doc = "Register `NMICL` reader"]
pub type R = crate::R<NMICL_SPEC>;
#[doc = "Register `NMICL` writer"]
pub type W = crate::W<NMICL_SPEC>;
#[doc = "Field `NCL` reader - NMI interrupt cause clear bit"]
pub type NCL_R = crate::BitReader;
#[doc = "Field `NCL` writer - NMI interrupt cause clear bit"]
pub type NCL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NMI interrupt cause clear bit"]
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI interrupt cause clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn ncl(&mut self) -> NCL_W<NMICL_SPEC> {
        NCL_W::new(self, 0)
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
#[doc = "Non Maskable Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmicl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmicl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NMICL_SPEC;
impl crate::RegisterSpec for NMICL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`nmicl::R`](R) reader structure"]
impl crate::Readable for NMICL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nmicl::W`](W) writer structure"]
impl crate::Writable for NMICL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets NMICL to value 0x01"]
impl crate::Resettable for NMICL_SPEC {
    const RESET_VALUE: u8 = 0x01;
}
