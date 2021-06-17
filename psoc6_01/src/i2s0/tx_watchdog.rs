#[doc = "Register `TX_WATCHDOG` reader"]
pub struct R(crate::R<TX_WATCHDOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_WATCHDOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_WATCHDOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_WATCHDOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_WATCHDOG` writer"]
pub struct W(crate::W<TX_WATCHDOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_WATCHDOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TX_WATCHDOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_WATCHDOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WD_COUNTER` reader - Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
pub struct WD_COUNTER_R(crate::FieldReader<u32, u32>);
impl WD_COUNTER_R {
    pub(crate) fn new(bits: u32) -> Self {
        WD_COUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WD_COUNTER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WD_COUNTER` writer - Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
pub struct WD_COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> WD_COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    pub fn wd_counter(&self) -> WD_COUNTER_R {
        WD_COUNTER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    pub fn wd_counter(&mut self) -> WD_COUNTER_W {
        WD_COUNTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter watchdog\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_watchdog](index.html) module"]
pub struct TX_WATCHDOG_SPEC;
impl crate::RegisterSpec for TX_WATCHDOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_watchdog::R](R) reader structure"]
impl crate::Readable for TX_WATCHDOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_watchdog::W](W) writer structure"]
impl crate::Writable for TX_WATCHDOG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_WATCHDOG to value 0"]
impl crate::Resettable for TX_WATCHDOG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
