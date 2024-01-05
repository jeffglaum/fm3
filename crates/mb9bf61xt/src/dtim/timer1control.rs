#[doc = "Register `TIMER1CONTROL` reader"]
pub type R = crate::R<TIMER1CONTROL_SPEC>;
#[doc = "Register `TIMER1CONTROL` writer"]
pub type W = crate::W<TIMER1CONTROL_SPEC>;
#[doc = "Field `OneShot` reader - One-shot mode bit"]
pub type ONE_SHOT_R = crate::BitReader;
#[doc = "Field `OneShot` writer - One-shot mode bit"]
pub type ONE_SHOT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TimerSize` reader - Counter size bit"]
pub type TIMER_SIZE_R = crate::BitReader;
#[doc = "Field `TimerSize` writer - Counter size bit"]
pub type TIMER_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TimerPre` reader - Prescale bits"]
pub type TIMER_PRE_R = crate::FieldReader;
#[doc = "Field `TimerPre` writer - Prescale bits"]
pub type TIMER_PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IntEnable` reader - Interrupt enable bit"]
pub type INT_ENABLE_R = crate::BitReader;
#[doc = "Field `IntEnable` writer - Interrupt enable bit"]
pub type INT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TimerMode` reader - Mode bit"]
pub type TIMER_MODE_R = crate::BitReader;
#[doc = "Field `TimerMode` writer - Mode bit"]
pub type TIMER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TimerEn` reader - Enable bit"]
pub type TIMER_EN_R = crate::BitReader;
#[doc = "Field `TimerEn` writer - Enable bit"]
pub type TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - One-shot mode bit"]
    #[inline(always)]
    pub fn one_shot(&self) -> ONE_SHOT_R {
        ONE_SHOT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter size bit"]
    #[inline(always)]
    pub fn timer_size(&self) -> TIMER_SIZE_R {
        TIMER_SIZE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Prescale bits"]
    #[inline(always)]
    pub fn timer_pre(&self) -> TIMER_PRE_R {
        TIMER_PRE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable bit"]
    #[inline(always)]
    pub fn int_enable(&self) -> INT_ENABLE_R {
        INT_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mode bit"]
    #[inline(always)]
    pub fn timer_mode(&self) -> TIMER_MODE_R {
        TIMER_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable bit"]
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - One-shot mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn one_shot(&mut self) -> ONE_SHOT_W<TIMER1CONTROL_SPEC> {
        ONE_SHOT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter size bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer_size(&mut self) -> TIMER_SIZE_W<TIMER1CONTROL_SPEC> {
        TIMER_SIZE_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Prescale bits"]
    #[inline(always)]
    #[must_use]
    pub fn timer_pre(&mut self) -> TIMER_PRE_W<TIMER1CONTROL_SPEC> {
        TIMER_PRE_W::new(self, 2)
    }
    #[doc = "Bit 5 - Interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn int_enable(&mut self) -> INT_ENABLE_W<TIMER1CONTROL_SPEC> {
        INT_ENABLE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mode(&mut self) -> TIMER_MODE_W<TIMER1CONTROL_SPEC> {
        TIMER_MODE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer_en(&mut self) -> TIMER_EN_W<TIMER1CONTROL_SPEC> {
        TIMER_EN_W::new(self, 7)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER1CONTROL_SPEC;
impl crate::RegisterSpec for TIMER1CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1control::R`](R) reader structure"]
impl crate::Readable for TIMER1CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer1control::W`](W) writer structure"]
impl crate::Writable for TIMER1CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER1CONTROL to value 0x20"]
impl crate::Resettable for TIMER1CONTROL_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
