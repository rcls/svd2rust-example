#[doc = "Register `P0_24` reader"]
pub struct R(crate::R<P0_24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P0_24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P0_24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P0_24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P0_24` writer"]
pub struct W(crate::W<P0_24_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P0_24_SPEC>;
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
impl From<crate::W<P0_24_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P0_24_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects pin function for pin P0\\[24\\]"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output pin."]
    P0_24 = 0,
    #[doc = "1: A/D converter 0, input 1. When configured as an ADC                                     input, the digital function of the pin must be                                     disabled."]
    ADC0_IN_1 = 1,
    #[doc = "2: Receive Word Select. It is driven by the master and                                     received by the slave. Corresponds to the signal WS in the                                             I2S-bus                                         specification."]
    I2S_RX_WS = 2,
    #[doc = "3: Capture input for Timer 3, channel 1."]
    T3_CAP1 = 3,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNC` reader - Selects pin function for pin P0\\[24\\]"]
pub struct FUNC_R(crate::FieldReader<u8, FUNC_A>);
impl FUNC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::P0_24),
            1 => Some(FUNC_A::ADC0_IN_1),
            2 => Some(FUNC_A::I2S_RX_WS),
            3 => Some(FUNC_A::T3_CAP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P0_24`"]
    #[inline(always)]
    pub fn is_p0_24(&self) -> bool {
        **self == FUNC_A::P0_24
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_1`"]
    #[inline(always)]
    pub fn is_adc0_in_1(&self) -> bool {
        **self == FUNC_A::ADC0_IN_1
    }
    #[doc = "Checks if the value of the field is `I2S_RX_WS`"]
    #[inline(always)]
    pub fn is_i2s_rx_ws(&self) -> bool {
        **self == FUNC_A::I2S_RX_WS
    }
    #[doc = "Checks if the value of the field is `T3_CAP1`"]
    #[inline(always)]
    pub fn is_t3_cap1(&self) -> bool {
        **self == FUNC_A::T3_CAP1
    }
}
impl core::ops::Deref for FUNC_R {
    type Target = crate::FieldReader<u8, FUNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P0\\[24\\]"]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p0_24(self) -> &'a mut W {
        self.variant(FUNC_A::P0_24)
    }
    #[doc = "A/D converter 0, input 1. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_1(self) -> &'a mut W {
        self.variant(FUNC_A::ADC0_IN_1)
    }
    #[doc = "Receive Word Select. It is driven by the master and received by the slave. Corresponds to the signal WS in the I2S-bus specification."]
    #[inline(always)]
    pub fn i2s_rx_ws(self) -> &'a mut W {
        self.variant(FUNC_A::I2S_RX_WS)
    }
    #[doc = "Capture input for Timer 3, channel 1."]
    #[inline(always)]
    pub fn t3_cap1(self) -> &'a mut W {
        self.variant(FUNC_A::T3_CAP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[24\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[24\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O configuration register for pin P0\\[24\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p0_24](index.html) module"]
pub struct P0_24_SPEC;
impl crate::RegisterSpec for P0_24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [p0_24::R](R) reader structure"]
impl crate::Readable for P0_24_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p0_24::W](W) writer structure"]
impl crate::Writable for P0_24_SPEC {
    type Writer = W;
}
