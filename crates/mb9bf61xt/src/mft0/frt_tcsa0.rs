#[doc = "Register `FRT_TCSA0` reader"]
pub type R = crate::R<FRT_TCSA0_SPEC>;
#[doc = "Register `FRT_TCSA0` writer"]
pub type W = crate::W<FRT_TCSA0_SPEC>;
#[doc = "Field `CLK` reader - FRT clock cycle"]
pub type CLK_R = crate::FieldReader;
#[doc = "Field `CLK` writer - FRT clock cycle"]
pub type CLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCLR` writer - FRT operation state initialization request"]
pub type SCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - FRT's count mode"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - FRT's count mode"]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Puts FRT in stopping state"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - Puts FRT in stopping state"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFE` reader - Enables TCCP's buffer function"]
pub type BFE_R = crate::BitReader;
#[doc = "Field `BFE` writer - Enables TCCP's buffer function"]
pub type BFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICRE` reader - \"Generates interrupt when \"\"1\"\" is set to TCSA.ICLR\""]
pub type ICRE_R = crate::BitReader;
#[doc = "Field `ICRE` writer - \"Generates interrupt when \"\"1\"\" is set to TCSA.ICLR\""]
pub type ICRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICLR` reader - interrupt flag"]
pub type ICLR_R = crate::BitReader;
#[doc = "Field `ICLR` writer - interrupt flag"]
pub type ICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQZE` reader - \"Generates interrupt, when \"\"1\"\" is set to TCSA.IRQZF\""]
pub type IRQZE_R = crate::BitReader;
#[doc = "Field `IRQZE` writer - \"Generates interrupt, when \"\"1\"\" is set to TCSA.IRQZF\""]
pub type IRQZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQZF` reader - zero interrupt flag"]
pub type IRQZF_R = crate::BitReader;
#[doc = "Field `IRQZF` writer - zero interrupt flag"]
pub type IRQZF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECKE` reader - Uses an external input clock (FRCK) as FRT's count clock"]
pub type ECKE_R = crate::BitReader;
#[doc = "Field `ECKE` writer - Uses an external input clock (FRCK) as FRT's count clock"]
pub type ECKE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - FRT clock cycle"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - FRT's count mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Puts FRT in stopping state"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables TCCP's buffer function"]
    #[inline(always)]
    pub fn bfe(&self) -> BFE_R {
        BFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - \"Generates interrupt when \"\"1\"\" is set to TCSA.ICLR\""]
    #[inline(always)]
    pub fn icre(&self) -> ICRE_R {
        ICRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - interrupt flag"]
    #[inline(always)]
    pub fn iclr(&self) -> ICLR_R {
        ICLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - \"Generates interrupt, when \"\"1\"\" is set to TCSA.IRQZF\""]
    #[inline(always)]
    pub fn irqze(&self) -> IRQZE_R {
        IRQZE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - zero interrupt flag"]
    #[inline(always)]
    pub fn irqzf(&self) -> IRQZF_R {
        IRQZF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Uses an external input clock (FRCK) as FRT's count clock"]
    #[inline(always)]
    pub fn ecke(&self) -> ECKE_R {
        ECKE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FRT clock cycle"]
    #[inline(always)]
    #[must_use]
    pub fn clk(&mut self) -> CLK_W<FRT_TCSA0_SPEC> {
        CLK_W::new(self, 0)
    }
    #[doc = "Bit 4 - FRT operation state initialization request"]
    #[inline(always)]
    #[must_use]
    pub fn sclr(&mut self) -> SCLR_W<FRT_TCSA0_SPEC> {
        SCLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - FRT's count mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<FRT_TCSA0_SPEC> {
        MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Puts FRT in stopping state"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<FRT_TCSA0_SPEC> {
        STOP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enables TCCP's buffer function"]
    #[inline(always)]
    #[must_use]
    pub fn bfe(&mut self) -> BFE_W<FRT_TCSA0_SPEC> {
        BFE_W::new(self, 7)
    }
    #[doc = "Bit 8 - \"Generates interrupt when \"\"1\"\" is set to TCSA.ICLR\""]
    #[inline(always)]
    #[must_use]
    pub fn icre(&mut self) -> ICRE_W<FRT_TCSA0_SPEC> {
        ICRE_W::new(self, 8)
    }
    #[doc = "Bit 9 - interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn iclr(&mut self) -> ICLR_W<FRT_TCSA0_SPEC> {
        ICLR_W::new(self, 9)
    }
    #[doc = "Bit 13 - \"Generates interrupt, when \"\"1\"\" is set to TCSA.IRQZF\""]
    #[inline(always)]
    #[must_use]
    pub fn irqze(&mut self) -> IRQZE_W<FRT_TCSA0_SPEC> {
        IRQZE_W::new(self, 13)
    }
    #[doc = "Bit 14 - zero interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn irqzf(&mut self) -> IRQZF_W<FRT_TCSA0_SPEC> {
        IRQZF_W::new(self, 14)
    }
    #[doc = "Bit 15 - Uses an external input clock (FRCK) as FRT's count clock"]
    #[inline(always)]
    #[must_use]
    pub fn ecke(&mut self) -> ECKE_W<FRT_TCSA0_SPEC> {
        ECKE_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FRT-ch.0 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frt_tcsa0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frt_tcsa0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRT_TCSA0_SPEC;
impl crate::RegisterSpec for FRT_TCSA0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`frt_tcsa0::R`](R) reader structure"]
impl crate::Readable for FRT_TCSA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frt_tcsa0::W`](W) writer structure"]
impl crate::Writable for FRT_TCSA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FRT_TCSA0 to value 0x40"]
impl crate::Resettable for FRT_TCSA0_SPEC {
    const RESET_VALUE: u16 = 0x40;
}
