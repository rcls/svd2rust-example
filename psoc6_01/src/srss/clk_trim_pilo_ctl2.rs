#[doc = "Register `CLK_TRIM_PILO_CTL2` reader"]
pub struct R(crate::R<CLK_TRIM_PILO_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TRIM_PILO_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TRIM_PILO_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TRIM_PILO_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TRIM_PILO_CTL2` writer"]
pub struct W(crate::W<CLK_TRIM_PILO_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TRIM_PILO_CTL2_SPEC>;
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
impl From<crate::W<CLK_TRIM_PILO_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TRIM_PILO_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PILO_VREF_TRIM` reader - Trim for voltage reference"]
pub struct PILO_VREF_TRIM_R(crate::FieldReader<u8, u8>);
impl PILO_VREF_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PILO_VREF_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PILO_VREF_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PILO_VREF_TRIM` writer - Trim for voltage reference"]
pub struct PILO_VREF_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_VREF_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PILO_IREFBM_TRIM` reader - Trim for beta-multiplier current reference"]
pub struct PILO_IREFBM_TRIM_R(crate::FieldReader<u8, u8>);
impl PILO_IREFBM_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PILO_IREFBM_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PILO_IREFBM_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PILO_IREFBM_TRIM` writer - Trim for beta-multiplier current reference"]
pub struct PILO_IREFBM_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_IREFBM_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `PILO_IREF_TRIM` reader - Trim for current reference"]
pub struct PILO_IREF_TRIM_R(crate::FieldReader<u8, u8>);
impl PILO_IREF_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PILO_IREF_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PILO_IREF_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PILO_IREF_TRIM` writer - Trim for current reference"]
pub struct PILO_IREF_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_IREF_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Trim for voltage reference"]
    #[inline(always)]
    pub fn pilo_vref_trim(&self) -> PILO_VREF_TRIM_R {
        PILO_VREF_TRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Trim for beta-multiplier current reference"]
    #[inline(always)]
    pub fn pilo_irefbm_trim(&self) -> PILO_IREFBM_TRIM_R {
        PILO_IREFBM_TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Trim for current reference"]
    #[inline(always)]
    pub fn pilo_iref_trim(&self) -> PILO_IREF_TRIM_R {
        PILO_IREF_TRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trim for voltage reference"]
    #[inline(always)]
    pub fn pilo_vref_trim(&mut self) -> PILO_VREF_TRIM_W {
        PILO_VREF_TRIM_W { w: self }
    }
    #[doc = "Bits 8:12 - Trim for beta-multiplier current reference"]
    #[inline(always)]
    pub fn pilo_irefbm_trim(&mut self) -> PILO_IREFBM_TRIM_W {
        PILO_IREFBM_TRIM_W { w: self }
    }
    #[doc = "Bits 16:23 - Trim for current reference"]
    #[inline(always)]
    pub fn pilo_iref_trim(&mut self) -> PILO_IREF_TRIM_W {
        PILO_IREF_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PILO Trim Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_pilo_ctl2](index.html) module"]
pub struct CLK_TRIM_PILO_CTL2_SPEC;
impl crate::RegisterSpec for CLK_TRIM_PILO_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_trim_pilo_ctl2::R](R) reader structure"]
impl crate::Readable for CLK_TRIM_PILO_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_trim_pilo_ctl2::W](W) writer structure"]
impl crate::Writable for CLK_TRIM_PILO_CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_TRIM_PILO_CTL2 to value 0x00da_10e0"]
impl crate::Resettable for CLK_TRIM_PILO_CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00da_10e0
    }
}
