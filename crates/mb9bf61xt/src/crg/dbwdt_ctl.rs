#[doc = "Register `DBWDT_CTL` reader"]
pub type R = crate::R<DBWDT_CTL_SPEC>;
#[doc = "Register `DBWDT_CTL` writer"]
pub type W = crate::W<DBWDT_CTL_SPEC>;
#[doc = "Field `DPSWBE` reader - SW-WDG debug mode break bit"]
pub type DPSWBE_R = crate::BitReader;
#[doc = "Field `DPSWBE` writer - SW-WDG debug mode break bit"]
pub type DPSWBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHWBE` reader - HW-WDG debug mode break bit"]
pub type DPHWBE_R = crate::BitReader;
#[doc = "Field `DPHWBE` writer - HW-WDG debug mode break bit"]
pub type DPHWBE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - SW-WDG debug mode break bit"]
    #[inline(always)]
    pub fn dpswbe(&self) -> DPSWBE_R {
        DPSWBE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - HW-WDG debug mode break bit"]
    #[inline(always)]
    pub fn dphwbe(&self) -> DPHWBE_R {
        DPHWBE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - SW-WDG debug mode break bit"]
    #[inline(always)]
    #[must_use]
    pub fn dpswbe(&mut self) -> DPSWBE_W<DBWDT_CTL_SPEC> {
        DPSWBE_W::new(self, 5)
    }
    #[doc = "Bit 7 - HW-WDG debug mode break bit"]
    #[inline(always)]
    #[must_use]
    pub fn dphwbe(&mut self) -> DPHWBE_W<DBWDT_CTL_SPEC> {
        DPHWBE_W::new(self, 7)
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
#[doc = "Debug Break Watchdog Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbwdt_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbwdt_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBWDT_CTL_SPEC;
impl crate::RegisterSpec for DBWDT_CTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dbwdt_ctl::R`](R) reader structure"]
impl crate::Readable for DBWDT_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbwdt_ctl::W`](W) writer structure"]
impl crate::Writable for DBWDT_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DBWDT_CTL to value 0"]
impl crate::Resettable for DBWDT_CTL_SPEC {
    const RESET_VALUE: u8 = 0;
}
