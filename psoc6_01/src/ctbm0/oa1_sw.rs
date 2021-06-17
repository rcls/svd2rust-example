#[doc = "Register `OA1_SW` reader"]
pub struct R(crate::R<OA1_SW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OA1_SW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OA1_SW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OA1_SW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OA1_SW` writer"]
pub struct W(crate::W<OA1_SW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OA1_SW_SPEC>;
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
impl From<crate::W<OA1_SW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OA1_SW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA1P_A03` reader - Opamp1 positive terminal amuxbusb"]
pub struct OA1P_A03_R(crate::FieldReader<bool, bool>);
impl OA1P_A03_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA1P_A03_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA1P_A03_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1P_A03` writer - Opamp1 positive terminal amuxbusb"]
pub struct OA1P_A03_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1P_A03_W<'a> {
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
#[doc = "Field `OA1P_A13` reader - Opamp1 positive terminal P5"]
pub struct OA1P_A13_R(crate::FieldReader<bool, bool>);
impl OA1P_A13_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA1P_A13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA1P_A13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1P_A13` writer - Opamp1 positive terminal P5"]
pub struct OA1P_A13_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1P_A13_W<'a> {
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
#[doc = "Field `OA1P_A43` reader - Opamp1 positive terminal ctbbus1"]
pub struct OA1P_A43_R(crate::FieldReader<bool, bool>);
impl OA1P_A43_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA1P_A43_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA1P_A43_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1P_A43` writer - Opamp1 positive terminal ctbbus1"]
pub struct OA1P_A43_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1P_A43_W<'a> {
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
#[doc = "Field `OA1P_A73` reader - Opamp1 positive terminal to vref1"]
pub struct OA1P_A73_R(crate::FieldReader<bool, bool>);
impl OA1P_A73_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA1P_A73_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA1P_A73_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1P_A73` writer - Opamp1 positive terminal to vref1"]
pub struct OA1P_A73_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1P_A73_W<'a> {
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
#[doc = "Field `OA1M_A22` reader - Opamp1 negative terminal P4"]
pub struct OA1M_A22_R(crate::FieldReader<bool, bool>);
impl OA1M_A22_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA1M_A22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA1M_A22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1M_A22` writer - Opamp1 negative terminal P4"]
pub struct OA1M_A22_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1M_A22_W<'a> {
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
#[doc = "Field `OA1M_A82` reader - Opamp1 negative terminal Opamp1 output"]
pub struct OA1M_A82_R(crate::FieldReader<bool, bool>);
impl OA1M_A82_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA1M_A82_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA1M_A82_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1M_A82` writer - Opamp1 negative terminal Opamp1 output"]
pub struct OA1M_A82_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1M_A82_W<'a> {
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
#[doc = "Field `OA1O_D52` reader - Opamp1 output sarbus0 (ctbbus2 in CTB)"]
pub struct OA1O_D52_R(crate::FieldReader<bool, bool>);
impl OA1O_D52_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA1O_D52_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA1O_D52_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1O_D52` writer - Opamp1 output sarbus0 (ctbbus2 in CTB)"]
pub struct OA1O_D52_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1O_D52_W<'a> {
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
#[doc = "Field `OA1O_D62` reader - Opamp1 output sarbus1 (ctbbus3 in CTB)"]
pub struct OA1O_D62_R(crate::FieldReader<bool, bool>);
impl OA1O_D62_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA1O_D62_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA1O_D62_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1O_D62` writer - Opamp1 output sarbus1 (ctbbus3 in CTB)"]
pub struct OA1O_D62_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1O_D62_W<'a> {
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
#[doc = "Field `OA1O_D82` reader - Opamp1 output switch to short 1x with 10x drive"]
pub struct OA1O_D82_R(crate::FieldReader<bool, bool>);
impl OA1O_D82_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA1O_D82_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA1O_D82_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1O_D82` writer - Opamp1 output switch to short 1x with 10x drive"]
pub struct OA1O_D82_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1O_D82_W<'a> {
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
    #[doc = "Bit 0 - Opamp1 positive terminal amuxbusb"]
    #[inline(always)]
    pub fn oa1p_a03(&self) -> OA1P_A03_R {
        OA1P_A03_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Opamp1 positive terminal P5"]
    #[inline(always)]
    pub fn oa1p_a13(&self) -> OA1P_A13_R {
        OA1P_A13_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Opamp1 positive terminal ctbbus1"]
    #[inline(always)]
    pub fn oa1p_a43(&self) -> OA1P_A43_R {
        OA1P_A43_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Opamp1 positive terminal to vref1"]
    #[inline(always)]
    pub fn oa1p_a73(&self) -> OA1P_A73_R {
        OA1P_A73_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Opamp1 negative terminal P4"]
    #[inline(always)]
    pub fn oa1m_a22(&self) -> OA1M_A22_R {
        OA1M_A22_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Opamp1 negative terminal Opamp1 output"]
    #[inline(always)]
    pub fn oa1m_a82(&self) -> OA1M_A82_R {
        OA1M_A82_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Opamp1 output sarbus0 (ctbbus2 in CTB)"]
    #[inline(always)]
    pub fn oa1o_d52(&self) -> OA1O_D52_R {
        OA1O_D52_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Opamp1 output sarbus1 (ctbbus3 in CTB)"]
    #[inline(always)]
    pub fn oa1o_d62(&self) -> OA1O_D62_R {
        OA1O_D62_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Opamp1 output switch to short 1x with 10x drive"]
    #[inline(always)]
    pub fn oa1o_d82(&self) -> OA1O_D82_R {
        OA1O_D82_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Opamp1 positive terminal amuxbusb"]
    #[inline(always)]
    pub fn oa1p_a03(&mut self) -> OA1P_A03_W {
        OA1P_A03_W { w: self }
    }
    #[doc = "Bit 1 - Opamp1 positive terminal P5"]
    #[inline(always)]
    pub fn oa1p_a13(&mut self) -> OA1P_A13_W {
        OA1P_A13_W { w: self }
    }
    #[doc = "Bit 4 - Opamp1 positive terminal ctbbus1"]
    #[inline(always)]
    pub fn oa1p_a43(&mut self) -> OA1P_A43_W {
        OA1P_A43_W { w: self }
    }
    #[doc = "Bit 7 - Opamp1 positive terminal to vref1"]
    #[inline(always)]
    pub fn oa1p_a73(&mut self) -> OA1P_A73_W {
        OA1P_A73_W { w: self }
    }
    #[doc = "Bit 8 - Opamp1 negative terminal P4"]
    #[inline(always)]
    pub fn oa1m_a22(&mut self) -> OA1M_A22_W {
        OA1M_A22_W { w: self }
    }
    #[doc = "Bit 14 - Opamp1 negative terminal Opamp1 output"]
    #[inline(always)]
    pub fn oa1m_a82(&mut self) -> OA1M_A82_W {
        OA1M_A82_W { w: self }
    }
    #[doc = "Bit 18 - Opamp1 output sarbus0 (ctbbus2 in CTB)"]
    #[inline(always)]
    pub fn oa1o_d52(&mut self) -> OA1O_D52_W {
        OA1O_D52_W { w: self }
    }
    #[doc = "Bit 19 - Opamp1 output sarbus1 (ctbbus3 in CTB)"]
    #[inline(always)]
    pub fn oa1o_d62(&mut self) -> OA1O_D62_W {
        OA1O_D62_W { w: self }
    }
    #[doc = "Bit 21 - Opamp1 output switch to short 1x with 10x drive"]
    #[inline(always)]
    pub fn oa1o_d82(&mut self) -> OA1O_D82_W {
        OA1O_D82_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opamp1 switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa1_sw](index.html) module"]
pub struct OA1_SW_SPEC;
impl crate::RegisterSpec for OA1_SW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oa1_sw::R](R) reader structure"]
impl crate::Readable for OA1_SW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oa1_sw::W](W) writer structure"]
impl crate::Writable for OA1_SW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OA1_SW to value 0"]
impl crate::Resettable for OA1_SW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
