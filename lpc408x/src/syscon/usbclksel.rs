#[doc = "Register `USBCLKSEL` reader"]
pub struct R(crate::R<USBCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCLKSEL` writer"]
pub struct W(crate::W<USBCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCLKSEL_SPEC>;
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
impl From<crate::W<USBCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBDIV_A {
    #[doc = "0: The divider is turned off, no clock will be provided to the USB subsystem."]
    THE_DIVIDER_IS_TURNE = 0,
    #[doc = "4: PLL0 output is divided by 4. PLL0 output must be 192 MHz."]
    PLL0_OUTPUT_IS_DIVID = 4,
    #[doc = "6: PLL0 output is divided by 6. PLL0 output must be 288 MHz."]
    PLL0_OUTPUT_IS_DIVID = 6,
}
impl From<USBDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: USBDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USBDIV` reader - Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation."]
pub struct USBDIV_R(crate::FieldReader<u8, USBDIV_A>);
impl USBDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USBDIV_A> {
        match self.bits {
            0 => Some(USBDIV_A::THE_DIVIDER_IS_TURNE),
            4 => Some(USBDIV_A::PLL0_OUTPUT_IS_DIVID),
            6 => Some(USBDIV_A::PLL0_OUTPUT_IS_DIVID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `THE_DIVIDER_IS_TURNE`"]
    #[inline(always)]
    pub fn is_the_divider_is_turne(&self) -> bool {
        **self == USBDIV_A::THE_DIVIDER_IS_TURNE
    }
    #[doc = "Checks if the value of the field is `PLL0_OUTPUT_IS_DIVID`"]
    #[inline(always)]
    pub fn is_pll0_output_is_divid(&self) -> bool {
        **self == USBDIV_A::PLL0_OUTPUT_IS_DIVID
    }
    #[doc = "Checks if the value of the field is `PLL0_OUTPUT_IS_DIVID`"]
    #[inline(always)]
    pub fn is_pll0_output_is_divid(&self) -> bool {
        **self == USBDIV_A::PLL0_OUTPUT_IS_DIVID
    }
}
impl core::ops::Deref for USBDIV_R {
    type Target = crate::FieldReader<u8, USBDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBDIV` writer - Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation."]
pub struct USBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The divider is turned off, no clock will be provided to the USB subsystem."]
    #[inline(always)]
    pub fn the_divider_is_turne(self) -> &'a mut W {
        self.variant(USBDIV_A::THE_DIVIDER_IS_TURNE)
    }
    #[doc = "PLL0 output is divided by 4. PLL0 output must be 192 MHz."]
    #[inline(always)]
    pub fn pll0_output_is_divid(self) -> &'a mut W {
        self.variant(USBDIV_A::PLL0_OUTPUT_IS_DIVID)
    }
    #[doc = "PLL0 output is divided by 6. PLL0 output must be 288 MHz."]
    #[inline(always)]
    pub fn pll0_output_is_divid(self) -> &'a mut W {
        self.variant(USBDIV_A::PLL0_OUTPUT_IS_DIVID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Selects the input clock for the USB clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBSEL_A {
    #[doc = "0: Sysclk is used as the input to the USB clock divider. When this clock is selected, the USB can be accessed by software but cannot perform USB functions."]
    SYSCLK = 0,
    #[doc = "1: The output of the Main PLL is used as the input to the USB clock divider."]
    MAINPLLOUT = 1,
    #[doc = "2: The output of the Alt PLL is used as the input to the USB clock divider."]
    ALTPLLOOUT = 2,
}
impl From<USBSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USBSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USBSEL` reader - Selects the input clock for the USB clock divider."]
pub struct USBSEL_R(crate::FieldReader<u8, USBSEL_A>);
impl USBSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSEL_A {
        match self.bits {
            0 => USBSEL_A::SYSCLK,
            1 => USBSEL_A::MAINPLLOUT,
            2 => USBSEL_A::ALTPLLOOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == USBSEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `MAINPLLOUT`"]
    #[inline(always)]
    pub fn is_mainpllout(&self) -> bool {
        **self == USBSEL_A::MAINPLLOUT
    }
    #[doc = "Checks if the value of the field is `ALTPLLOOUT`"]
    #[inline(always)]
    pub fn is_altplloout(&self) -> bool {
        **self == USBSEL_A::ALTPLLOOUT
    }
}
impl core::ops::Deref for USBSEL_R {
    type Target = crate::FieldReader<u8, USBSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBSEL` writer - Selects the input clock for the USB clock divider."]
pub struct USBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sysclk is used as the input to the USB clock divider. When this clock is selected, the USB can be accessed by software but cannot perform USB functions."]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USBSEL_A::SYSCLK)
    }
    #[doc = "The output of the Main PLL is used as the input to the USB clock divider."]
    #[inline(always)]
    pub fn mainpllout(self) -> &'a mut W {
        self.variant(USBSEL_A::MAINPLLOUT)
    }
    #[doc = "The output of the Alt PLL is used as the input to the USB clock divider."]
    #[inline(always)]
    pub fn altplloout(self) -> &'a mut W {
        self.variant(USBSEL_A::ALTPLLOOUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation."]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation."]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> USBDIV_W {
        USBDIV_W { w: self }
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W {
        USBSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock Selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbclksel](index.html) module"]
pub struct USBCLKSEL_SPEC;
impl crate::RegisterSpec for USBCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbclksel::R](R) reader structure"]
impl crate::Readable for USBCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbclksel::W](W) writer structure"]
impl crate::Writable for USBCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCLKSEL to value 0"]
impl crate::Resettable for USBCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
