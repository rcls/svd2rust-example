#[doc = "Register `EFUSE_TIM_CTRL2` reader"]
pub struct R(crate::R<EFUSE_TIM_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSE_TIM_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSE_TIM_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSE_TIM_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSE_TIM_CTRL2` writer"]
pub struct W(crate::W<EFUSE_TIM_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSE_TIM_CTRL2_SPEC>;
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
impl From<crate::W<EFUSE_TIM_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSE_TIM_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_SAMPLE_TIME` reader - This register specifies the time for data sampling from SCLK HIGH (TCKDQ_H)"]
pub struct DATA_SAMPLE_TIME_R(crate::FieldReader<u8, u8>);
impl DATA_SAMPLE_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_SAMPLE_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_SAMPLE_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_SAMPLE_TIME` writer - This register specifies the time for data sampling from SCLK HIGH (TCKDQ_H)"]
pub struct DATA_SAMPLE_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_SAMPLE_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DOUT_CS_HOLD_TIME` reader - Wait time DOUT to CS hold time out of read mode (TDQH)"]
pub struct DOUT_CS_HOLD_TIME_R(crate::FieldReader<u8, u8>);
impl DOUT_CS_HOLD_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        DOUT_CS_HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT_CS_HOLD_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT_CS_HOLD_TIME` writer - Wait time DOUT to CS hold time out of read mode (TDQH)"]
pub struct DOUT_CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT_CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register specifies the time for data sampling from SCLK HIGH (TCKDQ_H)"]
    #[inline(always)]
    pub fn data_sample_time(&self) -> DATA_SAMPLE_TIME_R {
        DATA_SAMPLE_TIME_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Wait time DOUT to CS hold time out of read mode (TDQH)"]
    #[inline(always)]
    pub fn dout_cs_hold_time(&self) -> DOUT_CS_HOLD_TIME_R {
        DOUT_CS_HOLD_TIME_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register specifies the time for data sampling from SCLK HIGH (TCKDQ_H)"]
    #[inline(always)]
    pub fn data_sample_time(&mut self) -> DATA_SAMPLE_TIME_W {
        DATA_SAMPLE_TIME_W { w: self }
    }
    #[doc = "Bits 8:11 - Wait time DOUT to CS hold time out of read mode (TDQH)"]
    #[inline(always)]
    pub fn dout_cs_hold_time(&mut self) -> DOUT_CS_HOLD_TIME_W {
        DOUT_CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EFUSE timing control Register (for Read)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_tim_ctrl2](index.html) module"]
pub struct EFUSE_TIM_CTRL2_SPEC;
impl crate::RegisterSpec for EFUSE_TIM_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuse_tim_ctrl2::R](R) reader structure"]
impl crate::Readable for EFUSE_TIM_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuse_tim_ctrl2::W](W) writer structure"]
impl crate::Writable for EFUSE_TIM_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EFUSE_TIM_CTRL2 to value 0x0102"]
impl crate::Resettable for EFUSE_TIM_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0102
    }
}
