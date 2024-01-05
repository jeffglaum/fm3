#[doc = "Register `LVD_CTL` reader"]
pub type R = crate::R<LVD_CTL_SPEC>;
#[doc = "Register `LVD_CTL` writer"]
pub type W = crate::W<LVD_CTL_SPEC>;
#[doc = "Field `SVHI` reader - Low-voltage detection interrupt voltage setting bits"]
pub type SVHI_R = crate::FieldReader;
#[doc = "Field `SVHI` writer - Low-voltage detection interrupt voltage setting bits"]
pub type SVHI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LVDIE` reader - Low-voltage detection interrupt enable bit"]
pub type LVDIE_R = crate::BitReader;
#[doc = "Field `LVDIE` writer - Low-voltage detection interrupt enable bit"]
pub type LVDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:5 - Low-voltage detection interrupt voltage setting bits"]
    #[inline(always)]
    pub fn svhi(&self) -> SVHI_R {
        SVHI_R::new((self.bits >> 2) & 0x0f)
    }
    #[doc = "Bit 7 - Low-voltage detection interrupt enable bit"]
    #[inline(always)]
    pub fn lvdie(&self) -> LVDIE_R {
        LVDIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:5 - Low-voltage detection interrupt voltage setting bits"]
    #[inline(always)]
    #[must_use]
    pub fn svhi(&mut self) -> SVHI_W<LVD_CTL_SPEC> {
        SVHI_W::new(self, 2)
    }
    #[doc = "Bit 7 - Low-voltage detection interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lvdie(&mut self) -> LVDIE_W<LVD_CTL_SPEC> {
        LVDIE_W::new(self, 7)
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
#[doc = "Low-voltage Detection Voltage Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvd_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvd_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVD_CTL_SPEC;
impl crate::RegisterSpec for LVD_CTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvd_ctl::R`](R) reader structure"]
impl crate::Readable for LVD_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lvd_ctl::W`](W) writer structure"]
impl crate::Writable for LVD_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LVD_CTL to value 0x40"]
impl crate::Resettable for LVD_CTL_SPEC {
    const RESET_VALUE: u8 = 0x40;
}
