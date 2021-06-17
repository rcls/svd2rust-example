#[doc = "Register `MASK%s` reader"]
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK%s` writer"]
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
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
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMASK0` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK0_R(crate::FieldReader<bool, bool>);
impl PMASK0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK0` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK0_W<'a> {
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
#[doc = "Field `PMASK1` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK1_R(crate::FieldReader<bool, bool>);
impl PMASK1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK1` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK1_W<'a> {
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
#[doc = "Field `PMASK2` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK2_R(crate::FieldReader<bool, bool>);
impl PMASK2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK2` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK2_W<'a> {
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
#[doc = "Field `PMASK3` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK3_R(crate::FieldReader<bool, bool>);
impl PMASK3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK3` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PMASK4` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK4_R(crate::FieldReader<bool, bool>);
impl PMASK4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK4` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK4_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PMASK5` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK5_R(crate::FieldReader<bool, bool>);
impl PMASK5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK5` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK5_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PMASK6` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK6_R(crate::FieldReader<bool, bool>);
impl PMASK6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK6` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK6_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK6_W<'a> {
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
#[doc = "Field `PMASK7` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK7_R(crate::FieldReader<bool, bool>);
impl PMASK7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK7` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK7_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK7_W<'a> {
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
#[doc = "Field `PMASK8` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK8_R(crate::FieldReader<bool, bool>);
impl PMASK8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK8` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK8_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PMASK9` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK9_R(crate::FieldReader<bool, bool>);
impl PMASK9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK9` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK9_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PMASK10` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK10_R(crate::FieldReader<bool, bool>);
impl PMASK10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK10` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK10_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `PMASK11` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK11_R(crate::FieldReader<bool, bool>);
impl PMASK11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK11` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK11_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PMASK12` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK12_R(crate::FieldReader<bool, bool>);
impl PMASK12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK12` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK12_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `PMASK13` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK13_R(crate::FieldReader<bool, bool>);
impl PMASK13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK13` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK13_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PMASK14` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK14_R(crate::FieldReader<bool, bool>);
impl PMASK14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK14` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK14_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PMASK15` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK15_R(crate::FieldReader<bool, bool>);
impl PMASK15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK15` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK15_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `PMASK16` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK16_R(crate::FieldReader<bool, bool>);
impl PMASK16_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK16` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK16_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PMASK17` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK17_R(crate::FieldReader<bool, bool>);
impl PMASK17_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK17` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK17_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PMASK18` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK18_R(crate::FieldReader<bool, bool>);
impl PMASK18_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK18` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK18_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PMASK19` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK19_R(crate::FieldReader<bool, bool>);
impl PMASK19_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK19` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK19_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PMASK20` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK20_R(crate::FieldReader<bool, bool>);
impl PMASK20_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK20` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK20_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PMASK21` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK21_R(crate::FieldReader<bool, bool>);
impl PMASK21_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK21` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK21_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `PMASK22` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK22_R(crate::FieldReader<bool, bool>);
impl PMASK22_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK22` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK22_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PMASK23` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK23_R(crate::FieldReader<bool, bool>);
impl PMASK23_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK23` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK23_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PMASK24` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK24_R(crate::FieldReader<bool, bool>);
impl PMASK24_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK24` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK24_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PMASK25` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK25_R(crate::FieldReader<bool, bool>);
impl PMASK25_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK25` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK25_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PMASK26` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK26_R(crate::FieldReader<bool, bool>);
impl PMASK26_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK26` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK26_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PMASK27` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK27_R(crate::FieldReader<bool, bool>);
impl PMASK27_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK27` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK27_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PMASK28` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK28_R(crate::FieldReader<bool, bool>);
impl PMASK28_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK28` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK28_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `PMASK29` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK29_R(crate::FieldReader<bool, bool>);
impl PMASK29_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK29` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK29_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `PMASK30` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK30_R(crate::FieldReader<bool, bool>);
impl PMASK30_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK30` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK30_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `PMASK31` reader - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK31_R(crate::FieldReader<bool, bool>);
impl PMASK31_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMASK31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMASK31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMASK31` writer - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
pub struct PMASK31_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask0(&self) -> PMASK0_R {
        PMASK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask1(&self) -> PMASK1_R {
        PMASK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask2(&self) -> PMASK2_R {
        PMASK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask3(&self) -> PMASK3_R {
        PMASK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask4(&self) -> PMASK4_R {
        PMASK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask5(&self) -> PMASK5_R {
        PMASK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask6(&self) -> PMASK6_R {
        PMASK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask7(&self) -> PMASK7_R {
        PMASK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask8(&self) -> PMASK8_R {
        PMASK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask9(&self) -> PMASK9_R {
        PMASK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask10(&self) -> PMASK10_R {
        PMASK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask11(&self) -> PMASK11_R {
        PMASK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask12(&self) -> PMASK12_R {
        PMASK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask13(&self) -> PMASK13_R {
        PMASK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask14(&self) -> PMASK14_R {
        PMASK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask15(&self) -> PMASK15_R {
        PMASK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask16(&self) -> PMASK16_R {
        PMASK16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask17(&self) -> PMASK17_R {
        PMASK17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask18(&self) -> PMASK18_R {
        PMASK18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask19(&self) -> PMASK19_R {
        PMASK19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask20(&self) -> PMASK20_R {
        PMASK20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask21(&self) -> PMASK21_R {
        PMASK21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask22(&self) -> PMASK22_R {
        PMASK22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask23(&self) -> PMASK23_R {
        PMASK23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask24(&self) -> PMASK24_R {
        PMASK24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask25(&self) -> PMASK25_R {
        PMASK25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask26(&self) -> PMASK26_R {
        PMASK26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask27(&self) -> PMASK27_R {
        PMASK27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask28(&self) -> PMASK28_R {
        PMASK28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask29(&self) -> PMASK29_R {
        PMASK29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask30(&self) -> PMASK30_R {
        PMASK30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask31(&self) -> PMASK31_R {
        PMASK31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask0(&mut self) -> PMASK0_W {
        PMASK0_W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask1(&mut self) -> PMASK1_W {
        PMASK1_W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask2(&mut self) -> PMASK2_W {
        PMASK2_W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask3(&mut self) -> PMASK3_W {
        PMASK3_W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask4(&mut self) -> PMASK4_W {
        PMASK4_W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask5(&mut self) -> PMASK5_W {
        PMASK5_W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask6(&mut self) -> PMASK6_W {
        PMASK6_W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask7(&mut self) -> PMASK7_W {
        PMASK7_W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask8(&mut self) -> PMASK8_W {
        PMASK8_W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask9(&mut self) -> PMASK9_W {
        PMASK9_W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask10(&mut self) -> PMASK10_W {
        PMASK10_W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask11(&mut self) -> PMASK11_W {
        PMASK11_W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask12(&mut self) -> PMASK12_W {
        PMASK12_W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask13(&mut self) -> PMASK13_W {
        PMASK13_W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask14(&mut self) -> PMASK14_W {
        PMASK14_W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask15(&mut self) -> PMASK15_W {
        PMASK15_W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask16(&mut self) -> PMASK16_W {
        PMASK16_W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask17(&mut self) -> PMASK17_W {
        PMASK17_W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask18(&mut self) -> PMASK18_W {
        PMASK18_W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask19(&mut self) -> PMASK19_W {
        PMASK19_W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask20(&mut self) -> PMASK20_W {
        PMASK20_W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask21(&mut self) -> PMASK21_W {
        PMASK21_W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask22(&mut self) -> PMASK22_W {
        PMASK22_W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask23(&mut self) -> PMASK23_W {
        PMASK23_W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask24(&mut self) -> PMASK24_W {
        PMASK24_W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask25(&mut self) -> PMASK25_W {
        PMASK25_W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask26(&mut self) -> PMASK26_W {
        PMASK26_W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask27(&mut self) -> PMASK27_W {
        PMASK27_W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask28(&mut self) -> PMASK28_W {
        PMASK28_W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask29(&mut self) -> PMASK29_W {
        PMASK29_W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask30(&mut self) -> PMASK30_W {
        PMASK30_W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's xSET, xCLR, and xPIN register(s). Current state of the pin can be read from the xPIN register. 1 = Controlled pin is not affected by writes into the port's xSET, xCLR and xPIN register(s). When the xPIN register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pmask31(&mut self) -> PMASK31_W {
        PMASK31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask register for Port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](index.html) module"]
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask::R](R) reader structure"]
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask::W](W) writer structure"]
impl crate::Writable for MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK%s to value 0"]
impl crate::Resettable for MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
