#[doc = "Register `CLOCK_CTL` reader"]
pub struct R(crate::R<CLOCK_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_CTL` writer"]
pub struct W(crate::W<CLOCK_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_CTL_SPEC>;
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
impl From<crate::W<CLOCK_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLOCK_DIV` reader - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
pub struct CLOCK_DIV_R(crate::FieldReader<u8, u8>);
impl CLOCK_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLOCK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCK_DIV` writer - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
pub struct CLOCK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `CLOCK_SEL` reader - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
pub struct CLOCK_SEL_R(crate::FieldReader<bool, bool>);
impl CLOCK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLOCK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCK_SEL` writer - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
pub struct CLOCK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_SEL_W<'a> {
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
impl R {
    #[doc = "Bits 0:5 - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
    #[inline(always)]
    pub fn clock_div(&self) -> CLOCK_DIV_R {
        CLOCK_DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
    #[inline(always)]
    pub fn clock_div(&mut self) -> CLOCK_DIV_W {
        CLOCK_DIV_W { w: self }
    }
    #[doc = "Bit 8 - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
    #[inline(always)]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W {
        CLOCK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_ctl](index.html) module"]
pub struct CLOCK_CTL_SPEC;
impl crate::RegisterSpec for CLOCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_ctl::R](R) reader structure"]
impl crate::Readable for CLOCK_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_ctl::W](W) writer structure"]
impl crate::Writable for CLOCK_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK_CTL to value 0"]
impl crate::Resettable for CLOCK_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
