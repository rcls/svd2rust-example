#[doc = "Register `EFUSE_TIM_CTRL3` reader"]
pub struct R(crate::R<EFUSE_TIM_CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSE_TIM_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSE_TIM_CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSE_TIM_CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSE_TIM_CTRL3` writer"]
pub struct W(crate::W<EFUSE_TIM_CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSE_TIM_CTRL3_SPEC>;
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
impl From<crate::W<EFUSE_TIM_CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSE_TIM_CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGM_SCLK_SETUP_TIME` reader - PGM to SCLK setup time (TS_PGM) PGM_SCLK_SETUP_TIME <CS_SCLK_SETUP_TIME"]
pub struct PGM_SCLK_SETUP_TIME_R(crate::FieldReader<u8, u8>);
impl PGM_SCLK_SETUP_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        PGM_SCLK_SETUP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_SCLK_SETUP_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM_SCLK_SETUP_TIME` writer - PGM to SCLK setup time (TS_PGM) PGM_SCLK_SETUP_TIME <CS_SCLK_SETUP_TIME"]
pub struct PGM_SCLK_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_SCLK_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PGM_SCLK_HOLD_TIME` reader - PGM to SCLK hold time (TH_PGM)"]
pub struct PGM_SCLK_HOLD_TIME_R(crate::FieldReader<u8, u8>);
impl PGM_SCLK_HOLD_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        PGM_SCLK_HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_SCLK_HOLD_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM_SCLK_HOLD_TIME` writer - PGM to SCLK hold time (TH_PGM)"]
pub struct PGM_SCLK_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_SCLK_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `AVDD_CS_SETUP_TIME` reader - AVDD to CS setup time into program mode (TSP_AVDD_CS)"]
pub struct AVDD_CS_SETUP_TIME_R(crate::FieldReader<u8, u8>);
impl AVDD_CS_SETUP_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        AVDD_CS_SETUP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVDD_CS_SETUP_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVDD_CS_SETUP_TIME` writer - AVDD to CS setup time into program mode (TSP_AVDD_CS)"]
pub struct AVDD_CS_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> AVDD_CS_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `AVDD_CS_HOLD_TIME` reader - AVDD to CS hold time out of program mode (THP_AVDD_CS)"]
pub struct AVDD_CS_HOLD_TIME_R(crate::FieldReader<u8, u8>);
impl AVDD_CS_HOLD_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        AVDD_CS_HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVDD_CS_HOLD_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVDD_CS_HOLD_TIME` writer - AVDD to CS hold time out of program mode (THP_AVDD_CS)"]
pub struct AVDD_CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> AVDD_CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PGM to SCLK setup time (TS_PGM) PGM_SCLK_SETUP_TIME <CS_SCLK_SETUP_TIME"]
    #[inline(always)]
    pub fn pgm_sclk_setup_time(&self) -> PGM_SCLK_SETUP_TIME_R {
        PGM_SCLK_SETUP_TIME_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PGM to SCLK hold time (TH_PGM)"]
    #[inline(always)]
    pub fn pgm_sclk_hold_time(&self) -> PGM_SCLK_HOLD_TIME_R {
        PGM_SCLK_HOLD_TIME_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - AVDD to CS setup time into program mode (TSP_AVDD_CS)"]
    #[inline(always)]
    pub fn avdd_cs_setup_time(&self) -> AVDD_CS_SETUP_TIME_R {
        AVDD_CS_SETUP_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - AVDD to CS hold time out of program mode (THP_AVDD_CS)"]
    #[inline(always)]
    pub fn avdd_cs_hold_time(&self) -> AVDD_CS_HOLD_TIME_R {
        AVDD_CS_HOLD_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PGM to SCLK setup time (TS_PGM) PGM_SCLK_SETUP_TIME <CS_SCLK_SETUP_TIME"]
    #[inline(always)]
    pub fn pgm_sclk_setup_time(&mut self) -> PGM_SCLK_SETUP_TIME_W {
        PGM_SCLK_SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 4:7 - PGM to SCLK hold time (TH_PGM)"]
    #[inline(always)]
    pub fn pgm_sclk_hold_time(&mut self) -> PGM_SCLK_HOLD_TIME_W {
        PGM_SCLK_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 8:15 - AVDD to CS setup time into program mode (TSP_AVDD_CS)"]
    #[inline(always)]
    pub fn avdd_cs_setup_time(&mut self) -> AVDD_CS_SETUP_TIME_W {
        AVDD_CS_SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 16:23 - AVDD to CS hold time out of program mode (THP_AVDD_CS)"]
    #[inline(always)]
    pub fn avdd_cs_hold_time(&mut self) -> AVDD_CS_HOLD_TIME_W {
        AVDD_CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EFUSE timing control Register (for Program)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_tim_ctrl3](index.html) module"]
pub struct EFUSE_TIM_CTRL3_SPEC;
impl crate::RegisterSpec for EFUSE_TIM_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuse_tim_ctrl3::R](R) reader structure"]
impl crate::Readable for EFUSE_TIM_CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuse_tim_ctrl3::W](W) writer structure"]
impl crate::Writable for EFUSE_TIM_CTRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EFUSE_TIM_CTRL3 to value 0x003a_3a11"]
impl crate::Resettable for EFUSE_TIM_CTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x003a_3a11
    }
}
