#[doc = "Register `ADC_CTL` reader"]
pub struct R(crate::R<ADC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CTL` writer"]
pub struct W(crate::W<ADC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CTL_SPEC>;
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
impl From<crate::W<ADC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_TIME` reader - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&2, or as the aperture to capture the input voltage on Cref1&2"]
pub struct ADC_TIME_R(crate::FieldReader<u8, u8>);
impl ADC_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_TIME` writer - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&2, or as the aperture to capture the input voltage on Cref1&2"]
pub struct ADC_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_MODE_A {
    #[doc = "0: No ADC measurement"]
    OFF = 0,
    #[doc = "1: Count time A to bring Cref1 + Cref2 up from Vssa to Vrefhi with IDACB"]
    VREF_CNT = 1,
    #[doc = "2: Count time B to bring Cref1 + Cref2 back up to Vrefhi with IDACB (after bringing them down for time A/2 cycles with IDACB sinking)"]
    VREF_BY2_CNT = 2,
    #[doc = "3: Determine HSCMP polarity and count time C to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    VIN_CNT = 3,
}
impl From<ADC_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_MODE` reader - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
pub struct ADC_MODE_R(crate::FieldReader<u8, ADC_MODE_A>);
impl ADC_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_MODE_A {
        match self.bits {
            0 => ADC_MODE_A::OFF,
            1 => ADC_MODE_A::VREF_CNT,
            2 => ADC_MODE_A::VREF_BY2_CNT,
            3 => ADC_MODE_A::VIN_CNT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == ADC_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `VREF_CNT`"]
    #[inline(always)]
    pub fn is_vref_cnt(&self) -> bool {
        **self == ADC_MODE_A::VREF_CNT
    }
    #[doc = "Checks if the value of the field is `VREF_BY2_CNT`"]
    #[inline(always)]
    pub fn is_vref_by2_cnt(&self) -> bool {
        **self == ADC_MODE_A::VREF_BY2_CNT
    }
    #[doc = "Checks if the value of the field is `VIN_CNT`"]
    #[inline(always)]
    pub fn is_vin_cnt(&self) -> bool {
        **self == ADC_MODE_A::VIN_CNT
    }
}
impl core::ops::Deref for ADC_MODE_R {
    type Target = crate::FieldReader<u8, ADC_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_MODE` writer - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
pub struct ADC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No ADC measurement"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ADC_MODE_A::OFF)
    }
    #[doc = "Count time A to bring Cref1 + Cref2 up from Vssa to Vrefhi with IDACB"]
    #[inline(always)]
    pub fn vref_cnt(self) -> &'a mut W {
        self.variant(ADC_MODE_A::VREF_CNT)
    }
    #[doc = "Count time B to bring Cref1 + Cref2 back up to Vrefhi with IDACB (after bringing them down for time A/2 cycles with IDACB sinking)"]
    #[inline(always)]
    pub fn vref_by2_cnt(self) -> &'a mut W {
        self.variant(ADC_MODE_A::VREF_BY2_CNT)
    }
    #[doc = "Determine HSCMP polarity and count time C to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    #[inline(always)]
    pub fn vin_cnt(self) -> &'a mut W {
        self.variant(ADC_MODE_A::VIN_CNT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&2, or as the aperture to capture the input voltage on Cref1&2"]
    #[inline(always)]
    pub fn adc_time(&self) -> ADC_TIME_R {
        ADC_TIME_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
    #[inline(always)]
    pub fn adc_mode(&self) -> ADC_MODE_R {
        ADC_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&2, or as the aperture to capture the input voltage on Cref1&2"]
    #[inline(always)]
    pub fn adc_time(&mut self) -> ADC_TIME_W {
        ADC_TIME_W { w: self }
    }
    #[doc = "Bits 16:17 - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
    #[inline(always)]
    pub fn adc_mode(&mut self) -> ADC_MODE_W {
        ADC_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ctl](index.html) module"]
pub struct ADC_CTL_SPEC;
impl crate::RegisterSpec for ADC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ctl::R](R) reader structure"]
impl crate::Readable for ADC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ctl::W](W) writer structure"]
impl crate::Writable for ADC_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CTL to value 0"]
impl crate::Resettable for ADC_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
