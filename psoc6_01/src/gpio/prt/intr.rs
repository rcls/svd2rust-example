#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDGE0` reader - Edge detect for IO pin 0 '0': No edge was detected on pin. '1': An edge was detected on pin."]
pub struct EDGE0_R(crate::FieldReader<bool, bool>);
impl EDGE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE0` writer - Edge detect for IO pin 0 '0': No edge was detected on pin. '1': An edge was detected on pin."]
pub struct EDGE0_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE0_W<'a> {
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
#[doc = "Field `EDGE1` reader - Edge detect for IO pin 1"]
pub struct EDGE1_R(crate::FieldReader<bool, bool>);
impl EDGE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE1` writer - Edge detect for IO pin 1"]
pub struct EDGE1_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE1_W<'a> {
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
#[doc = "Field `EDGE2` reader - Edge detect for IO pin 2"]
pub struct EDGE2_R(crate::FieldReader<bool, bool>);
impl EDGE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE2` writer - Edge detect for IO pin 2"]
pub struct EDGE2_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE2_W<'a> {
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
#[doc = "Field `EDGE3` reader - Edge detect for IO pin 3"]
pub struct EDGE3_R(crate::FieldReader<bool, bool>);
impl EDGE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE3` writer - Edge detect for IO pin 3"]
pub struct EDGE3_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE3_W<'a> {
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
#[doc = "Field `EDGE4` reader - Edge detect for IO pin 4"]
pub struct EDGE4_R(crate::FieldReader<bool, bool>);
impl EDGE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE4` writer - Edge detect for IO pin 4"]
pub struct EDGE4_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE4_W<'a> {
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
#[doc = "Field `EDGE5` reader - Edge detect for IO pin 5"]
pub struct EDGE5_R(crate::FieldReader<bool, bool>);
impl EDGE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE5` writer - Edge detect for IO pin 5"]
pub struct EDGE5_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE5_W<'a> {
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
#[doc = "Field `EDGE6` reader - Edge detect for IO pin 6"]
pub struct EDGE6_R(crate::FieldReader<bool, bool>);
impl EDGE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE6` writer - Edge detect for IO pin 6"]
pub struct EDGE6_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE6_W<'a> {
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
#[doc = "Field `EDGE7` reader - Edge detect for IO pin 7"]
pub struct EDGE7_R(crate::FieldReader<bool, bool>);
impl EDGE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE7` writer - Edge detect for IO pin 7"]
pub struct EDGE7_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE7_W<'a> {
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
#[doc = "Field `FLT_EDGE` reader - Edge detected on filtered pin selected by INTR_CFG.FLT_SEL"]
pub struct FLT_EDGE_R(crate::FieldReader<bool, bool>);
impl FLT_EDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_EDGE` writer - Edge detected on filtered pin selected by INTR_CFG.FLT_SEL"]
pub struct FLT_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_EDGE_W<'a> {
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
#[doc = "Field `IN_IN0` reader - IO pin state for pin 0"]
pub struct IN_IN0_R(crate::FieldReader<bool, bool>);
impl IN_IN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_IN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_IN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_IN1` reader - IO pin state for pin 1"]
pub struct IN_IN1_R(crate::FieldReader<bool, bool>);
impl IN_IN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_IN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_IN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_IN2` reader - IO pin state for pin 2"]
pub struct IN_IN2_R(crate::FieldReader<bool, bool>);
impl IN_IN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_IN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_IN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_IN3` reader - IO pin state for pin 3"]
pub struct IN_IN3_R(crate::FieldReader<bool, bool>);
impl IN_IN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_IN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_IN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_IN4` reader - IO pin state for pin 4"]
pub struct IN_IN4_R(crate::FieldReader<bool, bool>);
impl IN_IN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_IN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_IN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_IN5` reader - IO pin state for pin 5"]
pub struct IN_IN5_R(crate::FieldReader<bool, bool>);
impl IN_IN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_IN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_IN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_IN6` reader - IO pin state for pin 6"]
pub struct IN_IN6_R(crate::FieldReader<bool, bool>);
impl IN_IN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_IN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_IN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_IN7` reader - IO pin state for pin 7"]
pub struct IN_IN7_R(crate::FieldReader<bool, bool>);
impl IN_IN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_IN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_IN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_IN_IN` reader - Filtered pin state for pin selected by INTR_CFG.FLT_SEL"]
pub struct FLT_IN_IN_R(crate::FieldReader<bool, bool>);
impl FLT_IN_IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT_IN_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT_IN_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Edge detect for IO pin 0 '0': No edge was detected on pin. '1': An edge was detected on pin."]
    #[inline(always)]
    pub fn edge0(&self) -> EDGE0_R {
        EDGE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Edge detect for IO pin 1"]
    #[inline(always)]
    pub fn edge1(&self) -> EDGE1_R {
        EDGE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Edge detect for IO pin 2"]
    #[inline(always)]
    pub fn edge2(&self) -> EDGE2_R {
        EDGE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Edge detect for IO pin 3"]
    #[inline(always)]
    pub fn edge3(&self) -> EDGE3_R {
        EDGE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Edge detect for IO pin 4"]
    #[inline(always)]
    pub fn edge4(&self) -> EDGE4_R {
        EDGE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Edge detect for IO pin 5"]
    #[inline(always)]
    pub fn edge5(&self) -> EDGE5_R {
        EDGE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Edge detect for IO pin 6"]
    #[inline(always)]
    pub fn edge6(&self) -> EDGE6_R {
        EDGE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Edge detect for IO pin 7"]
    #[inline(always)]
    pub fn edge7(&self) -> EDGE7_R {
        EDGE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Edge detected on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge(&self) -> FLT_EDGE_R {
        FLT_EDGE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IO pin state for pin 0"]
    #[inline(always)]
    pub fn in_in0(&self) -> IN_IN0_R {
        IN_IN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IO pin state for pin 1"]
    #[inline(always)]
    pub fn in_in1(&self) -> IN_IN1_R {
        IN_IN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IO pin state for pin 2"]
    #[inline(always)]
    pub fn in_in2(&self) -> IN_IN2_R {
        IN_IN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IO pin state for pin 3"]
    #[inline(always)]
    pub fn in_in3(&self) -> IN_IN3_R {
        IN_IN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IO pin state for pin 4"]
    #[inline(always)]
    pub fn in_in4(&self) -> IN_IN4_R {
        IN_IN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IO pin state for pin 5"]
    #[inline(always)]
    pub fn in_in5(&self) -> IN_IN5_R {
        IN_IN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IO pin state for pin 6"]
    #[inline(always)]
    pub fn in_in6(&self) -> IN_IN6_R {
        IN_IN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - IO pin state for pin 7"]
    #[inline(always)]
    pub fn in_in7(&self) -> IN_IN7_R {
        IN_IN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filtered pin state for pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_in_in(&self) -> FLT_IN_IN_R {
        FLT_IN_IN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Edge detect for IO pin 0 '0': No edge was detected on pin. '1': An edge was detected on pin."]
    #[inline(always)]
    pub fn edge0(&mut self) -> EDGE0_W {
        EDGE0_W { w: self }
    }
    #[doc = "Bit 1 - Edge detect for IO pin 1"]
    #[inline(always)]
    pub fn edge1(&mut self) -> EDGE1_W {
        EDGE1_W { w: self }
    }
    #[doc = "Bit 2 - Edge detect for IO pin 2"]
    #[inline(always)]
    pub fn edge2(&mut self) -> EDGE2_W {
        EDGE2_W { w: self }
    }
    #[doc = "Bit 3 - Edge detect for IO pin 3"]
    #[inline(always)]
    pub fn edge3(&mut self) -> EDGE3_W {
        EDGE3_W { w: self }
    }
    #[doc = "Bit 4 - Edge detect for IO pin 4"]
    #[inline(always)]
    pub fn edge4(&mut self) -> EDGE4_W {
        EDGE4_W { w: self }
    }
    #[doc = "Bit 5 - Edge detect for IO pin 5"]
    #[inline(always)]
    pub fn edge5(&mut self) -> EDGE5_W {
        EDGE5_W { w: self }
    }
    #[doc = "Bit 6 - Edge detect for IO pin 6"]
    #[inline(always)]
    pub fn edge6(&mut self) -> EDGE6_W {
        EDGE6_W { w: self }
    }
    #[doc = "Bit 7 - Edge detect for IO pin 7"]
    #[inline(always)]
    pub fn edge7(&mut self) -> EDGE7_W {
        EDGE7_W { w: self }
    }
    #[doc = "Bit 8 - Edge detected on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge(&mut self) -> FLT_EDGE_W {
        FLT_EDGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
