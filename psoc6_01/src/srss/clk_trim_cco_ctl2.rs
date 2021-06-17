#[doc = "Register `CLK_TRIM_CCO_CTL2` reader"]
pub struct R(crate::R<CLK_TRIM_CCO_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TRIM_CCO_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TRIM_CCO_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TRIM_CCO_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TRIM_CCO_CTL2` writer"]
pub struct W(crate::W<CLK_TRIM_CCO_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TRIM_CCO_CTL2_SPEC>;
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
impl From<crate::W<CLK_TRIM_CCO_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TRIM_CCO_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCO_FCTRIM1` reader - CCO frequency 1st range calibration"]
pub struct CCO_FCTRIM1_R(crate::FieldReader<u8, u8>);
impl CCO_FCTRIM1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCO_FCTRIM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCO_FCTRIM1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCO_FCTRIM1` writer - CCO frequency 1st range calibration"]
pub struct CCO_FCTRIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_FCTRIM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `CCO_FCTRIM2` reader - CCO frequency 2nd range calibration"]
pub struct CCO_FCTRIM2_R(crate::FieldReader<u8, u8>);
impl CCO_FCTRIM2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCO_FCTRIM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCO_FCTRIM2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCO_FCTRIM2` writer - CCO frequency 2nd range calibration"]
pub struct CCO_FCTRIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_FCTRIM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `CCO_FCTRIM3` reader - CCO frequency 3rd range calibration"]
pub struct CCO_FCTRIM3_R(crate::FieldReader<u8, u8>);
impl CCO_FCTRIM3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCO_FCTRIM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCO_FCTRIM3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCO_FCTRIM3` writer - CCO frequency 3rd range calibration"]
pub struct CCO_FCTRIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_FCTRIM3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `CCO_FCTRIM4` reader - CCO frequency 4th range calibration"]
pub struct CCO_FCTRIM4_R(crate::FieldReader<u8, u8>);
impl CCO_FCTRIM4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCO_FCTRIM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCO_FCTRIM4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCO_FCTRIM4` writer - CCO frequency 4th range calibration"]
pub struct CCO_FCTRIM4_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_FCTRIM4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `CCO_FCTRIM5` reader - CCO frequency 5th range calibration"]
pub struct CCO_FCTRIM5_R(crate::FieldReader<u8, u8>);
impl CCO_FCTRIM5_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCO_FCTRIM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCO_FCTRIM5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCO_FCTRIM5` writer - CCO frequency 5th range calibration"]
pub struct CCO_FCTRIM5_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_FCTRIM5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - CCO frequency 1st range calibration"]
    #[inline(always)]
    pub fn cco_fctrim1(&self) -> CCO_FCTRIM1_R {
        CCO_FCTRIM1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - CCO frequency 2nd range calibration"]
    #[inline(always)]
    pub fn cco_fctrim2(&self) -> CCO_FCTRIM2_R {
        CCO_FCTRIM2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - CCO frequency 3rd range calibration"]
    #[inline(always)]
    pub fn cco_fctrim3(&self) -> CCO_FCTRIM3_R {
        CCO_FCTRIM3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - CCO frequency 4th range calibration"]
    #[inline(always)]
    pub fn cco_fctrim4(&self) -> CCO_FCTRIM4_R {
        CCO_FCTRIM4_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - CCO frequency 5th range calibration"]
    #[inline(always)]
    pub fn cco_fctrim5(&self) -> CCO_FCTRIM5_R {
        CCO_FCTRIM5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - CCO frequency 1st range calibration"]
    #[inline(always)]
    pub fn cco_fctrim1(&mut self) -> CCO_FCTRIM1_W {
        CCO_FCTRIM1_W { w: self }
    }
    #[doc = "Bits 5:9 - CCO frequency 2nd range calibration"]
    #[inline(always)]
    pub fn cco_fctrim2(&mut self) -> CCO_FCTRIM2_W {
        CCO_FCTRIM2_W { w: self }
    }
    #[doc = "Bits 10:14 - CCO frequency 3rd range calibration"]
    #[inline(always)]
    pub fn cco_fctrim3(&mut self) -> CCO_FCTRIM3_W {
        CCO_FCTRIM3_W { w: self }
    }
    #[doc = "Bits 15:19 - CCO frequency 4th range calibration"]
    #[inline(always)]
    pub fn cco_fctrim4(&mut self) -> CCO_FCTRIM4_W {
        CCO_FCTRIM4_W { w: self }
    }
    #[doc = "Bits 20:24 - CCO frequency 5th range calibration"]
    #[inline(always)]
    pub fn cco_fctrim5(&mut self) -> CCO_FCTRIM5_W {
        CCO_FCTRIM5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCO Trim Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_cco_ctl2](index.html) module"]
pub struct CLK_TRIM_CCO_CTL2_SPEC;
impl crate::RegisterSpec for CLK_TRIM_CCO_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_trim_cco_ctl2::R](R) reader structure"]
impl crate::Readable for CLK_TRIM_CCO_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_trim_cco_ctl2::W](W) writer structure"]
impl crate::Writable for CLK_TRIM_CCO_CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_TRIM_CCO_CTL2 to value 0x0088_4110"]
impl crate::Resettable for CLK_TRIM_CCO_CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0088_4110
    }
}
