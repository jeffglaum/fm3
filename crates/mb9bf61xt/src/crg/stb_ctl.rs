#[doc = "Register `STB_CTL` reader"]
pub type R = crate::R<STB_CTL_SPEC>;
#[doc = "Register `STB_CTL` writer"]
pub type W = crate::W<STB_CTL_SPEC>;
#[doc = "Field `STM` reader - Standby mode selection bit"]
pub type STM_R = crate::FieldReader;
#[doc = "Field `STM` writer - Standby mode selection bit"]
pub type STM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPL` reader - Standby pin level setting bit"]
pub type SPL_R = crate::BitReader;
#[doc = "Field `SPL` writer - Standby pin level setting bit"]
pub type SPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - Standby mode control write control bit"]
pub type KEY_R = crate::FieldReader<u16>;
#[doc = "Field `KEY` writer - Standby mode control write control bit"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - Standby mode selection bit"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Standby pin level setting bit"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Standby mode control write control bit"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Standby mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn stm(&mut self) -> STM_W<STB_CTL_SPEC> {
        STM_W::new(self, 0)
    }
    #[doc = "Bit 4 - Standby pin level setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn spl(&mut self) -> SPL_W<STB_CTL_SPEC> {
        SPL_W::new(self, 4)
    }
    #[doc = "Bits 16:31 - Standby mode control write control bit"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<STB_CTL_SPEC> {
        KEY_W::new(self, 16)
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
#[doc = "Standby Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stb_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stb_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STB_CTL_SPEC;
impl crate::RegisterSpec for STB_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stb_ctl::R`](R) reader structure"]
impl crate::Readable for STB_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stb_ctl::W`](W) writer structure"]
impl crate::Writable for STB_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STB_CTL to value 0"]
impl crate::Resettable for STB_CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
