#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HV_TIMER_RUNNING` reader - Indicates if the high voltage timer is running: '0': not running '1': running"]
pub struct HV_TIMER_RUNNING_R(crate::FieldReader<bool, bool>);
impl HV_TIMER_RUNNING_R {
    pub(crate) fn new(bits: bool) -> Self {
        HV_TIMER_RUNNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HV_TIMER_RUNNING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HV_REGS_ISOLATED` reader - Indicates the isolation status at HV trim and redundancy registers inputs '0' - Not isolated, writing permitted '1' - isolated writing disabled"]
pub struct HV_REGS_ISOLATED_R(crate::FieldReader<bool, bool>);
impl HV_REGS_ISOLATED_R {
    pub(crate) fn new(bits: bool) -> Self {
        HV_REGS_ISOLATED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HV_REGS_ISOLATED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ILLEGAL_HVOP` reader - Indicates a bulk, sector erase, program has been requested when axa=1 '0' - no error '1' - illegal HV operation error"]
pub struct ILLEGAL_HVOP_R(crate::FieldReader<bool, bool>);
impl ILLEGAL_HVOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ILLEGAL_HVOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ILLEGAL_HVOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TURBO_N` reader - After FM power up indicates the analog blocks currents are boosted to faster reach their functional state.. Used in the testchip boot only as an 'FM READY' flag. '0' - turbo mode '1' - normal mode"]
pub struct TURBO_N_R(crate::FieldReader<bool, bool>);
impl TURBO_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        TURBO_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TURBO_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_EN_MON` reader - FM_CTL.WR_EN bit after being synchronized in clk_r domain"]
pub struct WR_EN_MON_R(crate::FieldReader<bool, bool>);
impl WR_EN_MON_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_EN_MON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_EN_MON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IF_SEL_MON` reader - FM_CTL.IF_SEL bit after being synchronized in clk_r domain"]
pub struct IF_SEL_MON_R(crate::FieldReader<bool, bool>);
impl IF_SEL_MON_R {
    pub(crate) fn new(bits: bool) -> Self {
        IF_SEL_MON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IF_SEL_MON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if the high voltage timer is running: '0': not running '1': running"]
    #[inline(always)]
    pub fn hv_timer_running(&self) -> HV_TIMER_RUNNING_R {
        HV_TIMER_RUNNING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates the isolation status at HV trim and redundancy registers inputs '0' - Not isolated, writing permitted '1' - isolated writing disabled"]
    #[inline(always)]
    pub fn hv_regs_isolated(&self) -> HV_REGS_ISOLATED_R {
        HV_REGS_ISOLATED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates a bulk, sector erase, program has been requested when axa=1 '0' - no error '1' - illegal HV operation error"]
    #[inline(always)]
    pub fn illegal_hvop(&self) -> ILLEGAL_HVOP_R {
        ILLEGAL_HVOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - After FM power up indicates the analog blocks currents are boosted to faster reach their functional state.. Used in the testchip boot only as an 'FM READY' flag. '0' - turbo mode '1' - normal mode"]
    #[inline(always)]
    pub fn turbo_n(&self) -> TURBO_N_R {
        TURBO_N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FM_CTL.WR_EN bit after being synchronized in clk_r domain"]
    #[inline(always)]
    pub fn wr_en_mon(&self) -> WR_EN_MON_R {
        WR_EN_MON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FM_CTL.IF_SEL bit after being synchronized in clk_r domain"]
    #[inline(always)]
    pub fn if_sel_mon(&self) -> IF_SEL_MON_R {
        IF_SEL_MON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
