#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IrDA mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRDAEN_A {
    #[doc = "0: Disabled. IrDA mode on UART4 is disabled, UART4 acts as a standard UART."]
    DISABLED_IRDA_MODE_ = 0,
    #[doc = "1: Enabled. IrDA mode on UART4 is enabled."]
    ENABLED_IRDA_MODE_O = 1,
}
impl From<IRDAEN_A> for bool {
    #[inline(always)]
    fn from(variant: IRDAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDAEN` reader - IrDA mode"]
pub struct IRDAEN_R(crate::FieldReader<bool, IRDAEN_A>);
impl IRDAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRDAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRDAEN_A {
        match self.bits {
            false => IRDAEN_A::DISABLED_IRDA_MODE_,
            true => IRDAEN_A::ENABLED_IRDA_MODE_O,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_IRDA_MODE_`"]
    #[inline(always)]
    pub fn is_disabled_irda_mode_(&self) -> bool {
        **self == IRDAEN_A::DISABLED_IRDA_MODE_
    }
    #[doc = "Checks if the value of the field is `ENABLED_IRDA_MODE_O`"]
    #[inline(always)]
    pub fn is_enabled_irda_mode_o(&self) -> bool {
        **self == IRDAEN_A::ENABLED_IRDA_MODE_O
    }
}
impl core::ops::Deref for IRDAEN_R {
    type Target = crate::FieldReader<bool, IRDAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRDAEN` writer - IrDA mode"]
pub struct IRDAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRDAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. IrDA mode on UART4 is disabled, UART4 acts as a standard UART."]
    #[inline(always)]
    pub fn disabled_irda_mode_(self) -> &'a mut W {
        self.variant(IRDAEN_A::DISABLED_IRDA_MODE_)
    }
    #[doc = "Enabled. IrDA mode on UART4 is enabled."]
    #[inline(always)]
    pub fn enabled_irda_mode_o(self) -> &'a mut W {
        self.variant(IRDAEN_A::ENABLED_IRDA_MODE_O)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Serial input direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRDAINV_A {
    #[doc = "0: Not inverted."]
    NOT_INVERTED_ = 0,
    #[doc = "1: Inverted. This has no effect on the serial output."]
    INVERTED_THIS_HAS_N = 1,
}
impl From<IRDAINV_A> for bool {
    #[inline(always)]
    fn from(variant: IRDAINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDAINV` reader - Serial input direction."]
pub struct IRDAINV_R(crate::FieldReader<bool, IRDAINV_A>);
impl IRDAINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRDAINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRDAINV_A {
        match self.bits {
            false => IRDAINV_A::NOT_INVERTED_,
            true => IRDAINV_A::INVERTED_THIS_HAS_N,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED_`"]
    #[inline(always)]
    pub fn is_not_inverted_(&self) -> bool {
        **self == IRDAINV_A::NOT_INVERTED_
    }
    #[doc = "Checks if the value of the field is `INVERTED_THIS_HAS_N`"]
    #[inline(always)]
    pub fn is_inverted_this_has_n(&self) -> bool {
        **self == IRDAINV_A::INVERTED_THIS_HAS_N
    }
}
impl core::ops::Deref for IRDAINV_R {
    type Target = crate::FieldReader<bool, IRDAINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRDAINV` writer - Serial input direction."]
pub struct IRDAINV_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDAINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRDAINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not inverted."]
    #[inline(always)]
    pub fn not_inverted_(self) -> &'a mut W {
        self.variant(IRDAINV_A::NOT_INVERTED_)
    }
    #[doc = "Inverted. This has no effect on the serial output."]
    #[inline(always)]
    pub fn inverted_this_has_n(self) -> &'a mut W {
        self.variant(IRDAINV_A::INVERTED_THIS_HAS_N)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "IrDA fixed pulse width mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXPULSEEN_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Enabled."]
    ENABLED_ = 1,
}
impl From<FIXPULSEEN_A> for bool {
    #[inline(always)]
    fn from(variant: FIXPULSEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIXPULSEEN` reader - IrDA fixed pulse width mode."]
pub struct FIXPULSEEN_R(crate::FieldReader<bool, FIXPULSEEN_A>);
impl FIXPULSEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIXPULSEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXPULSEEN_A {
        match self.bits {
            false => FIXPULSEEN_A::DISABLED_,
            true => FIXPULSEEN_A::ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        **self == FIXPULSEEN_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        **self == FIXPULSEEN_A::ENABLED_
    }
}
impl core::ops::Deref for FIXPULSEEN_R {
    type Target = crate::FieldReader<bool, FIXPULSEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIXPULSEEN` writer - IrDA fixed pulse width mode."]
pub struct FIXPULSEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXPULSEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIXPULSEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(FIXPULSEEN_A::DISABLED_)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(FIXPULSEEN_A::ENABLED_)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Configures the pulse when FixPulseEn = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PULSEDIV_A {
    #[doc = "0: 2xTPCLK"]
    _2XTPCLK = 0,
    #[doc = "1: 4xTPCLK"]
    _4XTPCLK = 1,
    #[doc = "2: 8xTPCLK"]
    _8XTPCLK = 2,
    #[doc = "3: 16xTPCLK"]
    _16XTPCLK = 3,
    #[doc = "4: 32xTPCLK"]
    _32XTPCLK = 4,
    #[doc = "5: 64xTPCLK"]
    _64XTPCLK = 5,
    #[doc = "6: 128xTPCLK"]
    _128XTPCLK = 6,
    #[doc = "7: 256xTPCLK"]
    _256XTPCLK = 7,
}
impl From<PULSEDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PULSEDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PULSEDIV` reader - Configures the pulse when FixPulseEn = 1."]
pub struct PULSEDIV_R(crate::FieldReader<u8, PULSEDIV_A>);
impl PULSEDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PULSEDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULSEDIV_A {
        match self.bits {
            0 => PULSEDIV_A::_2XTPCLK,
            1 => PULSEDIV_A::_4XTPCLK,
            2 => PULSEDIV_A::_8XTPCLK,
            3 => PULSEDIV_A::_16XTPCLK,
            4 => PULSEDIV_A::_32XTPCLK,
            5 => PULSEDIV_A::_64XTPCLK,
            6 => PULSEDIV_A::_128XTPCLK,
            7 => PULSEDIV_A::_256XTPCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2XTPCLK`"]
    #[inline(always)]
    pub fn is_2xtpclk(&self) -> bool {
        **self == PULSEDIV_A::_2XTPCLK
    }
    #[doc = "Checks if the value of the field is `_4XTPCLK`"]
    #[inline(always)]
    pub fn is_4xtpclk(&self) -> bool {
        **self == PULSEDIV_A::_4XTPCLK
    }
    #[doc = "Checks if the value of the field is `_8XTPCLK`"]
    #[inline(always)]
    pub fn is_8xtpclk(&self) -> bool {
        **self == PULSEDIV_A::_8XTPCLK
    }
    #[doc = "Checks if the value of the field is `_16XTPCLK`"]
    #[inline(always)]
    pub fn is_16xtpclk(&self) -> bool {
        **self == PULSEDIV_A::_16XTPCLK
    }
    #[doc = "Checks if the value of the field is `_32XTPCLK`"]
    #[inline(always)]
    pub fn is_32xtpclk(&self) -> bool {
        **self == PULSEDIV_A::_32XTPCLK
    }
    #[doc = "Checks if the value of the field is `_64XTPCLK`"]
    #[inline(always)]
    pub fn is_64xtpclk(&self) -> bool {
        **self == PULSEDIV_A::_64XTPCLK
    }
    #[doc = "Checks if the value of the field is `_128XTPCLK`"]
    #[inline(always)]
    pub fn is_128xtpclk(&self) -> bool {
        **self == PULSEDIV_A::_128XTPCLK
    }
    #[doc = "Checks if the value of the field is `_256XTPCLK`"]
    #[inline(always)]
    pub fn is_256xtpclk(&self) -> bool {
        **self == PULSEDIV_A::_256XTPCLK
    }
}
impl core::ops::Deref for PULSEDIV_R {
    type Target = crate::FieldReader<u8, PULSEDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULSEDIV` writer - Configures the pulse when FixPulseEn = 1."]
pub struct PULSEDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSEDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PULSEDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2xTPCLK"]
    #[inline(always)]
    pub fn _2xtpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_2XTPCLK)
    }
    #[doc = "4xTPCLK"]
    #[inline(always)]
    pub fn _4xtpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_4XTPCLK)
    }
    #[doc = "8xTPCLK"]
    #[inline(always)]
    pub fn _8xtpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_8XTPCLK)
    }
    #[doc = "16xTPCLK"]
    #[inline(always)]
    pub fn _16xtpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_16XTPCLK)
    }
    #[doc = "32xTPCLK"]
    #[inline(always)]
    pub fn _32xtpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_32XTPCLK)
    }
    #[doc = "64xTPCLK"]
    #[inline(always)]
    pub fn _64xtpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_64XTPCLK)
    }
    #[doc = "128xTPCLK"]
    #[inline(always)]
    pub fn _128xtpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_128XTPCLK)
    }
    #[doc = "256xTPCLK"]
    #[inline(always)]
    pub fn _256xtpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_256XTPCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IrDA mode"]
    #[inline(always)]
    pub fn irdaen(&self) -> IRDAEN_R {
        IRDAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Serial input direction."]
    #[inline(always)]
    pub fn irdainv(&self) -> IRDAINV_R {
        IRDAINV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IrDA fixed pulse width mode."]
    #[inline(always)]
    pub fn fixpulseen(&self) -> FIXPULSEEN_R {
        FIXPULSEEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Configures the pulse when FixPulseEn = 1."]
    #[inline(always)]
    pub fn pulsediv(&self) -> PULSEDIV_R {
        PULSEDIV_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA mode"]
    #[inline(always)]
    pub fn irdaen(&mut self) -> IRDAEN_W {
        IRDAEN_W { w: self }
    }
    #[doc = "Bit 1 - Serial input direction."]
    #[inline(always)]
    pub fn irdainv(&mut self) -> IRDAINV_W {
        IRDAINV_W { w: self }
    }
    #[doc = "Bit 2 - IrDA fixed pulse width mode."]
    #[inline(always)]
    pub fn fixpulseen(&mut self) -> FIXPULSEEN_W {
        FIXPULSEEN_W { w: self }
    }
    #[doc = "Bits 3:5 - Configures the pulse when FixPulseEn = 1."]
    #[inline(always)]
    pub fn pulsediv(&mut self) -> PULSEDIV_W {
        PULSEDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IrDA Control Register. Enables and configures the IrDA mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
