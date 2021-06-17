#[doc = "Register `EFUSE_TIM_CTRL1` reader"]
pub struct R(crate::R<EFUSE_TIM_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSE_TIM_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSE_TIM_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSE_TIM_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSE_TIM_CTRL1` writer"]
pub struct W(crate::W<EFUSE_TIM_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSE_TIM_CTRL1_SPEC>;
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
impl From<crate::W<EFUSE_TIM_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSE_TIM_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLK_HIGH` reader - Decides the duration of TPGM (in Program mode) or TCKHP (in Read mode) TPGM: Burning Time TCKHP : SCLK high Period"]
pub struct SCLK_HIGH_R(crate::FieldReader<u8, u8>);
impl SCLK_HIGH_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_HIGH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_HIGH` writer - Decides the duration of TPGM (in Program mode) or TCKHP (in Read mode) TPGM: Burning Time TCKHP : SCLK high Period"]
pub struct SCLK_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SCLK_LOW` reader - Duration of SCLK LOW (TCLKP_R) or TCKLP_P"]
pub struct SCLK_LOW_R(crate::FieldReader<u8, u8>);
impl SCLK_LOW_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_LOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_LOW` writer - Duration of SCLK LOW (TCLKP_R) or TCKLP_P"]
pub struct SCLK_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CS_SCLK_SETUP_TIME` reader - This register specifies the setup time between CS and SCLK (TSR_CLK)"]
pub struct CS_SCLK_SETUP_TIME_R(crate::FieldReader<u8, u8>);
impl CS_SCLK_SETUP_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS_SCLK_SETUP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_SCLK_SETUP_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_SCLK_SETUP_TIME` writer - This register specifies the setup time between CS and SCLK (TSR_CLK)"]
pub struct CS_SCLK_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SCLK_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `CS_SCLK_HOLD_TIME` reader - This register specifies the hold time between CS and SCLK (THR_CLK)"]
pub struct CS_SCLK_HOLD_TIME_R(crate::FieldReader<u8, u8>);
impl CS_SCLK_HOLD_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS_SCLK_HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_SCLK_HOLD_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_SCLK_HOLD_TIME` writer - This register specifies the hold time between CS and SCLK (THR_CLK)"]
pub struct CS_SCLK_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SCLK_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `RW_CS_SETUP_TIME` reader - This field decides setup time between RW & CS (TSR_RW: in read mode) or RW & AVDD (TSP_RW: in Program mode). TSR_RW: RW to CS setup time into Read mode TSP_RW: RW to AVDD setup time into program mode"]
pub struct RW_CS_SETUP_TIME_R(crate::FieldReader<u8, u8>);
impl RW_CS_SETUP_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        RW_CS_SETUP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RW_CS_SETUP_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RW_CS_SETUP_TIME` writer - This field decides setup time between RW & CS (TSR_RW: in read mode) or RW & AVDD (TSP_RW: in Program mode). TSR_RW: RW to CS setup time into Read mode TSP_RW: RW to AVDD setup time into program mode"]
pub struct RW_CS_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> RW_CS_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `RW_CS_HOLD_TIME` reader - This field decides hold time between RW & CS (THR_RW: in read mode) or RW & AVDD (THP_RW: in Program mode). THR_RW: RW to CS hold time out of Read mode THP_RW: RW to AVDD hold time out of program mode"]
pub struct RW_CS_HOLD_TIME_R(crate::FieldReader<u8, u8>);
impl RW_CS_HOLD_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        RW_CS_HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RW_CS_HOLD_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RW_CS_HOLD_TIME` writer - This field decides hold time between RW & CS (THR_RW: in read mode) or RW & AVDD (THP_RW: in Program mode). THR_RW: RW to CS hold time out of Read mode THP_RW: RW to AVDD hold time out of program mode"]
pub struct RW_CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> RW_CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Decides the duration of TPGM (in Program mode) or TCKHP (in Read mode) TPGM: Burning Time TCKHP : SCLK high Period"]
    #[inline(always)]
    pub fn sclk_high(&self) -> SCLK_HIGH_R {
        SCLK_HIGH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Duration of SCLK LOW (TCLKP_R) or TCKLP_P"]
    #[inline(always)]
    pub fn sclk_low(&self) -> SCLK_LOW_R {
        SCLK_LOW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - This register specifies the setup time between CS and SCLK (TSR_CLK)"]
    #[inline(always)]
    pub fn cs_sclk_setup_time(&self) -> CS_SCLK_SETUP_TIME_R {
        CS_SCLK_SETUP_TIME_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - This register specifies the hold time between CS and SCLK (THR_CLK)"]
    #[inline(always)]
    pub fn cs_sclk_hold_time(&self) -> CS_SCLK_HOLD_TIME_R {
        CS_SCLK_HOLD_TIME_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - This field decides setup time between RW & CS (TSR_RW: in read mode) or RW & AVDD (TSP_RW: in Program mode). TSR_RW: RW to CS setup time into Read mode TSP_RW: RW to AVDD setup time into program mode"]
    #[inline(always)]
    pub fn rw_cs_setup_time(&self) -> RW_CS_SETUP_TIME_R {
        RW_CS_SETUP_TIME_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - This field decides hold time between RW & CS (THR_RW: in read mode) or RW & AVDD (THP_RW: in Program mode). THR_RW: RW to CS hold time out of Read mode THP_RW: RW to AVDD hold time out of program mode"]
    #[inline(always)]
    pub fn rw_cs_hold_time(&self) -> RW_CS_HOLD_TIME_R {
        RW_CS_HOLD_TIME_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Decides the duration of TPGM (in Program mode) or TCKHP (in Read mode) TPGM: Burning Time TCKHP : SCLK high Period"]
    #[inline(always)]
    pub fn sclk_high(&mut self) -> SCLK_HIGH_W {
        SCLK_HIGH_W { w: self }
    }
    #[doc = "Bits 8:15 - Duration of SCLK LOW (TCLKP_R) or TCKLP_P"]
    #[inline(always)]
    pub fn sclk_low(&mut self) -> SCLK_LOW_W {
        SCLK_LOW_W { w: self }
    }
    #[doc = "Bits 16:19 - This register specifies the setup time between CS and SCLK (TSR_CLK)"]
    #[inline(always)]
    pub fn cs_sclk_setup_time(&mut self) -> CS_SCLK_SETUP_TIME_W {
        CS_SCLK_SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 20:23 - This register specifies the hold time between CS and SCLK (THR_CLK)"]
    #[inline(always)]
    pub fn cs_sclk_hold_time(&mut self) -> CS_SCLK_HOLD_TIME_W {
        CS_SCLK_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 24:27 - This field decides setup time between RW & CS (TSR_RW: in read mode) or RW & AVDD (TSP_RW: in Program mode). TSR_RW: RW to CS setup time into Read mode TSP_RW: RW to AVDD setup time into program mode"]
    #[inline(always)]
    pub fn rw_cs_setup_time(&mut self) -> RW_CS_SETUP_TIME_W {
        RW_CS_SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 28:31 - This field decides hold time between RW & CS (THR_RW: in read mode) or RW & AVDD (THP_RW: in Program mode). THR_RW: RW to CS hold time out of Read mode THP_RW: RW to AVDD hold time out of program mode"]
    #[inline(always)]
    pub fn rw_cs_hold_time(&mut self) -> RW_CS_HOLD_TIME_W {
        RW_CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EFUSE timing control register (common for Program and Read modes)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_tim_ctrl1](index.html) module"]
pub struct EFUSE_TIM_CTRL1_SPEC;
impl crate::RegisterSpec for EFUSE_TIM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuse_tim_ctrl1::R](R) reader structure"]
impl crate::Readable for EFUSE_TIM_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuse_tim_ctrl1::W](W) writer structure"]
impl crate::Writable for EFUSE_TIM_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EFUSE_TIM_CTRL1 to value 0x1112_01c0"]
impl crate::Resettable for EFUSE_TIM_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1112_01c0
    }
}
