#[doc = "Register `PWR` reader"]
pub struct R(crate::R<PWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR` writer"]
pub struct W(crate::W<PWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_SPEC>;
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
impl From<crate::W<PWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTRL_A {
    #[doc = "0: Power-off"]
    POWER_OFF = 0,
    #[doc = "2: Power-up"]
    POWER_UP = 2,
    #[doc = "3: Power-on"]
    POWER_ON = 3,
}
impl From<CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTRL` reader - Power control"]
pub struct CTRL_R(crate::FieldReader<u8, CTRL_A>);
impl CTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRL_A {
        match self.bits {
            0 => CTRL_A::POWER_OFF,
            2 => CTRL_A::POWER_UP,
            3 => CTRL_A::POWER_ON,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POWER_OFF`"]
    #[inline(always)]
    pub fn is_power_off(&self) -> bool {
        **self == CTRL_A::POWER_OFF
    }
    #[doc = "Checks if the value of the field is `POWER_UP`"]
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        **self == CTRL_A::POWER_UP
    }
    #[doc = "Checks if the value of the field is `POWER_ON`"]
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        **self == CTRL_A::POWER_ON
    }
}
impl core::ops::Deref for CTRL_R {
    type Target = crate::FieldReader<u8, CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRL` writer - Power control"]
pub struct CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Power-off"]
    #[inline(always)]
    pub fn power_off(self) -> &'a mut W {
        self.variant(CTRL_A::POWER_OFF)
    }
    #[doc = "Power-up"]
    #[inline(always)]
    pub fn power_up(self) -> &'a mut W {
        self.variant(CTRL_A::POWER_UP)
    }
    #[doc = "Power-on"]
    #[inline(always)]
    pub fn power_on(self) -> &'a mut W {
        self.variant(CTRL_A::POWER_ON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `OPENDRAIN` reader - SD_CMD output control."]
pub struct OPENDRAIN_R(crate::FieldReader<bool, bool>);
impl OPENDRAIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPENDRAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPENDRAIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPENDRAIN` writer - SD_CMD output control."]
pub struct OPENDRAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPENDRAIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ROD` reader - Rod control."]
pub struct ROD_R(crate::FieldReader<bool, bool>);
impl ROD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROD` writer - Rod control."]
pub struct ROD_W<'a> {
    w: &'a mut W,
}
impl<'a> ROD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Power control"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 6 - SD_CMD output control."]
    #[inline(always)]
    pub fn opendrain(&self) -> OPENDRAIN_R {
        OPENDRAIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rod control."]
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power control"]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W {
        CTRL_W { w: self }
    }
    #[doc = "Bit 6 - SD_CMD output control."]
    #[inline(always)]
    pub fn opendrain(&mut self) -> OPENDRAIN_W {
        OPENDRAIN_W { w: self }
    }
    #[doc = "Bit 7 - Rod control."]
    #[inline(always)]
    pub fn rod(&mut self) -> ROD_W {
        ROD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr](index.html) module"]
pub struct PWR_SPEC;
impl crate::RegisterSpec for PWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr::R](R) reader structure"]
impl crate::Readable for PWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr::W](W) writer structure"]
impl crate::Writable for PWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR to value 0"]
impl crate::Resettable for PWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
