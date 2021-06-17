#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_CLK_EDGE` reader - Clock edge used for transmitting (Transmision uses internal core clock) 0: Negative Edge (Default) 1: Positive Edge"]
pub struct TX_CLK_EDGE_R(crate::FieldReader<bool, bool>);
impl TX_CLK_EDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CLK_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CLK_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CLK_EDGE` writer - Clock edge used for transmitting (Transmision uses internal core clock) 0: Negative Edge (Default) 1: Positive Edge"]
pub struct TX_CLK_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLK_EDGE_W<'a> {
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
#[doc = "Field `RX_CLK_EDGE` reader - Clock edge used for sampling the received data (Sampling uses clock selected by RX_CLK_SRC) 0: Negative Edge (Default) 1: Positive Edge Note: For RX_CLK_SRC =1, when pad clock is used as sampling clock, this field is ignored"]
pub struct RX_CLK_EDGE_R(crate::FieldReader<bool, bool>);
impl RX_CLK_EDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_CLK_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CLK_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CLK_EDGE` writer - Clock edge used for sampling the received data (Sampling uses clock selected by RX_CLK_SRC) 0: Negative Edge (Default) 1: Positive Edge Note: For RX_CLK_SRC =1, when pad clock is used as sampling clock, this field is ignored"]
pub struct RX_CLK_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLK_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RX_CLK_SRC` reader - Clock to be used for sampling the received data 0: Internal clock (Default) 1: Clock from the SCK pad When Clock from the SCK pad is used, sampling is always on negedge only"]
pub struct RX_CLK_SRC_R(crate::FieldReader<bool, bool>);
impl RX_CLK_SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_CLK_SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CLK_SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CLK_SRC` writer - Clock to be used for sampling the received data 0: Internal clock (Default) 1: Clock from the SCK pad When Clock from the SCK pad is used, sampling is always on negedge only"]
pub struct RX_CLK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLK_SRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SCLK_CONTINUOUS` reader - Controls the behaviour of the RCB clock '0': SCLK is generated, only when the RCB controller is enabled and data is transmitted. '1': SCLK is generated, when the RCB controller is enabled."]
pub struct SCLK_CONTINUOUS_R(crate::FieldReader<bool, bool>);
impl SCLK_CONTINUOUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCLK_CONTINUOUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_CONTINUOUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_CONTINUOUS` writer - Controls the behaviour of the RCB clock '0': SCLK is generated, only when the RCB controller is enabled and data is transmitted. '1': SCLK is generated, when the RCB controller is enabled."]
pub struct SCLK_CONTINUOUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_CONTINUOUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SSEL_POLARITY` reader - Slave select polarity. SSEL_POLARITY applies to the outgoing slave select signal '0': slave select is low/'0' active. '1': slave select is high/'1' active."]
pub struct SSEL_POLARITY_R(crate::FieldReader<bool, bool>);
impl SSEL_POLARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSEL_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSEL_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSEL_POLARITY` writer - Slave select polarity. SSEL_POLARITY applies to the outgoing slave select signal '0': slave select is low/'0' active. '1': slave select is high/'1' active."]
pub struct SSEL_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL_POLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `LEAD` reader - N/A"]
pub struct LEAD_R(crate::FieldReader<u8, u8>);
impl LEAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEAD` writer - N/A"]
pub struct LEAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `LAG` reader - N/A"]
pub struct LAG_R(crate::FieldReader<u8, u8>);
impl LAG_R {
    pub(crate) fn new(bits: u8) -> Self {
        LAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LAG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LAG` writer - N/A"]
pub struct LAG_W<'a> {
    w: &'a mut W,
}
impl<'a> LAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `DIV_ENABLED` reader - Enable for RCB Clock Divider. The internal core clock divider is bypassed when DIV_ENABLED=0"]
pub struct DIV_ENABLED_R(crate::FieldReader<bool, bool>);
impl DIV_ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIV_ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_ENABLED` writer - Enable for RCB Clock Divider. The internal core clock divider is bypassed when DIV_ENABLED=0"]
pub struct DIV_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DIV` reader - The internal core clock divider factor when DIV_ENABLED=1 Divider factor: 2*DIV. Max DIV value supported is 31. DIV value of zero is not supported. Make DIV_ENABLED=0 for undivided clock"]
pub struct DIV_R(crate::FieldReader<u8, u8>);
impl DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - The internal core clock divider factor when DIV_ENABLED=1 Divider factor: 2*DIV. Max DIV value supported is 31. DIV value of zero is not supported. Make DIV_ENABLED=0 for undivided clock"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 13)) | ((value as u32 & 0x3f) << 13);
        self.w
    }
}
#[doc = "Field `ADDR_WIDTH` reader - Width of Address phase (includes read/write mode bit) of the Dataframe width. ADDR_WIDTH + 1 is the amount of bits in a transmitted data frame. Allowed legal values are 'd8, 'd10 and 'd15"]
pub struct ADDR_WIDTH_R(crate::FieldReader<u8, u8>);
impl ADDR_WIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDR_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR_WIDTH` writer - Width of Address phase (includes read/write mode bit) of the Dataframe width. ADDR_WIDTH + 1 is the amount of bits in a transmitted data frame. Allowed legal values are 'd8, 'd10 and 'd15"]
pub struct ADDR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | ((value as u32 & 0x0f) << 19);
        self.w
    }
}
#[doc = "Field `DATA_WIDTH` reader - Width of Data phase of the transmit Dataframe width. 0 - 8 bits 1 - 16 bits"]
pub struct DATA_WIDTH_R(crate::FieldReader<bool, bool>);
impl DATA_WIDTH_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_WIDTH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_WIDTH` writer - Width of Data phase of the transmit Dataframe width. 0 - 8 bits 1 - 16 bits"]
pub struct DATA_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_WIDTH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ENABLED` reader - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using CTRL except ENABLED field. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the edges, dividers. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
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
#[doc = "Field `ENABLED` writer - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using CTRL except ENABLED field. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the edges, dividers. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
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
    #[doc = "Bit 1 - Clock edge used for transmitting (Transmision uses internal core clock) 0: Negative Edge (Default) 1: Positive Edge"]
    #[inline(always)]
    pub fn tx_clk_edge(&self) -> TX_CLK_EDGE_R {
        TX_CLK_EDGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock edge used for sampling the received data (Sampling uses clock selected by RX_CLK_SRC) 0: Negative Edge (Default) 1: Positive Edge Note: For RX_CLK_SRC =1, when pad clock is used as sampling clock, this field is ignored"]
    #[inline(always)]
    pub fn rx_clk_edge(&self) -> RX_CLK_EDGE_R {
        RX_CLK_EDGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clock to be used for sampling the received data 0: Internal clock (Default) 1: Clock from the SCK pad When Clock from the SCK pad is used, sampling is always on negedge only"]
    #[inline(always)]
    pub fn rx_clk_src(&self) -> RX_CLK_SRC_R {
        RX_CLK_SRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Controls the behaviour of the RCB clock '0': SCLK is generated, only when the RCB controller is enabled and data is transmitted. '1': SCLK is generated, when the RCB controller is enabled."]
    #[inline(always)]
    pub fn sclk_continuous(&self) -> SCLK_CONTINUOUS_R {
        SCLK_CONTINUOUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slave select polarity. SSEL_POLARITY applies to the outgoing slave select signal '0': slave select is low/'0' active. '1': slave select is high/'1' active."]
    #[inline(always)]
    pub fn ssel_polarity(&self) -> SSEL_POLARITY_R {
        SSEL_POLARITY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn lead(&self) -> LEAD_R {
        LEAD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - N/A"]
    #[inline(always)]
    pub fn lag(&self) -> LAG_R {
        LAG_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Enable for RCB Clock Divider. The internal core clock divider is bypassed when DIV_ENABLED=0"]
    #[inline(always)]
    pub fn div_enabled(&self) -> DIV_ENABLED_R {
        DIV_ENABLED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:18 - The internal core clock divider factor when DIV_ENABLED=1 Divider factor: 2*DIV. Max DIV value supported is 31. DIV value of zero is not supported. Make DIV_ENABLED=0 for undivided clock"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 13) & 0x3f) as u8)
    }
    #[doc = "Bits 19:22 - Width of Address phase (includes read/write mode bit) of the Dataframe width. ADDR_WIDTH + 1 is the amount of bits in a transmitted data frame. Allowed legal values are 'd8, 'd10 and 'd15"]
    #[inline(always)]
    pub fn addr_width(&self) -> ADDR_WIDTH_R {
        ADDR_WIDTH_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Width of Data phase of the transmit Dataframe width. 0 - 8 bits 1 - 16 bits"]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using CTRL except ENABLED field. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the edges, dividers. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock edge used for transmitting (Transmision uses internal core clock) 0: Negative Edge (Default) 1: Positive Edge"]
    #[inline(always)]
    pub fn tx_clk_edge(&mut self) -> TX_CLK_EDGE_W {
        TX_CLK_EDGE_W { w: self }
    }
    #[doc = "Bit 2 - Clock edge used for sampling the received data (Sampling uses clock selected by RX_CLK_SRC) 0: Negative Edge (Default) 1: Positive Edge Note: For RX_CLK_SRC =1, when pad clock is used as sampling clock, this field is ignored"]
    #[inline(always)]
    pub fn rx_clk_edge(&mut self) -> RX_CLK_EDGE_W {
        RX_CLK_EDGE_W { w: self }
    }
    #[doc = "Bit 3 - Clock to be used for sampling the received data 0: Internal clock (Default) 1: Clock from the SCK pad When Clock from the SCK pad is used, sampling is always on negedge only"]
    #[inline(always)]
    pub fn rx_clk_src(&mut self) -> RX_CLK_SRC_W {
        RX_CLK_SRC_W { w: self }
    }
    #[doc = "Bit 4 - Controls the behaviour of the RCB clock '0': SCLK is generated, only when the RCB controller is enabled and data is transmitted. '1': SCLK is generated, when the RCB controller is enabled."]
    #[inline(always)]
    pub fn sclk_continuous(&mut self) -> SCLK_CONTINUOUS_W {
        SCLK_CONTINUOUS_W { w: self }
    }
    #[doc = "Bit 5 - Slave select polarity. SSEL_POLARITY applies to the outgoing slave select signal '0': slave select is low/'0' active. '1': slave select is high/'1' active."]
    #[inline(always)]
    pub fn ssel_polarity(&mut self) -> SSEL_POLARITY_W {
        SSEL_POLARITY_W { w: self }
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn lead(&mut self) -> LEAD_W {
        LEAD_W { w: self }
    }
    #[doc = "Bits 10:11 - N/A"]
    #[inline(always)]
    pub fn lag(&mut self) -> LAG_W {
        LAG_W { w: self }
    }
    #[doc = "Bit 12 - Enable for RCB Clock Divider. The internal core clock divider is bypassed when DIV_ENABLED=0"]
    #[inline(always)]
    pub fn div_enabled(&mut self) -> DIV_ENABLED_W {
        DIV_ENABLED_W { w: self }
    }
    #[doc = "Bits 13:18 - The internal core clock divider factor when DIV_ENABLED=1 Divider factor: 2*DIV. Max DIV value supported is 31. DIV value of zero is not supported. Make DIV_ENABLED=0 for undivided clock"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Bits 19:22 - Width of Address phase (includes read/write mode bit) of the Dataframe width. ADDR_WIDTH + 1 is the amount of bits in a transmitted data frame. Allowed legal values are 'd8, 'd10 and 'd15"]
    #[inline(always)]
    pub fn addr_width(&mut self) -> ADDR_WIDTH_W {
        ADDR_WIDTH_W { w: self }
    }
    #[doc = "Bit 23 - Width of Data phase of the transmit Dataframe width. 0 - 8 bits 1 - 16 bits"]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W {
        DATA_WIDTH_W { w: self }
    }
    #[doc = "Bit 31 - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using CTRL except ENABLED field. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the edges, dividers. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
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
#[doc = "RCB control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x00f8_0000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00f8_0000
    }
}
