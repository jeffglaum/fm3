#[doc = "Register `UART_RDR` reader"]
pub type R = crate::R<UART_UART_RDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<UART_UART_RDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Received Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_rdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_UART_RDR_SPEC;
impl crate::RegisterSpec for UART_UART_RDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uart_uart_rdr::R`](R) reader structure"]
impl crate::Readable for UART_UART_RDR_SPEC {}
#[doc = "`reset()` method sets UART_RDR to value 0"]
impl crate::Resettable for UART_UART_RDR_SPEC {
    const RESET_VALUE: u16 = 0;
}
