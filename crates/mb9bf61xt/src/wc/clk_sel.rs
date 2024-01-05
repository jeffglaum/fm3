#[doc = "Register `CLK_SEL` reader"]
pub type R = crate::R<CLK_SEL_SPEC>;
#[doc = "Register `CLK_SEL` writer"]
pub type W = crate::W<CLK_SEL_SPEC>;
#[doc = "Field `SEL_IN` reader - Input clock selection bit"]
pub type SEL_IN_R = crate::BitReader;
#[doc = "Field `SEL_IN` writer - Input clock selection bit"]
pub type SEL_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_OUT` reader - Output clock selection bit"]
pub type SEL_OUT_R = crate::BitReader;
#[doc = "Field `SEL_OUT` writer - Output clock selection bit"]
pub type SEL_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Input clock selection bit"]
    #[inline(always)]
    pub fn sel_in(&self) -> SEL_IN_R {
        SEL_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Output clock selection bit"]
    #[inline(always)]
    pub fn sel_out(&self) -> SEL_OUT_R {
        SEL_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input clock selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn sel_in(&mut self) -> SEL_IN_W<CLK_SEL_SPEC> {
        SEL_IN_W::new(self, 0)
    }
    #[doc = "Bit 8 - Output clock selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn sel_out(&mut self) -> SEL_OUT_W<CLK_SEL_SPEC> {
        SEL_OUT_W::new(self, 8)
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
#[doc = "Clock Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_SEL_SPEC;
impl crate::RegisterSpec for CLK_SEL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`clk_sel::R`](R) reader structure"]
impl crate::Readable for CLK_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_sel::W`](W) writer structure"]
impl crate::Writable for CLK_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CLK_SEL to value 0"]
impl crate::Resettable for CLK_SEL_SPEC {
    const RESET_VALUE: u16 = 0;
}
