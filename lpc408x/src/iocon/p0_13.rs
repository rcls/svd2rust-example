#[doc = "Register `P0_13` reader"]
pub struct R(crate::R<P0_13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P0_13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P0_13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P0_13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P0_13` writer"]
pub struct W(crate::W<P0_13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P0_13_SPEC>;
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
impl From<crate::W<P0_13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P0_13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects pin function for pin P0\\[13\\]"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output pin."]
    P0_13 = 0,
    #[doc = "1: USB port 2 GoodLink LED indicator. It is LOW when the                                     device is configured (non-control endpoints enabled), or when                                     the host is enabled and has detected a device on the bus. It is                                     HIGH when the device is not configured, or when host is enabled                                     and has not detected a device on the bus, or during global                                     suspend. It transitions between LOW and HIGH (flashes) when the                                     host is enabled and detects activity on the bus."]
    USB_UP_LED2 = 1,
    #[doc = "2: Master Out Slave In for SSP1."]
    SSP1_MOSI = 2,
    #[doc = "3: A/D converter 0, input 7. When configured as an ADC                                     input, the digital function of the pin must be                                     disabled."]
    ADC0_IN_7 = 3,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNC` reader - Selects pin function for pin P0\\[13\\]"]
pub struct FUNC_R(crate::FieldReader<u8, FUNC_A>);
impl FUNC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::P0_13),
            1 => Some(FUNC_A::USB_UP_LED2),
            2 => Some(FUNC_A::SSP1_MOSI),
            3 => Some(FUNC_A::ADC0_IN_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P0_13`"]
    #[inline(always)]
    pub fn is_p0_13(&self) -> bool {
        **self == FUNC_A::P0_13
    }
    #[doc = "Checks if the value of the field is `USB_UP_LED2`"]
    #[inline(always)]
    pub fn is_usb_up_led2(&self) -> bool {
        **self == FUNC_A::USB_UP_LED2
    }
    #[doc = "Checks if the value of the field is `SSP1_MOSI`"]
    #[inline(always)]
    pub fn is_ssp1_mosi(&self) -> bool {
        **self == FUNC_A::SSP1_MOSI
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_7`"]
    #[inline(always)]
    pub fn is_adc0_in_7(&self) -> bool {
        **self == FUNC_A::ADC0_IN_7
    }
}
impl core::ops::Deref for FUNC_R {
    type Target = crate::FieldReader<u8, FUNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P0\\[13\\]"]
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
    pub fn p0_13(self) -> &'a mut W {
        self.variant(FUNC_A::P0_13)
    }
    #[doc = "USB port 2 GoodLink LED indicator. It is LOW when the device is configured (non-control endpoints enabled), or when the host is enabled and has detected a device on the bus. It is HIGH when the device is not configured, or when host is enabled and has not detected a device on the bus, or during global suspend. It transitions between LOW and HIGH (flashes) when the host is enabled and detects activity on the bus."]
    #[inline(always)]
    pub fn usb_up_led2(self) -> &'a mut W {
        self.variant(FUNC_A::USB_UP_LED2)
    }
    #[doc = "Master Out Slave In for SSP1."]
    #[inline(always)]
    pub fn ssp1_mosi(self) -> &'a mut W {
        self.variant(FUNC_A::SSP1_MOSI)
    }
    #[doc = "A/D converter 0, input 7. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_7(self) -> &'a mut W {
        self.variant(FUNC_A::ADC0_IN_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[13\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[13\\]"]
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
#[doc = "I/O configuration register for pin P0\\[13\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p0_13](index.html) module"]
pub struct P0_13_SPEC;
impl crate::RegisterSpec for P0_13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [p0_13::R](R) reader structure"]
impl crate::Readable for P0_13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p0_13::W](W) writer structure"]
impl crate::Writable for P0_13_SPEC {
    type Writer = W;
}
