#[doc = "Register `DIR%s` reader"]
pub struct R(crate::R<DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIR%s` writer"]
pub struct W(crate::W<DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIR_SPEC>;
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
impl From<crate::W<DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDIR0` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR0_R(crate::FieldReader<bool, bool>);
impl PDIR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR0` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR0_W<'a> {
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
#[doc = "Field `PDIR1` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR1_R(crate::FieldReader<bool, bool>);
impl PDIR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR1` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR1_W<'a> {
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
#[doc = "Field `PDIR2` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR2_R(crate::FieldReader<bool, bool>);
impl PDIR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR2` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR2_W<'a> {
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
#[doc = "Field `PDIR3` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR3_R(crate::FieldReader<bool, bool>);
impl PDIR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR3` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR3_W<'a> {
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
#[doc = "Field `PDIR4` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR4_R(crate::FieldReader<bool, bool>);
impl PDIR4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR4` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR4_W<'a> {
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
#[doc = "Field `PDIR5` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR5_R(crate::FieldReader<bool, bool>);
impl PDIR5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR5` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR5_W<'a> {
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
#[doc = "Field `PDIR6` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR6_R(crate::FieldReader<bool, bool>);
impl PDIR6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR6` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR6_W<'a> {
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
#[doc = "Field `PDIR7` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR7_R(crate::FieldReader<bool, bool>);
impl PDIR7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR7` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR7_W<'a> {
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
#[doc = "Field `PDIR8` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR8_R(crate::FieldReader<bool, bool>);
impl PDIR8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR8` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR8_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR8_W<'a> {
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
#[doc = "Field `PDIR9` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR9_R(crate::FieldReader<bool, bool>);
impl PDIR9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR9` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR9_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR9_W<'a> {
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
#[doc = "Field `PDIR10` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR10_R(crate::FieldReader<bool, bool>);
impl PDIR10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR10` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR10_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR10_W<'a> {
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
#[doc = "Field `PDIR11` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR11_R(crate::FieldReader<bool, bool>);
impl PDIR11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR11` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR11_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR11_W<'a> {
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
#[doc = "Field `PDIR12` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR12_R(crate::FieldReader<bool, bool>);
impl PDIR12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR12` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR12_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR12_W<'a> {
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
#[doc = "Field `PDIR13` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR13_R(crate::FieldReader<bool, bool>);
impl PDIR13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR13` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR13_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR13_W<'a> {
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
#[doc = "Field `PDIR14` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR14_R(crate::FieldReader<bool, bool>);
impl PDIR14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR14` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR14_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR14_W<'a> {
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
#[doc = "Field `PDIR15` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR15_R(crate::FieldReader<bool, bool>);
impl PDIR15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR15` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR15_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR15_W<'a> {
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
#[doc = "Field `PDIR16` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR16_R(crate::FieldReader<bool, bool>);
impl PDIR16_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR16` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR16_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR16_W<'a> {
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
#[doc = "Field `PDIR17` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR17_R(crate::FieldReader<bool, bool>);
impl PDIR17_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR17` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR17_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR17_W<'a> {
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
#[doc = "Field `PDIR18` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR18_R(crate::FieldReader<bool, bool>);
impl PDIR18_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR18` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR18_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR18_W<'a> {
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
#[doc = "Field `PDIR19` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR19_R(crate::FieldReader<bool, bool>);
impl PDIR19_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR19` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR19_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR19_W<'a> {
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
#[doc = "Field `PDIR20` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR20_R(crate::FieldReader<bool, bool>);
impl PDIR20_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR20` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR20_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR20_W<'a> {
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
#[doc = "Field `PDIR21` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR21_R(crate::FieldReader<bool, bool>);
impl PDIR21_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR21` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR21_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR21_W<'a> {
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
#[doc = "Field `PDIR22` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR22_R(crate::FieldReader<bool, bool>);
impl PDIR22_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR22` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR22_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR22_W<'a> {
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
#[doc = "Field `PDIR23` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR23_R(crate::FieldReader<bool, bool>);
impl PDIR23_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR23` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR23_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR23_W<'a> {
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
#[doc = "Field `PDIR24` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR24_R(crate::FieldReader<bool, bool>);
impl PDIR24_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR24` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR24_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR24_W<'a> {
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
#[doc = "Field `PDIR25` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR25_R(crate::FieldReader<bool, bool>);
impl PDIR25_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR25` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR25_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR25_W<'a> {
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
#[doc = "Field `PDIR26` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR26_R(crate::FieldReader<bool, bool>);
impl PDIR26_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR26` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR26_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR26_W<'a> {
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
#[doc = "Field `PDIR27` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR27_R(crate::FieldReader<bool, bool>);
impl PDIR27_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR27` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR27_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR27_W<'a> {
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
#[doc = "Field `PDIR28` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR28_R(crate::FieldReader<bool, bool>);
impl PDIR28_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR28` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR28_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR28_W<'a> {
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
#[doc = "Field `PDIR29` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR29_R(crate::FieldReader<bool, bool>);
impl PDIR29_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR29` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR29_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR29_W<'a> {
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
#[doc = "Field `PDIR30` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR30_R(crate::FieldReader<bool, bool>);
impl PDIR30_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR30` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR30_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR30_W<'a> {
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
#[doc = "Field `PDIR31` reader - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR31_R(crate::FieldReader<bool, bool>);
impl PDIR31_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIR31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIR31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIR31` writer - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
pub struct PDIR31_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIR31_W<'a> {
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
    #[doc = "Bit 0 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir0(&self) -> PDIR0_R {
        PDIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir1(&self) -> PDIR1_R {
        PDIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir2(&self) -> PDIR2_R {
        PDIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir3(&self) -> PDIR3_R {
        PDIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir4(&self) -> PDIR4_R {
        PDIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir5(&self) -> PDIR5_R {
        PDIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir6(&self) -> PDIR6_R {
        PDIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir7(&self) -> PDIR7_R {
        PDIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir8(&self) -> PDIR8_R {
        PDIR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir9(&self) -> PDIR9_R {
        PDIR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir10(&self) -> PDIR10_R {
        PDIR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir11(&self) -> PDIR11_R {
        PDIR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir12(&self) -> PDIR12_R {
        PDIR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir13(&self) -> PDIR13_R {
        PDIR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir14(&self) -> PDIR14_R {
        PDIR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir15(&self) -> PDIR15_R {
        PDIR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir16(&self) -> PDIR16_R {
        PDIR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir17(&self) -> PDIR17_R {
        PDIR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir18(&self) -> PDIR18_R {
        PDIR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir19(&self) -> PDIR19_R {
        PDIR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir20(&self) -> PDIR20_R {
        PDIR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir21(&self) -> PDIR21_R {
        PDIR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir22(&self) -> PDIR22_R {
        PDIR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir23(&self) -> PDIR23_R {
        PDIR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir24(&self) -> PDIR24_R {
        PDIR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir25(&self) -> PDIR25_R {
        PDIR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir26(&self) -> PDIR26_R {
        PDIR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir27(&self) -> PDIR27_R {
        PDIR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir28(&self) -> PDIR28_R {
        PDIR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir29(&self) -> PDIR29_R {
        PDIR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir30(&self) -> PDIR30_R {
        PDIR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir31(&self) -> PDIR31_R {
        PDIR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir0(&mut self) -> PDIR0_W {
        PDIR0_W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir1(&mut self) -> PDIR1_W {
        PDIR1_W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir2(&mut self) -> PDIR2_W {
        PDIR2_W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir3(&mut self) -> PDIR3_W {
        PDIR3_W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir4(&mut self) -> PDIR4_W {
        PDIR4_W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir5(&mut self) -> PDIR5_W {
        PDIR5_W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir6(&mut self) -> PDIR6_W {
        PDIR6_W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir7(&mut self) -> PDIR7_W {
        PDIR7_W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir8(&mut self) -> PDIR8_W {
        PDIR8_W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir9(&mut self) -> PDIR9_W {
        PDIR9_W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir10(&mut self) -> PDIR10_W {
        PDIR10_W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir11(&mut self) -> PDIR11_W {
        PDIR11_W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir12(&mut self) -> PDIR12_W {
        PDIR12_W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir13(&mut self) -> PDIR13_W {
        PDIR13_W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir14(&mut self) -> PDIR14_W {
        PDIR14_W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir15(&mut self) -> PDIR15_W {
        PDIR15_W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir16(&mut self) -> PDIR16_W {
        PDIR16_W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir17(&mut self) -> PDIR17_W {
        PDIR17_W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir18(&mut self) -> PDIR18_W {
        PDIR18_W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir19(&mut self) -> PDIR19_W {
        PDIR19_W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir20(&mut self) -> PDIR20_W {
        PDIR20_W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir21(&mut self) -> PDIR21_W {
        PDIR21_W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir22(&mut self) -> PDIR22_W {
        PDIR22_W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir23(&mut self) -> PDIR23_W {
        PDIR23_W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir24(&mut self) -> PDIR24_W {
        PDIR24_W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir25(&mut self) -> PDIR25_W {
        PDIR25_W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir26(&mut self) -> PDIR26_W {
        PDIR26_W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir27(&mut self) -> PDIR27_W {
        PDIR27_W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir28(&mut self) -> PDIR28_W {
        PDIR28_W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir29(&mut self) -> PDIR29_W {
        PDIR29_W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir30(&mut self) -> PDIR30_W {
        PDIR30_W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO Direction PORTx control bits. Bit 0 in xDIR controls pin Px\\[0\\], bit 31 in xDIR controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pdir31(&mut self) -> PDIR31_W {
        PDIR31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Port Direction control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](index.html) module"]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dir::R](R) reader structure"]
impl crate::Readable for DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dir::W](W) writer structure"]
impl crate::Writable for DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIR%s to value 0"]
impl crate::Resettable for DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
