#[doc = "Register `DDFT_CONFIG` reader"]
pub struct R(crate::R<DDFT_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDFT_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDFT_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDFT_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDFT_CONFIG` writer"]
pub struct W(crate::W<DDFT_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDFT_CONFIG_SPEC>;
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
impl From<crate::W<DDFT_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDFT_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDFT_ENABLE` reader - Enables the DDFT output from BLESS 1: DDFT is enabled 0: DDFT is disabled"]
pub struct DDFT_ENABLE_R(crate::FieldReader<bool, bool>);
impl DDFT_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDFT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDFT_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDFT_ENABLE` writer - Enables the DDFT output from BLESS 1: DDFT is enabled 0: DDFT is disabled"]
pub struct DDFT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `BLERD_DDFT_EN` reader - Enables the DDFT inputs from CYBLERD55 chip 1: DDFT inputs are enabled 0: DDFT inputs are disabled"]
pub struct BLERD_DDFT_EN_R(crate::FieldReader<bool, bool>);
impl BLERD_DDFT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLERD_DDFT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLERD_DDFT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLERD_DDFT_EN` writer - Enables the DDFT inputs from CYBLERD55 chip 1: DDFT inputs are enabled 0: DDFT inputs are disabled"]
pub struct BLERD_DDFT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLERD_DDFT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DDFT_MUX_CFG1` reader - dbg_mux_pin1 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[0\\]
5'h01 rcb_tx_fifo_empty 5'h02 hv_ldo_lv_detect_raw 5'h03 dbus_rx_en 5'h04 1'b0 5'h05 clk_switch_to_sysclk 5'h06 ll_clk_en_sync 5'h07 dsm_entry_stat 5'h08 proc_tx_en 5'h09 rssi_read_start 5'h0A tx_2mbps 5'h0B rcb_bus_busy 5'h0C hv_ldo_en_mt (act_stdbyb) 5'h0D ll_eco_clk_en 5'h0E blerd_reset_assert 5'h0F hv_ldo_byp_n 5'h10 hv_ldo_lv_detect_mt 5'h11 enable_ldo 5'h12 enable_ldo_dly 5'h13 bless_rcb_le_out 5'h14 bless_rcb_clk_out 5'h15 bless_dig_ldo_on_out 5'h16 bless_act_ldo_en_out 5'h17 bless_clk_en_out 5'h18 bless_buck_en_out 5'h19 bless_ret_switch_hv_out 5'h1A efuse_rw_out 5'h1B efuse_avdd_out 5'h1C efuse_config_efuse_mode 5'h1D bless_dbus_tx_en_pad 5'h1E bless_bpktctl_rd 5'h1F 1'b0"]
pub struct DDFT_MUX_CFG1_R(crate::FieldReader<u8, u8>);
impl DDFT_MUX_CFG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DDFT_MUX_CFG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDFT_MUX_CFG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDFT_MUX_CFG1` writer - dbg_mux_pin1 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[0\\]
5'h01 rcb_tx_fifo_empty 5'h02 hv_ldo_lv_detect_raw 5'h03 dbus_rx_en 5'h04 1'b0 5'h05 clk_switch_to_sysclk 5'h06 ll_clk_en_sync 5'h07 dsm_entry_stat 5'h08 proc_tx_en 5'h09 rssi_read_start 5'h0A tx_2mbps 5'h0B rcb_bus_busy 5'h0C hv_ldo_en_mt (act_stdbyb) 5'h0D ll_eco_clk_en 5'h0E blerd_reset_assert 5'h0F hv_ldo_byp_n 5'h10 hv_ldo_lv_detect_mt 5'h11 enable_ldo 5'h12 enable_ldo_dly 5'h13 bless_rcb_le_out 5'h14 bless_rcb_clk_out 5'h15 bless_dig_ldo_on_out 5'h16 bless_act_ldo_en_out 5'h17 bless_clk_en_out 5'h18 bless_buck_en_out 5'h19 bless_ret_switch_hv_out 5'h1A efuse_rw_out 5'h1B efuse_avdd_out 5'h1C efuse_config_efuse_mode 5'h1D bless_dbus_tx_en_pad 5'h1E bless_bpktctl_rd 5'h1F 1'b0"]
pub struct DDFT_MUX_CFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_MUX_CFG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `DDFT_MUX_CFG2` reader - dbg_mux_pin2 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[1\\]
5'h01 rcb_rx_fifo_empty 5'h02 ll_decode_rxdata 5'h03 dbus_tx_en 5'h04 fw_clk_en 5'h05 interrupt_ll_n 5'h06 llh_st_sm 5'h07 llh_st_dsm 5'h08 proc_rx_en 5'h09 rssi_rx_done 5'h0A rx_2mbps 5'h0B rcb_ll_ctrl 5'h0C hv_ldo_byp_n 5'h0D reset_deassert 5'h0E rcb_intr 5'h0F rcb_ll_intr 5'h10 hv_ldo_en_mt (act_stdbyb) 5'h11 hv_ldo_lv_detect_raw 5'h12 bless_rcb_data_in 5'h13 bless_xtal_en_out 5'h14 bless_isolate_n_out 5'h15 bless_reset_n_out 5'h16 bless_ret_ldo_ol_hv_out 5'h17 bless_txd_rxd_out 5'h18 tx_rx_ctrl_sel 5'h19 bless_bpktctl_cy 5'h1A efuse_cs_out 5'h1B efuse_pgm_out 5'h1C efuse_sclk_out 5'h1D hv_ldo_lv_detect_mt 5'h1E enable_ldo 5'h1F enable_ldo_dly"]
pub struct DDFT_MUX_CFG2_R(crate::FieldReader<u8, u8>);
impl DDFT_MUX_CFG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DDFT_MUX_CFG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDFT_MUX_CFG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDFT_MUX_CFG2` writer - dbg_mux_pin2 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[1\\]
5'h01 rcb_rx_fifo_empty 5'h02 ll_decode_rxdata 5'h03 dbus_tx_en 5'h04 fw_clk_en 5'h05 interrupt_ll_n 5'h06 llh_st_sm 5'h07 llh_st_dsm 5'h08 proc_rx_en 5'h09 rssi_rx_done 5'h0A rx_2mbps 5'h0B rcb_ll_ctrl 5'h0C hv_ldo_byp_n 5'h0D reset_deassert 5'h0E rcb_intr 5'h0F rcb_ll_intr 5'h10 hv_ldo_en_mt (act_stdbyb) 5'h11 hv_ldo_lv_detect_raw 5'h12 bless_rcb_data_in 5'h13 bless_xtal_en_out 5'h14 bless_isolate_n_out 5'h15 bless_reset_n_out 5'h16 bless_ret_ldo_ol_hv_out 5'h17 bless_txd_rxd_out 5'h18 tx_rx_ctrl_sel 5'h19 bless_bpktctl_cy 5'h1A efuse_cs_out 5'h1B efuse_pgm_out 5'h1C efuse_sclk_out 5'h1D hv_ldo_lv_detect_mt 5'h1E enable_ldo 5'h1F enable_ldo_dly"]
pub struct DDFT_MUX_CFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_MUX_CFG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enables the DDFT output from BLESS 1: DDFT is enabled 0: DDFT is disabled"]
    #[inline(always)]
    pub fn ddft_enable(&self) -> DDFT_ENABLE_R {
        DDFT_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the DDFT inputs from CYBLERD55 chip 1: DDFT inputs are enabled 0: DDFT inputs are disabled"]
    #[inline(always)]
    pub fn blerd_ddft_en(&self) -> BLERD_DDFT_EN_R {
        BLERD_DDFT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - dbg_mux_pin1 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[0\\]
5'h01 rcb_tx_fifo_empty 5'h02 hv_ldo_lv_detect_raw 5'h03 dbus_rx_en 5'h04 1'b0 5'h05 clk_switch_to_sysclk 5'h06 ll_clk_en_sync 5'h07 dsm_entry_stat 5'h08 proc_tx_en 5'h09 rssi_read_start 5'h0A tx_2mbps 5'h0B rcb_bus_busy 5'h0C hv_ldo_en_mt (act_stdbyb) 5'h0D ll_eco_clk_en 5'h0E blerd_reset_assert 5'h0F hv_ldo_byp_n 5'h10 hv_ldo_lv_detect_mt 5'h11 enable_ldo 5'h12 enable_ldo_dly 5'h13 bless_rcb_le_out 5'h14 bless_rcb_clk_out 5'h15 bless_dig_ldo_on_out 5'h16 bless_act_ldo_en_out 5'h17 bless_clk_en_out 5'h18 bless_buck_en_out 5'h19 bless_ret_switch_hv_out 5'h1A efuse_rw_out 5'h1B efuse_avdd_out 5'h1C efuse_config_efuse_mode 5'h1D bless_dbus_tx_en_pad 5'h1E bless_bpktctl_rd 5'h1F 1'b0"]
    #[inline(always)]
    pub fn ddft_mux_cfg1(&self) -> DDFT_MUX_CFG1_R {
        DDFT_MUX_CFG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - dbg_mux_pin2 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[1\\]
5'h01 rcb_rx_fifo_empty 5'h02 ll_decode_rxdata 5'h03 dbus_tx_en 5'h04 fw_clk_en 5'h05 interrupt_ll_n 5'h06 llh_st_sm 5'h07 llh_st_dsm 5'h08 proc_rx_en 5'h09 rssi_rx_done 5'h0A rx_2mbps 5'h0B rcb_ll_ctrl 5'h0C hv_ldo_byp_n 5'h0D reset_deassert 5'h0E rcb_intr 5'h0F rcb_ll_intr 5'h10 hv_ldo_en_mt (act_stdbyb) 5'h11 hv_ldo_lv_detect_raw 5'h12 bless_rcb_data_in 5'h13 bless_xtal_en_out 5'h14 bless_isolate_n_out 5'h15 bless_reset_n_out 5'h16 bless_ret_ldo_ol_hv_out 5'h17 bless_txd_rxd_out 5'h18 tx_rx_ctrl_sel 5'h19 bless_bpktctl_cy 5'h1A efuse_cs_out 5'h1B efuse_pgm_out 5'h1C efuse_sclk_out 5'h1D hv_ldo_lv_detect_mt 5'h1E enable_ldo 5'h1F enable_ldo_dly"]
    #[inline(always)]
    pub fn ddft_mux_cfg2(&self) -> DDFT_MUX_CFG2_R {
        DDFT_MUX_CFG2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the DDFT output from BLESS 1: DDFT is enabled 0: DDFT is disabled"]
    #[inline(always)]
    pub fn ddft_enable(&mut self) -> DDFT_ENABLE_W {
        DDFT_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Enables the DDFT inputs from CYBLERD55 chip 1: DDFT inputs are enabled 0: DDFT inputs are disabled"]
    #[inline(always)]
    pub fn blerd_ddft_en(&mut self) -> BLERD_DDFT_EN_W {
        BLERD_DDFT_EN_W { w: self }
    }
    #[doc = "Bits 8:12 - dbg_mux_pin1 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[0\\]
5'h01 rcb_tx_fifo_empty 5'h02 hv_ldo_lv_detect_raw 5'h03 dbus_rx_en 5'h04 1'b0 5'h05 clk_switch_to_sysclk 5'h06 ll_clk_en_sync 5'h07 dsm_entry_stat 5'h08 proc_tx_en 5'h09 rssi_read_start 5'h0A tx_2mbps 5'h0B rcb_bus_busy 5'h0C hv_ldo_en_mt (act_stdbyb) 5'h0D ll_eco_clk_en 5'h0E blerd_reset_assert 5'h0F hv_ldo_byp_n 5'h10 hv_ldo_lv_detect_mt 5'h11 enable_ldo 5'h12 enable_ldo_dly 5'h13 bless_rcb_le_out 5'h14 bless_rcb_clk_out 5'h15 bless_dig_ldo_on_out 5'h16 bless_act_ldo_en_out 5'h17 bless_clk_en_out 5'h18 bless_buck_en_out 5'h19 bless_ret_switch_hv_out 5'h1A efuse_rw_out 5'h1B efuse_avdd_out 5'h1C efuse_config_efuse_mode 5'h1D bless_dbus_tx_en_pad 5'h1E bless_bpktctl_rd 5'h1F 1'b0"]
    #[inline(always)]
    pub fn ddft_mux_cfg1(&mut self) -> DDFT_MUX_CFG1_W {
        DDFT_MUX_CFG1_W { w: self }
    }
    #[doc = "Bits 16:20 - dbg_mux_pin2 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[1\\]
5'h01 rcb_rx_fifo_empty 5'h02 ll_decode_rxdata 5'h03 dbus_tx_en 5'h04 fw_clk_en 5'h05 interrupt_ll_n 5'h06 llh_st_sm 5'h07 llh_st_dsm 5'h08 proc_rx_en 5'h09 rssi_rx_done 5'h0A rx_2mbps 5'h0B rcb_ll_ctrl 5'h0C hv_ldo_byp_n 5'h0D reset_deassert 5'h0E rcb_intr 5'h0F rcb_ll_intr 5'h10 hv_ldo_en_mt (act_stdbyb) 5'h11 hv_ldo_lv_detect_raw 5'h12 bless_rcb_data_in 5'h13 bless_xtal_en_out 5'h14 bless_isolate_n_out 5'h15 bless_reset_n_out 5'h16 bless_ret_ldo_ol_hv_out 5'h17 bless_txd_rxd_out 5'h18 tx_rx_ctrl_sel 5'h19 bless_bpktctl_cy 5'h1A efuse_cs_out 5'h1B efuse_pgm_out 5'h1C efuse_sclk_out 5'h1D hv_ldo_lv_detect_mt 5'h1E enable_ldo 5'h1F enable_ldo_dly"]
    #[inline(always)]
    pub fn ddft_mux_cfg2(&mut self) -> DDFT_MUX_CFG2_W {
        DDFT_MUX_CFG2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLESS DDFT configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddft_config](index.html) module"]
pub struct DDFT_CONFIG_SPEC;
impl crate::RegisterSpec for DDFT_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddft_config::R](R) reader structure"]
impl crate::Readable for DDFT_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddft_config::W](W) writer structure"]
impl crate::Writable for DDFT_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDFT_CONFIG to value 0"]
impl crate::Resettable for DDFT_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
