#[doc = "Register `OA0_SW_CLEAR` reader"]
pub struct R(crate::R<OA0_SW_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OA0_SW_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OA0_SW_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OA0_SW_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OA0_SW_CLEAR` writer"]
pub struct W(crate::W<OA0_SW_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OA0_SW_CLEAR_SPEC>;
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
impl From<crate::W<OA0_SW_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OA0_SW_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA0P_A00` reader - see corresponding bit in OA0_SW"]
pub struct OA0P_A00_R(crate::FieldReader<bool, bool>);
impl OA0P_A00_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA0P_A00_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA0P_A00_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA0P_A00` writer - see corresponding bit in OA0_SW"]
pub struct OA0P_A00_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0P_A00_W<'a> {
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
#[doc = "Field `OA0P_A20` reader - see corresponding bit in OA0_SW"]
pub struct OA0P_A20_R(crate::FieldReader<bool, bool>);
impl OA0P_A20_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA0P_A20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA0P_A20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA0P_A20` writer - see corresponding bit in OA0_SW"]
pub struct OA0P_A20_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0P_A20_W<'a> {
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
#[doc = "Field `OA0P_A30` reader - see corresponding bit in OA0_SW"]
pub struct OA0P_A30_R(crate::FieldReader<bool, bool>);
impl OA0P_A30_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA0P_A30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA0P_A30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA0P_A30` writer - see corresponding bit in OA0_SW"]
pub struct OA0P_A30_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0P_A30_W<'a> {
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
#[doc = "Field `OA0M_A11` reader - see corresponding bit in OA0_SW"]
pub struct OA0M_A11_R(crate::FieldReader<bool, bool>);
impl OA0M_A11_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA0M_A11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA0M_A11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA0M_A11` writer - see corresponding bit in OA0_SW"]
pub struct OA0M_A11_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0M_A11_W<'a> {
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
#[doc = "Field `OA0M_A81` reader - see corresponding bit in OA0_SW"]
pub struct OA0M_A81_R(crate::FieldReader<bool, bool>);
impl OA0M_A81_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA0M_A81_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA0M_A81_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA0M_A81` writer - see corresponding bit in OA0_SW"]
pub struct OA0M_A81_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0M_A81_W<'a> {
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
#[doc = "Field `OA0O_D51` reader - see corresponding bit in OA0_SW"]
pub struct OA0O_D51_R(crate::FieldReader<bool, bool>);
impl OA0O_D51_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA0O_D51_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA0O_D51_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA0O_D51` writer - see corresponding bit in OA0_SW"]
pub struct OA0O_D51_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0O_D51_W<'a> {
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
#[doc = "Field `OA0O_D81` reader - see corresponding bit in OA0_SW"]
pub struct OA0O_D81_R(crate::FieldReader<bool, bool>);
impl OA0O_D81_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA0O_D81_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA0O_D81_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA0O_D81` writer - see corresponding bit in OA0_SW"]
pub struct OA0O_D81_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0O_D81_W<'a> {
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
impl R {
    #[doc = "Bit 0 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a00(&self) -> OA0P_A00_R {
        OA0P_A00_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a20(&self) -> OA0P_A20_R {
        OA0P_A20_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a30(&self) -> OA0P_A30_R {
        OA0P_A30_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0m_a11(&self) -> OA0M_A11_R {
        OA0M_A11_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 14 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0m_a81(&self) -> OA0M_A81_R {
        OA0M_A81_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 18 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d51(&self) -> OA0O_D51_R {
        OA0O_D51_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 21 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d81(&self) -> OA0O_D81_R {
        OA0O_D81_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a00(&mut self) -> OA0P_A00_W {
        OA0P_A00_W { w: self }
    }
    #[doc = "Bit 2 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a20(&mut self) -> OA0P_A20_W {
        OA0P_A20_W { w: self }
    }
    #[doc = "Bit 3 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a30(&mut self) -> OA0P_A30_W {
        OA0P_A30_W { w: self }
    }
    #[doc = "Bit 8 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0m_a11(&mut self) -> OA0M_A11_W {
        OA0M_A11_W { w: self }
    }
    #[doc = "Bit 14 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0m_a81(&mut self) -> OA0M_A81_W {
        OA0M_A81_W { w: self }
    }
    #[doc = "Bit 18 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d51(&mut self) -> OA0O_D51_W {
        OA0O_D51_W { w: self }
    }
    #[doc = "Bit 21 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d81(&mut self) -> OA0O_D81_W {
        OA0O_D81_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opamp0 switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa0_sw_clear](index.html) module"]
pub struct OA0_SW_CLEAR_SPEC;
impl crate::RegisterSpec for OA0_SW_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oa0_sw_clear::R](R) reader structure"]
impl crate::Readable for OA0_SW_CLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oa0_sw_clear::W](W) writer structure"]
impl crate::Writable for OA0_SW_CLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OA0_SW_CLEAR to value 0"]
impl crate::Resettable for OA0_SW_CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
