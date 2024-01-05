#[doc = "Register `ETH_MODE` reader"]
pub type R = crate::R<ETH_MODE_SPEC>;
#[doc = "Register `ETH_MODE` writer"]
pub type W = crate::W<ETH_MODE_SPEC>;
#[doc = "Field `IFMODE` reader - Mode selector"]
pub type IFMODE_R = crate::BitReader;
#[doc = "Field `IFMODE` writer - Mode selector"]
pub type IFMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST0` reader - reset signal against Ethernet-MAC (ch.0)"]
pub type RST0_R = crate::BitReader;
#[doc = "Field `RST0` writer - reset signal against Ethernet-MAC (ch.0)"]
pub type RST0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST1` reader - reset signal against Ethernet-MAC (ch.1)"]
pub type RST1_R = crate::BitReader;
#[doc = "Field `RST1` writer - reset signal against Ethernet-MAC (ch.1)"]
pub type RST1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPSSEL` reader - Select either of the system time counter pulse outputs of the Ethernet-MAC PTP function to output to E_PPS0_PPS1 pin"]
pub type PPSSEL_R = crate::BitReader;
#[doc = "Field `PPSSEL` writer - Select either of the system time counter pulse outputs of the Ethernet-MAC PTP function to output to E_PPS0_PPS1 pin"]
pub type PPSSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mode selector"]
    #[inline(always)]
    pub fn ifmode(&self) -> IFMODE_R {
        IFMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - reset signal against Ethernet-MAC (ch.0)"]
    #[inline(always)]
    pub fn rst0(&self) -> RST0_R {
        RST0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reset signal against Ethernet-MAC (ch.1)"]
    #[inline(always)]
    pub fn rst1(&self) -> RST1_R {
        RST1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 28 - Select either of the system time counter pulse outputs of the Ethernet-MAC PTP function to output to E_PPS0_PPS1 pin"]
    #[inline(always)]
    pub fn ppssel(&self) -> PPSSEL_R {
        PPSSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode selector"]
    #[inline(always)]
    #[must_use]
    pub fn ifmode(&mut self) -> IFMODE_W<ETH_MODE_SPEC> {
        IFMODE_W::new(self, 0)
    }
    #[doc = "Bit 8 - reset signal against Ethernet-MAC (ch.0)"]
    #[inline(always)]
    #[must_use]
    pub fn rst0(&mut self) -> RST0_W<ETH_MODE_SPEC> {
        RST0_W::new(self, 8)
    }
    #[doc = "Bit 9 - reset signal against Ethernet-MAC (ch.1)"]
    #[inline(always)]
    #[must_use]
    pub fn rst1(&mut self) -> RST1_W<ETH_MODE_SPEC> {
        RST1_W::new(self, 9)
    }
    #[doc = "Bit 28 - Select either of the system time counter pulse outputs of the Ethernet-MAC PTP function to output to E_PPS0_PPS1 pin"]
    #[inline(always)]
    #[must_use]
    pub fn ppssel(&mut self) -> PPSSEL_W<ETH_MODE_SPEC> {
        PPSSEL_W::new(self, 28)
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
#[doc = "Mode Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MODE_SPEC;
impl crate::RegisterSpec for ETH_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mode::R`](R) reader structure"]
impl crate::Readable for ETH_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eth_mode::W`](W) writer structure"]
impl crate::Writable for ETH_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MODE to value 0"]
impl crate::Resettable for ETH_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
