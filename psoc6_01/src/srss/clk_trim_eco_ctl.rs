#[doc = "Register `CLK_TRIM_ECO_CTL` reader"]
pub struct R(crate::R<CLK_TRIM_ECO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TRIM_ECO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TRIM_ECO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TRIM_ECO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TRIM_ECO_CTL` writer"]
pub struct W(crate::W<CLK_TRIM_ECO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TRIM_ECO_CTL_SPEC>;
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
impl From<crate::W<CLK_TRIM_ECO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TRIM_ECO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTRIM` reader - Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
pub struct WDTRIM_R(crate::FieldReader<u8, u8>);
impl WDTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTRIM` writer - Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
pub struct WDTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `ATRIM` reader - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
pub struct ATRIM_R(crate::FieldReader<u8, u8>);
impl ATRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATRIM` writer - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
pub struct ATRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ATRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `FTRIM` reader - Filter Trim - 3rd harmonic oscillation"]
pub struct FTRIM_R(crate::FieldReader<u8, u8>);
impl FTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTRIM` writer - Filter Trim - 3rd harmonic oscillation"]
pub struct FTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `RTRIM` reader - Feedback resistor Trim"]
pub struct RTRIM_R(crate::FieldReader<u8, u8>);
impl RTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTRIM` writer - Feedback resistor Trim"]
pub struct RTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `GTRIM` reader - Gain Trim - Startup time"]
pub struct GTRIM_R(crate::FieldReader<u8, u8>);
impl GTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        GTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTRIM` writer - Gain Trim - Startup time"]
pub struct GTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> GTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `ITRIM` reader - Current Trim"]
pub struct ITRIM_R(crate::FieldReader<u8, u8>);
impl ITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITRIM` writer - Current Trim"]
pub struct ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
    #[inline(always)]
    pub fn wdtrim(&self) -> WDTRIM_R {
        WDTRIM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
    #[inline(always)]
    pub fn atrim(&self) -> ATRIM_R {
        ATRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Filter Trim - 3rd harmonic oscillation"]
    #[inline(always)]
    pub fn ftrim(&self) -> FTRIM_R {
        FTRIM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Feedback resistor Trim"]
    #[inline(always)]
    pub fn rtrim(&self) -> RTRIM_R {
        RTRIM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Gain Trim - Startup time"]
    #[inline(always)]
    pub fn gtrim(&self) -> GTRIM_R {
        GTRIM_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:21 - Current Trim"]
    #[inline(always)]
    pub fn itrim(&self) -> ITRIM_R {
        ITRIM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
    #[inline(always)]
    pub fn wdtrim(&mut self) -> WDTRIM_W {
        WDTRIM_W { w: self }
    }
    #[doc = "Bits 4:7 - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
    #[inline(always)]
    pub fn atrim(&mut self) -> ATRIM_W {
        ATRIM_W { w: self }
    }
    #[doc = "Bits 8:9 - Filter Trim - 3rd harmonic oscillation"]
    #[inline(always)]
    pub fn ftrim(&mut self) -> FTRIM_W {
        FTRIM_W { w: self }
    }
    #[doc = "Bits 10:11 - Feedback resistor Trim"]
    #[inline(always)]
    pub fn rtrim(&mut self) -> RTRIM_W {
        RTRIM_W { w: self }
    }
    #[doc = "Bits 12:13 - Gain Trim - Startup time"]
    #[inline(always)]
    pub fn gtrim(&mut self) -> GTRIM_W {
        GTRIM_W { w: self }
    }
    #[doc = "Bits 16:21 - Current Trim"]
    #[inline(always)]
    pub fn itrim(&mut self) -> ITRIM_W {
        ITRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECO Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_eco_ctl](index.html) module"]
pub struct CLK_TRIM_ECO_CTL_SPEC;
impl crate::RegisterSpec for CLK_TRIM_ECO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_trim_eco_ctl::R](R) reader structure"]
impl crate::Readable for CLK_TRIM_ECO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_trim_eco_ctl::W](W) writer structure"]
impl crate::Writable for CLK_TRIM_ECO_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_TRIM_ECO_CTL to value 0x001f_0003"]
impl crate::Resettable for CLK_TRIM_ECO_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x001f_0003
    }
}
