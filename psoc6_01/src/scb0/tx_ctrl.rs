#[doc = "Register `TX_CTRL` reader"]
pub struct R(crate::R<TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CTRL` writer"]
pub struct W(crate::W<TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CTRL_SPEC>;
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
impl From<crate::W<TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_WIDTH` reader - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7."]
pub struct DATA_WIDTH_R(crate::FieldReader<u8, u8>);
impl DATA_WIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_WIDTH` writer - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7."]
pub struct DATA_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `MSB_FIRST` reader - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
pub struct MSB_FIRST_R(crate::FieldReader<bool, bool>);
impl MSB_FIRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSB_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSB_FIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSB_FIRST` writer - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
pub struct MSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSB_FIRST_W<'a> {
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
#[doc = "Field `OPEN_DRAIN` reader - Each IO cell 'xxx' has two associated IP output signals 'xxx_out_en' and 'xxx_out'. '0': Normal operation mode. Typically, this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by a single IO cell. In this operation mode, for an IO cell 'xxx' that is used as an output, the 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. Typically this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by multiple IO cells (possibly on multiple chips). In this operation mode, for and IO cell 'xxx' that is used as an output, the 'xxx_out_en' output controls the outputted value. Typically, open drain operation mode drives low/'0' and the 'xxx_out' output is constant '1'. In other words, in open drain operation mode, the 'xxx_out_en' output is used to control the IO cell output value: in drive low/'0' mode: 'xxx_out_en' is '1' (drive enabled) to drive an IO cell output value of '0' and 'xxx_out_en' is '1' (drive disabled) to not drive an IO cell output value (another IO cell can drive the wire/line or a pull up results in a wire/line value '1'). The open drain mode is supported for: - I2C mode, 'i2c_scl' and 'i2c_sda' IO cells. - UART mode, 'uart_tx' IO cell (SPI slave). - SPI mode, 'spi_miso' IO cell."]
pub struct OPEN_DRAIN_R(crate::FieldReader<bool, bool>);
impl OPEN_DRAIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPEN_DRAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPEN_DRAIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPEN_DRAIN` writer - Each IO cell 'xxx' has two associated IP output signals 'xxx_out_en' and 'xxx_out'. '0': Normal operation mode. Typically, this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by a single IO cell. In this operation mode, for an IO cell 'xxx' that is used as an output, the 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. Typically this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by multiple IO cells (possibly on multiple chips). In this operation mode, for and IO cell 'xxx' that is used as an output, the 'xxx_out_en' output controls the outputted value. Typically, open drain operation mode drives low/'0' and the 'xxx_out' output is constant '1'. In other words, in open drain operation mode, the 'xxx_out_en' output is used to control the IO cell output value: in drive low/'0' mode: 'xxx_out_en' is '1' (drive enabled) to drive an IO cell output value of '0' and 'xxx_out_en' is '1' (drive disabled) to not drive an IO cell output value (another IO cell can drive the wire/line or a pull up results in a wire/line value '1'). The open drain mode is supported for: - I2C mode, 'i2c_scl' and 'i2c_sda' IO cells. - UART mode, 'uart_tx' IO cell (SPI slave). - SPI mode, 'spi_miso' IO cell."]
pub struct OPEN_DRAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPEN_DRAIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7."]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Each IO cell 'xxx' has two associated IP output signals 'xxx_out_en' and 'xxx_out'. '0': Normal operation mode. Typically, this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by a single IO cell. In this operation mode, for an IO cell 'xxx' that is used as an output, the 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. Typically this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by multiple IO cells (possibly on multiple chips). In this operation mode, for and IO cell 'xxx' that is used as an output, the 'xxx_out_en' output controls the outputted value. Typically, open drain operation mode drives low/'0' and the 'xxx_out' output is constant '1'. In other words, in open drain operation mode, the 'xxx_out_en' output is used to control the IO cell output value: in drive low/'0' mode: 'xxx_out_en' is '1' (drive enabled) to drive an IO cell output value of '0' and 'xxx_out_en' is '1' (drive disabled) to not drive an IO cell output value (another IO cell can drive the wire/line or a pull up results in a wire/line value '1'). The open drain mode is supported for: - I2C mode, 'i2c_scl' and 'i2c_sda' IO cells. - UART mode, 'uart_tx' IO cell (SPI slave). - SPI mode, 'spi_miso' IO cell."]
    #[inline(always)]
    pub fn open_drain(&self) -> OPEN_DRAIN_R {
        OPEN_DRAIN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7."]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W {
        DATA_WIDTH_W { w: self }
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    pub fn msb_first(&mut self) -> MSB_FIRST_W {
        MSB_FIRST_W { w: self }
    }
    #[doc = "Bit 16 - Each IO cell 'xxx' has two associated IP output signals 'xxx_out_en' and 'xxx_out'. '0': Normal operation mode. Typically, this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by a single IO cell. In this operation mode, for an IO cell 'xxx' that is used as an output, the 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. Typically this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by multiple IO cells (possibly on multiple chips). In this operation mode, for and IO cell 'xxx' that is used as an output, the 'xxx_out_en' output controls the outputted value. Typically, open drain operation mode drives low/'0' and the 'xxx_out' output is constant '1'. In other words, in open drain operation mode, the 'xxx_out_en' output is used to control the IO cell output value: in drive low/'0' mode: 'xxx_out_en' is '1' (drive enabled) to drive an IO cell output value of '0' and 'xxx_out_en' is '1' (drive disabled) to not drive an IO cell output value (another IO cell can drive the wire/line or a pull up results in a wire/line value '1'). The open drain mode is supported for: - I2C mode, 'i2c_scl' and 'i2c_sda' IO cells. - UART mode, 'uart_tx' IO cell (SPI slave). - SPI mode, 'spi_miso' IO cell."]
    #[inline(always)]
    pub fn open_drain(&mut self) -> OPEN_DRAIN_W {
        OPEN_DRAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ctrl](index.html) module"]
pub struct TX_CTRL_SPEC;
impl crate::RegisterSpec for TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ctrl::R](R) reader structure"]
impl crate::Readable for TX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ctrl::W](W) writer structure"]
impl crate::Writable for TX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CTRL to value 0x0107"]
impl crate::Resettable for TX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0107
    }
}
