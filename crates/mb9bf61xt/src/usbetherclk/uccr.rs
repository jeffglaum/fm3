#[doc = "Register `UCCR` reader"]
pub type R = crate::R<UCCR_SPEC>;
#[doc = "Register `UCCR` writer"]
pub type W = crate::W<UCCR_SPEC>;
#[doc = "Field `UCEN0` reader - USB0 clock output enable bit"]
pub type UCEN0_R = crate::BitReader;
#[doc = "Field `UCEN0` writer - USB0 clock output enable bit"]
pub type UCEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSEL0` reader - USB0 clock selection bit"]
pub type UCSEL0_R = crate::BitReader;
#[doc = "Field `UCSEL0` writer - USB0 clock selection bit"]
pub type UCSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSEL1` reader - USB1 clock selection bit"]
pub type UCSEL1_R = crate::BitReader;
#[doc = "Field `UCSEL1` writer - USB1 clock selection bit"]
pub type UCSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCEN1` reader - USB1 clock output enable bit"]
pub type UCEN1_R = crate::BitReader;
#[doc = "Field `UCEN1` writer - USB1 clock output enable bit"]
pub type UCEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECEN` reader - Ethernet clock output enable bit"]
pub type ECEN_R = crate::BitReader;
#[doc = "Field `ECEN` writer - Ethernet clock output enable bit"]
pub type ECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECSEL0` reader - Ethernet clock selection bit 0"]
pub type ECSEL0_R = crate::BitReader;
#[doc = "Field `ECSEL0` writer - Ethernet clock selection bit 0"]
pub type ECSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECSEL1` reader - Ethernet clock selection bit 1"]
pub type ECSEL1_R = crate::BitReader;
#[doc = "Field `ECSEL1` writer - Ethernet clock selection bit 1"]
pub type ECSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB0 clock output enable bit"]
    #[inline(always)]
    pub fn ucen0(&self) -> UCEN0_R {
        UCEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB0 clock selection bit"]
    #[inline(always)]
    pub fn ucsel0(&self) -> UCSEL0_R {
        UCSEL0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB1 clock selection bit"]
    #[inline(always)]
    pub fn ucsel1(&self) -> UCSEL1_R {
        UCSEL1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB1 clock output enable bit"]
    #[inline(always)]
    pub fn ucen1(&self) -> UCEN1_R {
        UCEN1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ethernet clock output enable bit"]
    #[inline(always)]
    pub fn ecen(&self) -> ECEN_R {
        ECEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Ethernet clock selection bit 0"]
    #[inline(always)]
    pub fn ecsel0(&self) -> ECSEL0_R {
        ECSEL0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ethernet clock selection bit 1"]
    #[inline(always)]
    pub fn ecsel1(&self) -> ECSEL1_R {
        ECSEL1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB0 clock output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ucen0(&mut self) -> UCEN0_W<UCCR_SPEC> {
        UCEN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - USB0 clock selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn ucsel0(&mut self) -> UCSEL0_W<UCCR_SPEC> {
        UCSEL0_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB1 clock selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn ucsel1(&mut self) -> UCSEL1_W<UCCR_SPEC> {
        UCSEL1_W::new(self, 2)
    }
    #[doc = "Bit 3 - USB1 clock output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ucen1(&mut self) -> UCEN1_W<UCCR_SPEC> {
        UCEN1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Ethernet clock output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ecen(&mut self) -> ECEN_W<UCCR_SPEC> {
        ECEN_W::new(self, 4)
    }
    #[doc = "Bit 6 - Ethernet clock selection bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ecsel0(&mut self) -> ECSEL0_W<UCCR_SPEC> {
        ECSEL0_W::new(self, 6)
    }
    #[doc = "Bit 7 - Ethernet clock selection bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ecsel1(&mut self) -> ECSEL1_W<UCCR_SPEC> {
        ECSEL1_W::new(self, 7)
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
#[doc = "USB/Ethernet-PLL Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCCR_SPEC;
impl crate::RegisterSpec for UCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uccr::R`](R) reader structure"]
impl crate::Readable for UCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uccr::W`](W) writer structure"]
impl crate::Writable for UCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCCR to value 0"]
impl crate::Resettable for UCCR_SPEC {
    const RESET_VALUE: u8 = 0;
}
