#[doc = "Register `P1_31` reader"]
pub struct R(crate::R<P1_31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1_31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1_31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1_31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1_31` writer"]
pub struct W(crate::W<P1_31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1_31_SPEC>;
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
impl From<crate::W<P1_31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1_31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects pin function for pin P1\\[31\\]"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output pin."]
    P1_31 = 0,
    #[doc = "1: Over-Current status for USB port 2."]
    USB_OVRCR2 = 1,
    #[doc = "2: Serial Clock for SSP1."]
    SSP1_SCK = 2,
    #[doc = "3: A/D converter 0, input 5. When configured as an ADC                                     input, the digital function of the pin must be                                     disabled."]
    ADC0_IN_5 = 3,
    #[doc = "4: I2C0 clock input/output (this                                     pin does not use a specialized I2C pad."]
    I2C0_SCL = 4,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNC` reader - Selects pin function for pin P1\\[31\\]"]
pub struct FUNC_R(crate::FieldReader<u8, FUNC_A>);
impl FUNC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::P1_31),
            1 => Some(FUNC_A::USB_OVRCR2),
            2 => Some(FUNC_A::SSP1_SCK),
            3 => Some(FUNC_A::ADC0_IN_5),
            4 => Some(FUNC_A::I2C0_SCL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P1_31`"]
    #[inline(always)]
    pub fn is_p1_31(&self) -> bool {
        **self == FUNC_A::P1_31
    }
    #[doc = "Checks if the value of the field is `USB_OVRCR2`"]
    #[inline(always)]
    pub fn is_usb_ovrcr2(&self) -> bool {
        **self == FUNC_A::USB_OVRCR2
    }
    #[doc = "Checks if the value of the field is `SSP1_SCK`"]
    #[inline(always)]
    pub fn is_ssp1_sck(&self) -> bool {
        **self == FUNC_A::SSP1_SCK
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_5`"]
    #[inline(always)]
    pub fn is_adc0_in_5(&self) -> bool {
        **self == FUNC_A::ADC0_IN_5
    }
    #[doc = "Checks if the value of the field is `I2C0_SCL`"]
    #[inline(always)]
    pub fn is_i2c0_scl(&self) -> bool {
        **self == FUNC_A::I2C0_SCL
    }
}
impl core::ops::Deref for FUNC_R {
    type Target = crate::FieldReader<u8, FUNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P1\\[31\\]"]
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
    pub fn p1_31(self) -> &'a mut W {
        self.variant(FUNC_A::P1_31)
    }
    #[doc = "Over-Current status for USB port 2."]
    #[inline(always)]
    pub fn usb_ovrcr2(self) -> &'a mut W {
        self.variant(FUNC_A::USB_OVRCR2)
    }
    #[doc = "Serial Clock for SSP1."]
    #[inline(always)]
    pub fn ssp1_sck(self) -> &'a mut W {
        self.variant(FUNC_A::SSP1_SCK)
    }
    #[doc = "A/D converter 0, input 5. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_5(self) -> &'a mut W {
        self.variant(FUNC_A::ADC0_IN_5)
    }
    #[doc = "I2C0 clock input/output (this pin does not use a specialized I2C pad."]
    #[inline(always)]
    pub fn i2c0_scl(self) -> &'a mut W {
        self.variant(FUNC_A::I2C0_SCL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P1\\[31\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P1\\[31\\]"]
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
#[doc = "I/O configuration register for pin P1\\[31\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1_31](index.html) module"]
pub struct P1_31_SPEC;
impl crate::RegisterSpec for P1_31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [p1_31::R](R) reader structure"]
impl crate::Readable for P1_31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1_31::W](W) writer structure"]
impl crate::Writable for P1_31_SPEC {
    type Writer = W;
}
