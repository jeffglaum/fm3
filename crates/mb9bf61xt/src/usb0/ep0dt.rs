#[doc = "Register `EP0DT` reader"]
pub type R = crate::R<EP0DT_SPEC>;
#[doc = "Register `EP0DT` writer"]
pub type W = crate::W<EP0DT_SPEC>;
#[doc = "Field `BFDT` reader - Endpoint Send/Receive Buffer Data"]
pub type BFDT_R = crate::FieldReader<u16>;
#[doc = "Field `BFDT` writer - Endpoint Send/Receive Buffer Data"]
pub type BFDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Endpoint Send/Receive Buffer Data"]
    #[inline(always)]
    pub fn bfdt(&self) -> BFDT_R {
        BFDT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint Send/Receive Buffer Data"]
    #[inline(always)]
    #[must_use]
    pub fn bfdt(&mut self) -> BFDT_W<EP0DT_SPEC> {
        BFDT_W::new(self, 0)
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
#[doc = "EP0 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP0DT_SPEC;
impl crate::RegisterSpec for EP0DT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep0dt::R`](R) reader structure"]
impl crate::Readable for EP0DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep0dt::W`](W) writer structure"]
impl crate::Writable for EP0DT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EP0DT to value 0"]
impl crate::Resettable for EP0DT_SPEC {
    const RESET_VALUE: u16 = 0;
}
