#[doc = "Register `MT_DELAY_CFG2` reader"]
pub struct R(crate::R<MT_DELAY_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MT_DELAY_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MT_DELAY_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MT_DELAY_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MT_DELAY_CFG2` writer"]
pub struct W(crate::W<MT_DELAY_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MT_DELAY_CFG2_SPEC>;
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
impl From<crate::W<MT_DELAY_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MT_DELAY_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC_STARTUP_DELAY_LF` reader - This register specifies the time for OSC Startup. After this delay, clock is enabled to the link layer. Clock is enabled after OSC_STARTUP_DELAY + 1 LF clock cycles. If PSoC was in DPSLP when XTAL is enabled, then the wakeup delay will be OSC_STARTUP_DELAY + 1 + PSoC Wakeup time. Minimum value to be programmed in 1. This is equivalent to Link Layer register WAKEUP_CONFIG.OSC_STARTUP_DELAY, but is specified in LF cycles"]
pub struct OSC_STARTUP_DELAY_LF_R(crate::FieldReader<u8, u8>);
impl OSC_STARTUP_DELAY_LF_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSC_STARTUP_DELAY_LF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_STARTUP_DELAY_LF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC_STARTUP_DELAY_LF` writer - This register specifies the time for OSC Startup. After this delay, clock is enabled to the link layer. Clock is enabled after OSC_STARTUP_DELAY + 1 LF clock cycles. If PSoC was in DPSLP when XTAL is enabled, then the wakeup delay will be OSC_STARTUP_DELAY + 1 + PSoC Wakeup time. Minimum value to be programmed in 1. This is equivalent to Link Layer register WAKEUP_CONFIG.OSC_STARTUP_DELAY, but is specified in LF cycles"]
pub struct OSC_STARTUP_DELAY_LF_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_STARTUP_DELAY_LF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DSM_OFFSET_TO_WAKEUP_INSTANT_LF` reader - This register specifies the pre-processing time required in Link Layer. This is esentially the time from CLK_EN (ungating clock in CYBLERD55) to the time when logic in CYBLERD55 is switched to Active mode Regulator.The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. This is equivalent to Link Layer register WAKEUP_CONFIG.DSM_OFFSET_TO_WAKEUP_INSTANT_LF, but is specified in LF cycles."]
pub struct DSM_OFFSET_TO_WAKEUP_INSTANT_LF_R(crate::FieldReader<u8, u8>);
impl DSM_OFFSET_TO_WAKEUP_INSTANT_LF_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSM_OFFSET_TO_WAKEUP_INSTANT_LF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSM_OFFSET_TO_WAKEUP_INSTANT_LF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSM_OFFSET_TO_WAKEUP_INSTANT_LF` writer - This register specifies the pre-processing time required in Link Layer. This is esentially the time from CLK_EN (ungating clock in CYBLERD55) to the time when logic in CYBLERD55 is switched to Active mode Regulator.The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. This is equivalent to Link Layer register WAKEUP_CONFIG.DSM_OFFSET_TO_WAKEUP_INSTANT_LF, but is specified in LF cycles."]
pub struct DSM_OFFSET_TO_WAKEUP_INSTANT_LF_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_OFFSET_TO_WAKEUP_INSTANT_LF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ACT_STARTUP_DELAY` reader - This register specifes the Active Regulator startup time in CYBLERD55. The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. The digital LDO will be turned on after this time elapses"]
pub struct ACT_STARTUP_DELAY_R(crate::FieldReader<u8, u8>);
impl ACT_STARTUP_DELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACT_STARTUP_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_STARTUP_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_STARTUP_DELAY` writer - This register specifes the Active Regulator startup time in CYBLERD55. The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. The digital LDO will be turned on after this time elapses"]
pub struct ACT_STARTUP_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_STARTUP_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DIG_LDO_STARTUP_DELAY` reader - This register specifes the Digital LDO startup time in CYBLERD55.The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. The logic in CYBLERD55 is switched to Active mode Regulator after this (ACT_STARTUP_DELAY + DIG_LDO_STARTUP_DELAY)"]
pub struct DIG_LDO_STARTUP_DELAY_R(crate::FieldReader<u8, u8>);
impl DIG_LDO_STARTUP_DELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIG_LDO_STARTUP_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_LDO_STARTUP_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_LDO_STARTUP_DELAY` writer - This register specifes the Digital LDO startup time in CYBLERD55.The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. The logic in CYBLERD55 is switched to Active mode Regulator after this (ACT_STARTUP_DELAY + DIG_LDO_STARTUP_DELAY)"]
pub struct DIG_LDO_STARTUP_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_LDO_STARTUP_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register specifies the time for OSC Startup. After this delay, clock is enabled to the link layer. Clock is enabled after OSC_STARTUP_DELAY + 1 LF clock cycles. If PSoC was in DPSLP when XTAL is enabled, then the wakeup delay will be OSC_STARTUP_DELAY + 1 + PSoC Wakeup time. Minimum value to be programmed in 1. This is equivalent to Link Layer register WAKEUP_CONFIG.OSC_STARTUP_DELAY, but is specified in LF cycles"]
    #[inline(always)]
    pub fn osc_startup_delay_lf(&self) -> OSC_STARTUP_DELAY_LF_R {
        OSC_STARTUP_DELAY_LF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register specifies the pre-processing time required in Link Layer. This is esentially the time from CLK_EN (ungating clock in CYBLERD55) to the time when logic in CYBLERD55 is switched to Active mode Regulator.The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. This is equivalent to Link Layer register WAKEUP_CONFIG.DSM_OFFSET_TO_WAKEUP_INSTANT_LF, but is specified in LF cycles."]
    #[inline(always)]
    pub fn dsm_offset_to_wakeup_instant_lf(&self) -> DSM_OFFSET_TO_WAKEUP_INSTANT_LF_R {
        DSM_OFFSET_TO_WAKEUP_INSTANT_LF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register specifes the Active Regulator startup time in CYBLERD55. The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. The digital LDO will be turned on after this time elapses"]
    #[inline(always)]
    pub fn act_startup_delay(&self) -> ACT_STARTUP_DELAY_R {
        ACT_STARTUP_DELAY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This register specifes the Digital LDO startup time in CYBLERD55.The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. The logic in CYBLERD55 is switched to Active mode Regulator after this (ACT_STARTUP_DELAY + DIG_LDO_STARTUP_DELAY)"]
    #[inline(always)]
    pub fn dig_ldo_startup_delay(&self) -> DIG_LDO_STARTUP_DELAY_R {
        DIG_LDO_STARTUP_DELAY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register specifies the time for OSC Startup. After this delay, clock is enabled to the link layer. Clock is enabled after OSC_STARTUP_DELAY + 1 LF clock cycles. If PSoC was in DPSLP when XTAL is enabled, then the wakeup delay will be OSC_STARTUP_DELAY + 1 + PSoC Wakeup time. Minimum value to be programmed in 1. This is equivalent to Link Layer register WAKEUP_CONFIG.OSC_STARTUP_DELAY, but is specified in LF cycles"]
    #[inline(always)]
    pub fn osc_startup_delay_lf(&mut self) -> OSC_STARTUP_DELAY_LF_W {
        OSC_STARTUP_DELAY_LF_W { w: self }
    }
    #[doc = "Bits 8:15 - This register specifies the pre-processing time required in Link Layer. This is esentially the time from CLK_EN (ungating clock in CYBLERD55) to the time when logic in CYBLERD55 is switched to Active mode Regulator.The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. This is equivalent to Link Layer register WAKEUP_CONFIG.DSM_OFFSET_TO_WAKEUP_INSTANT_LF, but is specified in LF cycles."]
    #[inline(always)]
    pub fn dsm_offset_to_wakeup_instant_lf(&mut self) -> DSM_OFFSET_TO_WAKEUP_INSTANT_LF_W {
        DSM_OFFSET_TO_WAKEUP_INSTANT_LF_W { w: self }
    }
    #[doc = "Bits 16:23 - This register specifes the Active Regulator startup time in CYBLERD55. The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. The digital LDO will be turned on after this time elapses"]
    #[inline(always)]
    pub fn act_startup_delay(&mut self) -> ACT_STARTUP_DELAY_W {
        ACT_STARTUP_DELAY_W { w: self }
    }
    #[doc = "Bits 24:31 - This register specifes the Digital LDO startup time in CYBLERD55.The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. The logic in CYBLERD55 is switched to Active mode Regulator after this (ACT_STARTUP_DELAY + DIG_LDO_STARTUP_DELAY)"]
    #[inline(always)]
    pub fn dig_ldo_startup_delay(&mut self) -> DIG_LDO_STARTUP_DELAY_W {
        DIG_LDO_STARTUP_DELAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MT Delay configuration for state transitions\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_delay_cfg2](index.html) module"]
pub struct MT_DELAY_CFG2_SPEC;
impl crate::RegisterSpec for MT_DELAY_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mt_delay_cfg2::R](R) reader structure"]
impl crate::Readable for MT_DELAY_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mt_delay_cfg2::W](W) writer structure"]
impl crate::Writable for MT_DELAY_CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MT_DELAY_CFG2 to value 0"]
impl crate::Resettable for MT_DELAY_CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
