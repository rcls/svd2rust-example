#[doc = "Register `CMP1_SW` reader"]
pub struct R(crate::R<CMP1_SW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP1_SW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP1_SW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP1_SW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP1_SW` writer"]
pub struct W(crate::W<CMP1_SW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP1_SW_SPEC>;
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
impl From<crate::W<CMP1_SW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP1_SW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1_IP1` reader - Comparator 1 positive terminal isolation switch to GPIO"]
pub struct CMP1_IP1_R(crate::FieldReader<bool, bool>);
impl CMP1_IP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1_IP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1_IP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1_IP1` writer - Comparator 1 positive terminal isolation switch to GPIO"]
pub struct CMP1_IP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_IP1_W<'a> {
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
#[doc = "Field `CMP1_AP1` reader - Comparator 1 positive terminal switch to amuxbusA"]
pub struct CMP1_AP1_R(crate::FieldReader<bool, bool>);
impl CMP1_AP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1_AP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1_AP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1_AP1` writer - Comparator 1 positive terminal switch to amuxbusA"]
pub struct CMP1_AP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_AP1_W<'a> {
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
#[doc = "Field `CMP1_BP1` reader - Comparator 1 positive terminal switch to amuxbusB"]
pub struct CMP1_BP1_R(crate::FieldReader<bool, bool>);
impl CMP1_BP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1_BP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1_BP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1_BP1` writer - Comparator 1 positive terminal switch to amuxbusB"]
pub struct CMP1_BP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_BP1_W<'a> {
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
#[doc = "Field `CMP1_IN1` reader - Comparator 1 negative terminal isolation switch to GPIO"]
pub struct CMP1_IN1_R(crate::FieldReader<bool, bool>);
impl CMP1_IN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1_IN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1_IN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1_IN1` writer - Comparator 1 negative terminal isolation switch to GPIO"]
pub struct CMP1_IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_IN1_W<'a> {
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
#[doc = "Field `CMP1_AN1` reader - Comparator 1 negative terminal switch to amuxbusA"]
pub struct CMP1_AN1_R(crate::FieldReader<bool, bool>);
impl CMP1_AN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1_AN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1_AN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1_AN1` writer - Comparator 1 negative terminal switch to amuxbusA"]
pub struct CMP1_AN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_AN1_W<'a> {
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
#[doc = "Field `CMP1_BN1` reader - Comparator 1 negative terminal switch to amuxbusB"]
pub struct CMP1_BN1_R(crate::FieldReader<bool, bool>);
impl CMP1_BN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1_BN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1_BN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1_BN1` writer - Comparator 1 negative terminal switch to amuxbusB"]
pub struct CMP1_BN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_BN1_W<'a> {
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
#[doc = "Field `CMP1_VN1` reader - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
pub struct CMP1_VN1_R(crate::FieldReader<bool, bool>);
impl CMP1_VN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1_VN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1_VN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1_VN1` writer - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
pub struct CMP1_VN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_VN1_W<'a> {
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
    #[doc = "Bit 0 - Comparator 1 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_ip1(&self) -> CMP1_IP1_R {
        CMP1_IP1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_ap1(&self) -> CMP1_AP1_R {
        CMP1_AP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator 1 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bp1(&self) -> CMP1_BP1_R {
        CMP1_BP1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator 1 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_in1(&self) -> CMP1_IN1_R {
        CMP1_IN1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comparator 1 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_an1(&self) -> CMP1_AN1_R {
        CMP1_AN1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Comparator 1 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bn1(&self) -> CMP1_BN1_R {
        CMP1_BN1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub fn cmp1_vn1(&self) -> CMP1_VN1_R {
        CMP1_VN1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_ip1(&mut self) -> CMP1_IP1_W {
        CMP1_IP1_W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_ap1(&mut self) -> CMP1_AP1_W {
        CMP1_AP1_W { w: self }
    }
    #[doc = "Bit 2 - Comparator 1 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bp1(&mut self) -> CMP1_BP1_W {
        CMP1_BP1_W { w: self }
    }
    #[doc = "Bit 4 - Comparator 1 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_in1(&mut self) -> CMP1_IN1_W {
        CMP1_IN1_W { w: self }
    }
    #[doc = "Bit 5 - Comparator 1 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_an1(&mut self) -> CMP1_AN1_W {
        CMP1_AN1_W { w: self }
    }
    #[doc = "Bit 6 - Comparator 1 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bn1(&mut self) -> CMP1_BN1_W {
        CMP1_BN1_W { w: self }
    }
    #[doc = "Bit 7 - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub fn cmp1_vn1(&mut self) -> CMP1_VN1_W {
        CMP1_VN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 1 switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1_sw](index.html) module"]
pub struct CMP1_SW_SPEC;
impl crate::RegisterSpec for CMP1_SW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp1_sw::R](R) reader structure"]
impl crate::Readable for CMP1_SW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp1_sw::W](W) writer structure"]
impl crate::Writable for CMP1_SW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMP1_SW to value 0"]
impl crate::Resettable for CMP1_SW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
