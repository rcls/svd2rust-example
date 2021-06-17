#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\]
is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and SMARTIO fabric is always bypassed. '0': No bypass (programmable SMARTIO fabric is exposed). '1': Bypass (programmable SMARTIOIO fabric is hidden)."]
pub struct BYPASS_R(crate::FieldReader<u8, u8>);
impl BYPASS_R {
    pub(crate) fn new(bits: u8) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS` writer - Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\]
is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and SMARTIO fabric is always bypassed. '0': No bypass (programmable SMARTIO fabric is exposed). '1': Bypass (programmable SMARTIOIO fabric is hidden)."]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CLOCK_SRC` reader - Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_smartio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_smartio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_smartio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_smartio' (note that 'clk_smartio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': Same as '17'. Note that the M0S8 SMARTIO version used the Hibernate reset for this value, but the MXS40 SMARTIO version does not support Hibernate functionality. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': asynchronous mode/'1'. Select this when clockless operation is configured. NOTE: Two positive edges of the selected clock are required for the block to be enabled (to deactivate reset). In asynchronous (clockless) mode clk_sys is used to enable the block, but is not available for clocking."]
pub struct CLOCK_SRC_R(crate::FieldReader<u8, u8>);
impl CLOCK_SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLOCK_SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCK_SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCK_SRC` writer - Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_smartio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_smartio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_smartio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_smartio' (note that 'clk_smartio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': Same as '17'. Note that the M0S8 SMARTIO version used the Hibernate reset for this value, but the MXS40 SMARTIO version does not support Hibernate functionality. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': asynchronous mode/'1'. Select this when clockless operation is configured. NOTE: Two positive edges of the selected clock are required for the block to be enabled (to deactivate reset). In asynchronous (clockless) mode clk_sys is used to enable the block, but is not available for clocking."]
pub struct CLOCK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `HLD_OVR` reader - IO cell hold override functionality. In DeepSleep power mode, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the SMARTIO is supposed to deliver DeepSleep output functionality on these IO pads. This field is used to control the hold override functionality from the SMARTIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The SMARTIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\]
is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\]
is '0'), the SMARTIO sets hold override to 'pwr_hld_ovr_hib' to enable SMARTIO functionality in DeepSleep power mode (but disables it in Hibernate or Stop power mode)."]
pub struct HLD_OVR_R(crate::FieldReader<bool, bool>);
impl HLD_OVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HLD_OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HLD_OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HLD_OVR` writer - IO cell hold override functionality. In DeepSleep power mode, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the SMARTIO is supposed to deliver DeepSleep output functionality on these IO pads. This field is used to control the hold override functionality from the SMARTIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The SMARTIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\]
is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\]
is '0'), the SMARTIO sets hold override to 'pwr_hld_ovr_hib' to enable SMARTIO functionality in DeepSleep power mode (but disables it in Hibernate or Stop power mode)."]
pub struct HLD_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> HLD_OVR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PIPELINE_EN` reader - Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
pub struct PIPELINE_EN_R(crate::FieldReader<bool, bool>);
impl PIPELINE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIPELINE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIPELINE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIPELINE_EN` writer - Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
pub struct PIPELINE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIPELINE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `ENABLED` reader - Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
pub struct ENABLED_R(crate::FieldReader<bool, bool>);
impl ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLED` writer - Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
    #[doc = "Bits 0:7 - Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\]
is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and SMARTIO fabric is always bypassed. '0': No bypass (programmable SMARTIO fabric is exposed). '1': Bypass (programmable SMARTIOIO fabric is hidden)."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_smartio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_smartio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_smartio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_smartio' (note that 'clk_smartio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': Same as '17'. Note that the M0S8 SMARTIO version used the Hibernate reset for this value, but the MXS40 SMARTIO version does not support Hibernate functionality. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': asynchronous mode/'1'. Select this when clockless operation is configured. NOTE: Two positive edges of the selected clock are required for the block to be enabled (to deactivate reset). In asynchronous (clockless) mode clk_sys is used to enable the block, but is not available for clocking."]
    #[inline(always)]
    pub fn clock_src(&self) -> CLOCK_SRC_R {
        CLOCK_SRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - IO cell hold override functionality. In DeepSleep power mode, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the SMARTIO is supposed to deliver DeepSleep output functionality on these IO pads. This field is used to control the hold override functionality from the SMARTIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The SMARTIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\]
is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\]
is '0'), the SMARTIO sets hold override to 'pwr_hld_ovr_hib' to enable SMARTIO functionality in DeepSleep power mode (but disables it in Hibernate or Stop power mode)."]
    #[inline(always)]
    pub fn hld_ovr(&self) -> HLD_OVR_R {
        HLD_OVR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
    #[inline(always)]
    pub fn pipeline_en(&self) -> PIPELINE_EN_R {
        PIPELINE_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\]
is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and SMARTIO fabric is always bypassed. '0': No bypass (programmable SMARTIO fabric is exposed). '1': Bypass (programmable SMARTIOIO fabric is hidden)."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bits 8:12 - Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_smartio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_smartio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_smartio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_smartio' (note that 'clk_smartio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': Same as '17'. Note that the M0S8 SMARTIO version used the Hibernate reset for this value, but the MXS40 SMARTIO version does not support Hibernate functionality. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': asynchronous mode/'1'. Select this when clockless operation is configured. NOTE: Two positive edges of the selected clock are required for the block to be enabled (to deactivate reset). In asynchronous (clockless) mode clk_sys is used to enable the block, but is not available for clocking."]
    #[inline(always)]
    pub fn clock_src(&mut self) -> CLOCK_SRC_W {
        CLOCK_SRC_W { w: self }
    }
    #[doc = "Bit 24 - IO cell hold override functionality. In DeepSleep power mode, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the SMARTIO is supposed to deliver DeepSleep output functionality on these IO pads. This field is used to control the hold override functionality from the SMARTIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The SMARTIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\]
is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\]
is '0'), the SMARTIO sets hold override to 'pwr_hld_ovr_hib' to enable SMARTIO functionality in DeepSleep power mode (but disables it in Hibernate or Stop power mode)."]
    #[inline(always)]
    pub fn hld_ovr(&mut self) -> HLD_OVR_W {
        HLD_OVR_W { w: self }
    }
    #[doc = "Bit 25 - Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
    #[inline(always)]
    pub fn pipeline_en(&mut self) -> PIPELINE_EN_W {
        PIPELINE_EN_W { w: self }
    }
    #[doc = "Bit 31 - Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0x0200_1400"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_1400
    }
}
