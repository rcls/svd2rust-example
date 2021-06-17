#[doc = "Register `PIN%s` reader"]
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN%s` writer"]
pub struct W(crate::W<PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_SPEC>;
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
impl From<crate::W<PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL0` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL0_R(crate::FieldReader<bool, bool>);
impl VAL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL0` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL0_W<'a> {
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
#[doc = "Field `VAL1` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL1_R(crate::FieldReader<bool, bool>);
impl VAL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL1` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL1_W<'a> {
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
#[doc = "Field `VAL2` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL2_R(crate::FieldReader<bool, bool>);
impl VAL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL2` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL2_W<'a> {
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
#[doc = "Field `VAL3` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL3_R(crate::FieldReader<bool, bool>);
impl VAL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL3` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL3_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL3_W<'a> {
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
#[doc = "Field `VAL4` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL4_R(crate::FieldReader<bool, bool>);
impl VAL4_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL4` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL4_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL4_W<'a> {
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
#[doc = "Field `VAL5` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL5_R(crate::FieldReader<bool, bool>);
impl VAL5_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL5` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL5_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL5_W<'a> {
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
#[doc = "Field `VAL6` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL6_R(crate::FieldReader<bool, bool>);
impl VAL6_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL6` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL6_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL6_W<'a> {
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
#[doc = "Field `VAL7` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL7_R(crate::FieldReader<bool, bool>);
impl VAL7_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL7` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL7_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL7_W<'a> {
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
#[doc = "Field `VAL8` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL8_R(crate::FieldReader<bool, bool>);
impl VAL8_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL8` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL8_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL8_W<'a> {
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
#[doc = "Field `VAL9` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL9_R(crate::FieldReader<bool, bool>);
impl VAL9_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL9` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL9_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL9_W<'a> {
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
#[doc = "Field `VAL10` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL10_R(crate::FieldReader<bool, bool>);
impl VAL10_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL10` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL10_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL10_W<'a> {
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
#[doc = "Field `VAL11` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL11_R(crate::FieldReader<bool, bool>);
impl VAL11_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL11` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL11_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL11_W<'a> {
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
#[doc = "Field `VAL12` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL12_R(crate::FieldReader<bool, bool>);
impl VAL12_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL12` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL12_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL12_W<'a> {
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
#[doc = "Field `VAL13` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL13_R(crate::FieldReader<bool, bool>);
impl VAL13_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL13` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL13_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL13_W<'a> {
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
#[doc = "Field `VAL14` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL14_R(crate::FieldReader<bool, bool>);
impl VAL14_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL14` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL14_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL14_W<'a> {
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
#[doc = "Field `VAL15` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL15_R(crate::FieldReader<bool, bool>);
impl VAL15_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL15` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL15_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL15_W<'a> {
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
#[doc = "Field `VAL16` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL16_R(crate::FieldReader<bool, bool>);
impl VAL16_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL16` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL16_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL16_W<'a> {
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
#[doc = "Field `VAL17` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL17_R(crate::FieldReader<bool, bool>);
impl VAL17_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL17` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL17_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL17_W<'a> {
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
#[doc = "Field `VAL18` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL18_R(crate::FieldReader<bool, bool>);
impl VAL18_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL18` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL18_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL18_W<'a> {
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
#[doc = "Field `VAL19` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL19_R(crate::FieldReader<bool, bool>);
impl VAL19_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL19` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL19_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL19_W<'a> {
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
#[doc = "Field `VAL20` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL20_R(crate::FieldReader<bool, bool>);
impl VAL20_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL20` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL20_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL20_W<'a> {
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
#[doc = "Field `VAL21` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL21_R(crate::FieldReader<bool, bool>);
impl VAL21_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL21` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL21_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL21_W<'a> {
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
#[doc = "Field `VAL22` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL22_R(crate::FieldReader<bool, bool>);
impl VAL22_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL22` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL22_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL22_W<'a> {
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
#[doc = "Field `VAL23` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL23_R(crate::FieldReader<bool, bool>);
impl VAL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL23` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL23_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL23_W<'a> {
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
#[doc = "Field `VAL24` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL24_R(crate::FieldReader<bool, bool>);
impl VAL24_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL24` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL24_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL24_W<'a> {
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
#[doc = "Field `VAL25` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL25_R(crate::FieldReader<bool, bool>);
impl VAL25_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL25` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL25_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL25_W<'a> {
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
#[doc = "Field `VAL26` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL26_R(crate::FieldReader<bool, bool>);
impl VAL26_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL26` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL26_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL26_W<'a> {
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
#[doc = "Field `VAL27` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL27_R(crate::FieldReader<bool, bool>);
impl VAL27_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL27` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL27_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL27_W<'a> {
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
#[doc = "Field `VAL28` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL28_R(crate::FieldReader<bool, bool>);
impl VAL28_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL28` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL28_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL28_W<'a> {
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
#[doc = "Field `VAL29` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL29_R(crate::FieldReader<bool, bool>);
impl VAL29_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL29` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL29_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL29_W<'a> {
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
#[doc = "Field `VAL30` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL30_R(crate::FieldReader<bool, bool>);
impl VAL30_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL30` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL30_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL30_W<'a> {
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
#[doc = "Field `VAL31` reader - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL31_R(crate::FieldReader<bool, bool>);
impl VAL31_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL31` writer - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
pub struct VAL31_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL31_W<'a> {
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
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val0(&self) -> VAL0_R {
        VAL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val1(&self) -> VAL1_R {
        VAL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val2(&self) -> VAL2_R {
        VAL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val3(&self) -> VAL3_R {
        VAL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val4(&self) -> VAL4_R {
        VAL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val5(&self) -> VAL5_R {
        VAL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val6(&self) -> VAL6_R {
        VAL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val7(&self) -> VAL7_R {
        VAL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val8(&self) -> VAL8_R {
        VAL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val9(&self) -> VAL9_R {
        VAL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val10(&self) -> VAL10_R {
        VAL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val11(&self) -> VAL11_R {
        VAL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val12(&self) -> VAL12_R {
        VAL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val13(&self) -> VAL13_R {
        VAL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val14(&self) -> VAL14_R {
        VAL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val15(&self) -> VAL15_R {
        VAL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val16(&self) -> VAL16_R {
        VAL16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val17(&self) -> VAL17_R {
        VAL17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val18(&self) -> VAL18_R {
        VAL18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val19(&self) -> VAL19_R {
        VAL19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val20(&self) -> VAL20_R {
        VAL20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val21(&self) -> VAL21_R {
        VAL21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val22(&self) -> VAL22_R {
        VAL22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val23(&self) -> VAL23_R {
        VAL23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val24(&self) -> VAL24_R {
        VAL24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val25(&self) -> VAL25_R {
        VAL25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val26(&self) -> VAL26_R {
        VAL26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val27(&self) -> VAL27_R {
        VAL27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val28(&self) -> VAL28_R {
        VAL28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val29(&self) -> VAL29_R {
        VAL29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val30(&self) -> VAL30_R {
        VAL30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val31(&self) -> VAL31_R {
        VAL31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val0(&mut self) -> VAL0_W {
        VAL0_W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val1(&mut self) -> VAL1_W {
        VAL1_W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val2(&mut self) -> VAL2_W {
        VAL2_W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val3(&mut self) -> VAL3_W {
        VAL3_W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val4(&mut self) -> VAL4_W {
        VAL4_W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val5(&mut self) -> VAL5_W {
        VAL5_W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val6(&mut self) -> VAL6_W {
        VAL6_W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val7(&mut self) -> VAL7_W {
        VAL7_W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val8(&mut self) -> VAL8_W {
        VAL8_W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val9(&mut self) -> VAL9_W {
        VAL9_W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val10(&mut self) -> VAL10_W {
        VAL10_W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val11(&mut self) -> VAL11_W {
        VAL11_W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val12(&mut self) -> VAL12_W {
        VAL12_W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val13(&mut self) -> VAL13_W {
        VAL13_W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val14(&mut self) -> VAL14_W {
        VAL14_W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val15(&mut self) -> VAL15_W {
        VAL15_W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val16(&mut self) -> VAL16_W {
        VAL16_W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val17(&mut self) -> VAL17_W {
        VAL17_W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val18(&mut self) -> VAL18_W {
        VAL18_W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val19(&mut self) -> VAL19_W {
        VAL19_W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val20(&mut self) -> VAL20_W {
        VAL20_W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val21(&mut self) -> VAL21_W {
        VAL21_W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val22(&mut self) -> VAL22_W {
        VAL22_W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val23(&mut self) -> VAL23_W {
        VAL23_W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val24(&mut self) -> VAL24_W {
        VAL24_W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val25(&mut self) -> VAL25_W {
        VAL25_W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val26(&mut self) -> VAL26_W {
        VAL26_W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val27(&mut self) -> VAL27_W {
        VAL27_W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val28(&mut self) -> VAL28_W {
        VAL28_W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val29(&mut self) -> VAL29_W {
        VAL29_W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val30(&mut self) -> VAL30_W {
        VAL30_W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in xCLR corresponds to pin Px\\[0\\], bit 31 in xCLR corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn val31(&mut self) -> VAL31_W {
        VAL31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Pin value register using MASK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](index.html) module"]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin::R](R) reader structure"]
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin::W](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
