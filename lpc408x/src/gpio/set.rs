#[doc = "Register `SET%s` reader"]
pub struct R(crate::R<SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SET%s` writer"]
pub struct W(crate::W<SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_SPEC>;
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
impl From<crate::W<SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSET0` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET0_R(crate::FieldReader<bool, bool>);
impl PSET0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET0` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET0_W<'a> {
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
#[doc = "Field `PSET1` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET1_R(crate::FieldReader<bool, bool>);
impl PSET1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET1` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET1_W<'a> {
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
#[doc = "Field `PSET2` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET2_R(crate::FieldReader<bool, bool>);
impl PSET2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET2` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET2_W<'a> {
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
#[doc = "Field `PSET3` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET3_R(crate::FieldReader<bool, bool>);
impl PSET3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET3` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET3_W<'a> {
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
#[doc = "Field `PSET4` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET4_R(crate::FieldReader<bool, bool>);
impl PSET4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET4` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET4_W<'a> {
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
#[doc = "Field `PSET5` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET5_R(crate::FieldReader<bool, bool>);
impl PSET5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET5` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET5_W<'a> {
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
#[doc = "Field `PSET6` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET6_R(crate::FieldReader<bool, bool>);
impl PSET6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET6` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET6_W<'a> {
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
#[doc = "Field `PSET7` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET7_R(crate::FieldReader<bool, bool>);
impl PSET7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET7` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET7_W<'a> {
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
#[doc = "Field `PSET8` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET8_R(crate::FieldReader<bool, bool>);
impl PSET8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET8` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET8_W<'a> {
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
#[doc = "Field `PSET9` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET9_R(crate::FieldReader<bool, bool>);
impl PSET9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET9` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET9_W<'a> {
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
#[doc = "Field `PSET10` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET10_R(crate::FieldReader<bool, bool>);
impl PSET10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET10` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET10_W<'a> {
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
#[doc = "Field `PSET11` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET11_R(crate::FieldReader<bool, bool>);
impl PSET11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET11` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET11_W<'a> {
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
#[doc = "Field `PSET12` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET12_R(crate::FieldReader<bool, bool>);
impl PSET12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET12` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET12_W<'a> {
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
#[doc = "Field `PSET13` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET13_R(crate::FieldReader<bool, bool>);
impl PSET13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET13` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET13_W<'a> {
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
#[doc = "Field `PSET14` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET14_R(crate::FieldReader<bool, bool>);
impl PSET14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET14` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET14_W<'a> {
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
#[doc = "Field `PSET15` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET15_R(crate::FieldReader<bool, bool>);
impl PSET15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET15` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET15_W<'a> {
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
#[doc = "Field `PSET16` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET16_R(crate::FieldReader<bool, bool>);
impl PSET16_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET16` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET16_W<'a> {
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
#[doc = "Field `PSET17` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET17_R(crate::FieldReader<bool, bool>);
impl PSET17_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET17` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET17_W<'a> {
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
#[doc = "Field `PSET18` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET18_R(crate::FieldReader<bool, bool>);
impl PSET18_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET18` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET18_W<'a> {
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
#[doc = "Field `PSET19` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET19_R(crate::FieldReader<bool, bool>);
impl PSET19_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET19` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET19_W<'a> {
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
#[doc = "Field `PSET20` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET20_R(crate::FieldReader<bool, bool>);
impl PSET20_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET20` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET20_W<'a> {
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
#[doc = "Field `PSET21` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET21_R(crate::FieldReader<bool, bool>);
impl PSET21_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET21` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET21_W<'a> {
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
#[doc = "Field `PSET22` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET22_R(crate::FieldReader<bool, bool>);
impl PSET22_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET22` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET22_W<'a> {
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
#[doc = "Field `PSET23` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET23_R(crate::FieldReader<bool, bool>);
impl PSET23_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET23` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET23_W<'a> {
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
#[doc = "Field `PSET24` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET24_R(crate::FieldReader<bool, bool>);
impl PSET24_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET24` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET24_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET24_W<'a> {
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
#[doc = "Field `PSET25` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET25_R(crate::FieldReader<bool, bool>);
impl PSET25_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET25` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET25_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET25_W<'a> {
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
#[doc = "Field `PSET26` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET26_R(crate::FieldReader<bool, bool>);
impl PSET26_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET26` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET26_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET26_W<'a> {
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
#[doc = "Field `PSET27` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET27_R(crate::FieldReader<bool, bool>);
impl PSET27_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET27` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET27_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET27_W<'a> {
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
#[doc = "Field `PSET28` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET28_R(crate::FieldReader<bool, bool>);
impl PSET28_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET28` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET28_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET28_W<'a> {
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
#[doc = "Field `PSET29` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET29_R(crate::FieldReader<bool, bool>);
impl PSET29_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET29` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET29_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET29_W<'a> {
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
#[doc = "Field `PSET30` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET30_R(crate::FieldReader<bool, bool>);
impl PSET30_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET30` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET30_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET30_W<'a> {
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
#[doc = "Field `PSET31` reader - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET31_R(crate::FieldReader<bool, bool>);
impl PSET31_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSET31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSET31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSET31` writer - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
pub struct PSET31_W<'a> {
    w: &'a mut W,
}
impl<'a> PSET31_W<'a> {
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
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset0(&self) -> PSET0_R {
        PSET0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset1(&self) -> PSET1_R {
        PSET1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset2(&self) -> PSET2_R {
        PSET2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset3(&self) -> PSET3_R {
        PSET3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset4(&self) -> PSET4_R {
        PSET4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset5(&self) -> PSET5_R {
        PSET5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset6(&self) -> PSET6_R {
        PSET6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset7(&self) -> PSET7_R {
        PSET7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset8(&self) -> PSET8_R {
        PSET8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset9(&self) -> PSET9_R {
        PSET9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset10(&self) -> PSET10_R {
        PSET10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset11(&self) -> PSET11_R {
        PSET11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset12(&self) -> PSET12_R {
        PSET12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset13(&self) -> PSET13_R {
        PSET13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset14(&self) -> PSET14_R {
        PSET14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset15(&self) -> PSET15_R {
        PSET15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset16(&self) -> PSET16_R {
        PSET16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset17(&self) -> PSET17_R {
        PSET17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset18(&self) -> PSET18_R {
        PSET18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset19(&self) -> PSET19_R {
        PSET19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset20(&self) -> PSET20_R {
        PSET20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset21(&self) -> PSET21_R {
        PSET21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset22(&self) -> PSET22_R {
        PSET22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset23(&self) -> PSET23_R {
        PSET23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset24(&self) -> PSET24_R {
        PSET24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset25(&self) -> PSET25_R {
        PSET25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset26(&self) -> PSET26_R {
        PSET26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset27(&self) -> PSET27_R {
        PSET27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset28(&self) -> PSET28_R {
        PSET28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset29(&self) -> PSET29_R {
        PSET29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset30(&self) -> PSET30_R {
        PSET30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset31(&self) -> PSET31_R {
        PSET31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset0(&mut self) -> PSET0_W {
        PSET0_W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset1(&mut self) -> PSET1_W {
        PSET1_W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset2(&mut self) -> PSET2_W {
        PSET2_W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset3(&mut self) -> PSET3_W {
        PSET3_W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset4(&mut self) -> PSET4_W {
        PSET4_W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset5(&mut self) -> PSET5_W {
        PSET5_W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset6(&mut self) -> PSET6_W {
        PSET6_W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset7(&mut self) -> PSET7_W {
        PSET7_W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset8(&mut self) -> PSET8_W {
        PSET8_W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset9(&mut self) -> PSET9_W {
        PSET9_W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset10(&mut self) -> PSET10_W {
        PSET10_W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset11(&mut self) -> PSET11_W {
        PSET11_W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset12(&mut self) -> PSET12_W {
        PSET12_W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset13(&mut self) -> PSET13_W {
        PSET13_W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset14(&mut self) -> PSET14_W {
        PSET14_W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset15(&mut self) -> PSET15_W {
        PSET15_W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset16(&mut self) -> PSET16_W {
        PSET16_W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset17(&mut self) -> PSET17_W {
        PSET17_W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset18(&mut self) -> PSET18_W {
        PSET18_W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset19(&mut self) -> PSET19_W {
        PSET19_W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset20(&mut self) -> PSET20_W {
        PSET20_W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset21(&mut self) -> PSET21_W {
        PSET21_W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset22(&mut self) -> PSET22_W {
        PSET22_W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset23(&mut self) -> PSET23_W {
        PSET23_W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset24(&mut self) -> PSET24_W {
        PSET24_W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset25(&mut self) -> PSET25_W {
        PSET25_W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset26(&mut self) -> PSET26_W {
        PSET26_W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset27(&mut self) -> PSET27_W {
        PSET27_W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset28(&mut self) -> PSET28_W {
        PSET28_W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset29(&mut self) -> PSET29_W {
        PSET29_W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset30(&mut self) -> PSET30_W {
        PSET30_W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in xSET controls pin Px\\[0\\], bit 31 in xSET controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pset31(&mut self) -> PSET31_W {
        PSET31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Output Set register using MASK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set](index.html) module"]
pub struct SET_SPEC;
impl crate::RegisterSpec for SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [set::R](R) reader structure"]
impl crate::Readable for SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [set::W](W) writer structure"]
impl crate::Writable for SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SET%s to value 0"]
impl crate::Resettable for SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
