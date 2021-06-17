#[doc = "Register `INJ_CHAN_CONFIG` reader"]
pub struct R(crate::R<INJ_CHAN_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INJ_CHAN_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INJ_CHAN_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INJ_CHAN_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INJ_CHAN_CONFIG` writer"]
pub struct W(crate::W<INJ_CHAN_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INJ_CHAN_CONFIG_SPEC>;
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
impl From<crate::W<INJ_CHAN_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INJ_CHAN_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INJ_PIN_ADDR` reader - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
pub struct INJ_PIN_ADDR_R(crate::FieldReader<u8, u8>);
impl INJ_PIN_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        INJ_PIN_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_PIN_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_PIN_ADDR` writer - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
pub struct INJ_PIN_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_PIN_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Address of the port that contains the pin to be sampled by this channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INJ_PORT_ADDR_A {
    #[doc = "0: SARMUX pins."]
    SARMUX = 0,
    #[doc = "1: CTB0"]
    CTB0 = 1,
    #[doc = "2: CTB1"]
    CTB1 = 2,
    #[doc = "3: CTB2"]
    CTB2 = 3,
    #[doc = "4: CTB3"]
    CTB3 = 4,
    #[doc = "6: AROUTE virtual port"]
    AROUTE_VIRT = 6,
    #[doc = "7: SARMUX virtual port"]
    SARMUX_VIRT = 7,
}
impl From<INJ_PORT_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: INJ_PORT_ADDR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INJ_PORT_ADDR` reader - Address of the port that contains the pin to be sampled by this channel."]
pub struct INJ_PORT_ADDR_R(crate::FieldReader<u8, INJ_PORT_ADDR_A>);
impl INJ_PORT_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        INJ_PORT_ADDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INJ_PORT_ADDR_A> {
        match self.bits {
            0 => Some(INJ_PORT_ADDR_A::SARMUX),
            1 => Some(INJ_PORT_ADDR_A::CTB0),
            2 => Some(INJ_PORT_ADDR_A::CTB1),
            3 => Some(INJ_PORT_ADDR_A::CTB2),
            4 => Some(INJ_PORT_ADDR_A::CTB3),
            6 => Some(INJ_PORT_ADDR_A::AROUTE_VIRT),
            7 => Some(INJ_PORT_ADDR_A::SARMUX_VIRT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SARMUX`"]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        **self == INJ_PORT_ADDR_A::SARMUX
    }
    #[doc = "Checks if the value of the field is `CTB0`"]
    #[inline(always)]
    pub fn is_ctb0(&self) -> bool {
        **self == INJ_PORT_ADDR_A::CTB0
    }
    #[doc = "Checks if the value of the field is `CTB1`"]
    #[inline(always)]
    pub fn is_ctb1(&self) -> bool {
        **self == INJ_PORT_ADDR_A::CTB1
    }
    #[doc = "Checks if the value of the field is `CTB2`"]
    #[inline(always)]
    pub fn is_ctb2(&self) -> bool {
        **self == INJ_PORT_ADDR_A::CTB2
    }
    #[doc = "Checks if the value of the field is `CTB3`"]
    #[inline(always)]
    pub fn is_ctb3(&self) -> bool {
        **self == INJ_PORT_ADDR_A::CTB3
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT`"]
    #[inline(always)]
    pub fn is_aroute_virt(&self) -> bool {
        **self == INJ_PORT_ADDR_A::AROUTE_VIRT
    }
    #[doc = "Checks if the value of the field is `SARMUX_VIRT`"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        **self == INJ_PORT_ADDR_A::SARMUX_VIRT
    }
}
impl core::ops::Deref for INJ_PORT_ADDR_R {
    type Target = crate::FieldReader<u8, INJ_PORT_ADDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_PORT_ADDR` writer - Address of the port that contains the pin to be sampled by this channel."]
pub struct INJ_PORT_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_PORT_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INJ_PORT_ADDR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::SARMUX)
    }
    #[doc = "CTB0"]
    #[inline(always)]
    pub fn ctb0(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::CTB0)
    }
    #[doc = "CTB1"]
    #[inline(always)]
    pub fn ctb1(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::CTB1)
    }
    #[doc = "CTB2"]
    #[inline(always)]
    pub fn ctb2(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::CTB2)
    }
    #[doc = "CTB3"]
    #[inline(always)]
    pub fn ctb3(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::CTB3)
    }
    #[doc = "AROUTE virtual port"]
    #[inline(always)]
    pub fn aroute_virt(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::AROUTE_VIRT)
    }
    #[doc = "SARMUX virtual port"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::SARMUX_VIRT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `INJ_DIFFERENTIAL_EN` reader - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\]
is ignored)."]
pub struct INJ_DIFFERENTIAL_EN_R(crate::FieldReader<bool, bool>);
impl INJ_DIFFERENTIAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INJ_DIFFERENTIAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_DIFFERENTIAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_DIFFERENTIAL_EN` writer - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\]
is ignored)."]
pub struct INJ_DIFFERENTIAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_DIFFERENTIAL_EN_W<'a> {
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
#[doc = "Field `INJ_AVG_EN` reader - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub struct INJ_AVG_EN_R(crate::FieldReader<bool, bool>);
impl INJ_AVG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INJ_AVG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_AVG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_AVG_EN` writer - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub struct INJ_AVG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_AVG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `INJ_SAMPLE_TIME_SEL` reader - Injection sample time select: select which of the 4 global sample times to use for this channel"]
pub struct INJ_SAMPLE_TIME_SEL_R(crate::FieldReader<u8, u8>);
impl INJ_SAMPLE_TIME_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INJ_SAMPLE_TIME_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_SAMPLE_TIME_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_SAMPLE_TIME_SEL` writer - Injection sample time select: select which of the 4 global sample times to use for this channel"]
pub struct INJ_SAMPLE_TIME_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_SAMPLE_TIME_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `INJ_TAILGATING` reader - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set if the SAR is not busy. If the SAR is busy, the INJ channel addressed pin is sampled at the end of the current scan. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
pub struct INJ_TAILGATING_R(crate::FieldReader<bool, bool>);
impl INJ_TAILGATING_R {
    pub(crate) fn new(bits: bool) -> Self {
        INJ_TAILGATING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_TAILGATING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_TAILGATING` writer - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set if the SAR is not busy. If the SAR is busy, the INJ channel addressed pin is sampled at the end of the current scan. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
pub struct INJ_TAILGATING_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_TAILGATING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `INJ_START_EN` reader - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
pub struct INJ_START_EN_R(crate::FieldReader<bool, bool>);
impl INJ_START_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INJ_START_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_START_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_START_EN` writer - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
pub struct INJ_START_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_START_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
    #[inline(always)]
    pub fn inj_pin_addr(&self) -> INJ_PIN_ADDR_R {
        INJ_PIN_ADDR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn inj_port_addr(&self) -> INJ_PORT_ADDR_R {
        INJ_PORT_ADDR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    pub fn inj_differential_en(&self) -> INJ_DIFFERENTIAL_EN_R {
        INJ_DIFFERENTIAL_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn inj_avg_en(&self) -> INJ_AVG_EN_R {
        INJ_AVG_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Injection sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn inj_sample_time_sel(&self) -> INJ_SAMPLE_TIME_SEL_R {
        INJ_SAMPLE_TIME_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set if the SAR is not busy. If the SAR is busy, the INJ channel addressed pin is sampled at the end of the current scan. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
    #[inline(always)]
    pub fn inj_tailgating(&self) -> INJ_TAILGATING_R {
        INJ_TAILGATING_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
    #[inline(always)]
    pub fn inj_start_en(&self) -> INJ_START_EN_R {
        INJ_START_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
    #[inline(always)]
    pub fn inj_pin_addr(&mut self) -> INJ_PIN_ADDR_W {
        INJ_PIN_ADDR_W { w: self }
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn inj_port_addr(&mut self) -> INJ_PORT_ADDR_W {
        INJ_PORT_ADDR_W { w: self }
    }
    #[doc = "Bit 8 - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    pub fn inj_differential_en(&mut self) -> INJ_DIFFERENTIAL_EN_W {
        INJ_DIFFERENTIAL_EN_W { w: self }
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn inj_avg_en(&mut self) -> INJ_AVG_EN_W {
        INJ_AVG_EN_W { w: self }
    }
    #[doc = "Bits 12:13 - Injection sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn inj_sample_time_sel(&mut self) -> INJ_SAMPLE_TIME_SEL_W {
        INJ_SAMPLE_TIME_SEL_W { w: self }
    }
    #[doc = "Bit 30 - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set if the SAR is not busy. If the SAR is busy, the INJ channel addressed pin is sampled at the end of the current scan. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
    #[inline(always)]
    pub fn inj_tailgating(&mut self) -> INJ_TAILGATING_W {
        INJ_TAILGATING_W { w: self }
    }
    #[doc = "Bit 31 - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
    #[inline(always)]
    pub fn inj_start_en(&mut self) -> INJ_START_EN_W {
        INJ_START_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Injection channel configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inj_chan_config](index.html) module"]
pub struct INJ_CHAN_CONFIG_SPEC;
impl crate::RegisterSpec for INJ_CHAN_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inj_chan_config::R](R) reader structure"]
impl crate::Readable for INJ_CHAN_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inj_chan_config::W](W) writer structure"]
impl crate::Writable for INJ_CHAN_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INJ_CHAN_CONFIG to value 0"]
impl crate::Resettable for INJ_CHAN_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
