#[doc = "Register `I2C_CTRL` reader"]
pub struct R(crate::R<I2C_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CTRL` writer"]
pub struct W(crate::W<I2C_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CTRL_SPEC>;
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
impl From<crate::W<I2C_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIGH_PHASE_OVS` reader - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period. The valid range is \\[5, 15\\]
with input signal median filtering and \\[4, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular interface (IF) high time to guarantee functional correct behavior. With input signal median filtering, the IF high time should be >= 6 IP clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF high time should be >= 5 IP clock cycles and <= 16 IP clock cycles."]
pub struct HIGH_PHASE_OVS_R(crate::FieldReader<u8, u8>);
impl HIGH_PHASE_OVS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HIGH_PHASE_OVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIGH_PHASE_OVS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIGH_PHASE_OVS` writer - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period. The valid range is \\[5, 15\\]
with input signal median filtering and \\[4, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular interface (IF) high time to guarantee functional correct behavior. With input signal median filtering, the IF high time should be >= 6 IP clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF high time should be >= 5 IP clock cycles and <= 16 IP clock cycles."]
pub struct HIGH_PHASE_OVS_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_PHASE_OVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `LOW_PHASE_OVS` reader - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period. The valid range is \\[7, 15\\]
with input signal median filtering and \\[6, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular (no stretching) interface (IF) low time to guarantee functional correct behavior. With input signal median filtering, the IF low time should be >= 8 IP clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF low time should be >= 7 IP clock cycles and <= 16 IP clock cycles."]
pub struct LOW_PHASE_OVS_R(crate::FieldReader<u8, u8>);
impl LOW_PHASE_OVS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOW_PHASE_OVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW_PHASE_OVS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOW_PHASE_OVS` writer - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period. The valid range is \\[7, 15\\]
with input signal median filtering and \\[6, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular (no stretching) interface (IF) low time to guarantee functional correct behavior. With input signal median filtering, the IF low time should be >= 8 IP clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF low time should be >= 7 IP clock cycles and <= 16 IP clock cycles."]
pub struct LOW_PHASE_OVS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_PHASE_OVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `M_READY_DATA_ACK` reader - When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
pub struct M_READY_DATA_ACK_R(crate::FieldReader<bool, bool>);
impl M_READY_DATA_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        M_READY_DATA_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_READY_DATA_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_READY_DATA_ACK` writer - When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
pub struct M_READY_DATA_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> M_READY_DATA_ACK_W<'a> {
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
#[doc = "Field `M_NOT_READY_DATA_NACK` reader - When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
pub struct M_NOT_READY_DATA_NACK_R(crate::FieldReader<bool, bool>);
impl M_NOT_READY_DATA_NACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        M_NOT_READY_DATA_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_NOT_READY_DATA_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_NOT_READY_DATA_NACK` writer - When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
pub struct M_NOT_READY_DATA_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> M_NOT_READY_DATA_NACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `S_GENERAL_IGNORE` reader - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure."]
pub struct S_GENERAL_IGNORE_R(crate::FieldReader<bool, bool>);
impl S_GENERAL_IGNORE_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_GENERAL_IGNORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_GENERAL_IGNORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_GENERAL_IGNORE` writer - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure."]
pub struct S_GENERAL_IGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> S_GENERAL_IGNORE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `S_READY_ADDR_ACK` reader - When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
pub struct S_READY_ADDR_ACK_R(crate::FieldReader<bool, bool>);
impl S_READY_ADDR_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_READY_ADDR_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_READY_ADDR_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_READY_ADDR_ACK` writer - When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
pub struct S_READY_ADDR_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_READY_ADDR_ACK_W<'a> {
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
#[doc = "Field `S_READY_DATA_ACK` reader - When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
pub struct S_READY_DATA_ACK_R(crate::FieldReader<bool, bool>);
impl S_READY_DATA_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_READY_DATA_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_READY_DATA_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_READY_DATA_ACK` writer - When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
pub struct S_READY_DATA_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_READY_DATA_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `S_NOT_READY_ADDR_NACK` reader - For internally clocked logic (EC_AM is '0' and EC_OP is '0') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when: - EC_AM is '0', EC_OP is '0' and non EZ mode. Functionality is as follows: - 1: a received (matching) slave address is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full). For externally clocked logic (EC_AM is '1') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when (NOT used when EC_AM is '1' and EC_OP is '1' and address match and EZ mode): - EC_AM is '1' and EC_OP is '0'. - EC_AM is '1' and general call address match. - EC_AM is '1' and non EZ mode. Functionality is as follows: - 1: a received (matching or general) slave address is always immediately NACK'd. There are two possibilities: 1). the internally clocked logic is enabled (we are in Active system power mode) and it handles the rest of the current transfer. In this case the I2C master will not observe the NACK. 2). the internally clocked logic is not enabled (we are in DeepSleep system power mode). In this case the I2C master will observe the NACK and may retry the transfer in the future (which gives the internally clocked logic the time to wake up from DeepSleep system power mode). - 0: clock stretching is performed (till the internally clocked logic takes over). The internally clocked logic will handle the ongoing transfer as soon as it is enabled."]
pub struct S_NOT_READY_ADDR_NACK_R(crate::FieldReader<bool, bool>);
impl S_NOT_READY_ADDR_NACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_NOT_READY_ADDR_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_NOT_READY_ADDR_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_NOT_READY_ADDR_NACK` writer - For internally clocked logic (EC_AM is '0' and EC_OP is '0') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when: - EC_AM is '0', EC_OP is '0' and non EZ mode. Functionality is as follows: - 1: a received (matching) slave address is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full). For externally clocked logic (EC_AM is '1') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when (NOT used when EC_AM is '1' and EC_OP is '1' and address match and EZ mode): - EC_AM is '1' and EC_OP is '0'. - EC_AM is '1' and general call address match. - EC_AM is '1' and non EZ mode. Functionality is as follows: - 1: a received (matching or general) slave address is always immediately NACK'd. There are two possibilities: 1). the internally clocked logic is enabled (we are in Active system power mode) and it handles the rest of the current transfer. In this case the I2C master will not observe the NACK. 2). the internally clocked logic is not enabled (we are in DeepSleep system power mode). In this case the I2C master will observe the NACK and may retry the transfer in the future (which gives the internally clocked logic the time to wake up from DeepSleep system power mode). - 0: clock stretching is performed (till the internally clocked logic takes over). The internally clocked logic will handle the ongoing transfer as soon as it is enabled."]
pub struct S_NOT_READY_ADDR_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_NOT_READY_ADDR_NACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `S_NOT_READY_DATA_NACK` reader - For internally clocked logic only. Only used when: - non EZ mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
pub struct S_NOT_READY_DATA_NACK_R(crate::FieldReader<bool, bool>);
impl S_NOT_READY_DATA_NACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_NOT_READY_DATA_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_NOT_READY_DATA_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_NOT_READY_DATA_NACK` writer - For internally clocked logic only. Only used when: - non EZ mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
pub struct S_NOT_READY_DATA_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_NOT_READY_DATA_NACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `LOOPBACK` reader - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', the I2C SCL and SDA lines are connected to the I2C SCL and SDA pins. When '1', I2C SCL and SDA lines are routed internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
pub struct LOOPBACK_R(crate::FieldReader<bool, bool>);
impl LOOPBACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOOPBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOPBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPBACK` writer - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', the I2C SCL and SDA lines are connected to the I2C SCL and SDA pins. When '1', I2C SCL and SDA lines are routed internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
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
#[doc = "Field `SLAVE_MODE` reader - Slave mode enabled ('1') or not ('0')."]
pub struct SLAVE_MODE_R(crate::FieldReader<bool, bool>);
impl SLAVE_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_MODE` writer - Slave mode enabled ('1') or not ('0')."]
pub struct SLAVE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_MODE_W<'a> {
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
#[doc = "Field `MASTER_MODE` reader - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
pub struct MASTER_MODE_R(crate::FieldReader<bool, bool>);
impl MASTER_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_MODE` writer - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
pub struct MASTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_MODE_W<'a> {
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
    #[doc = "Bits 0:3 - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period. The valid range is \\[5, 15\\]
with input signal median filtering and \\[4, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular interface (IF) high time to guarantee functional correct behavior. With input signal median filtering, the IF high time should be >= 6 IP clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF high time should be >= 5 IP clock cycles and <= 16 IP clock cycles."]
    #[inline(always)]
    pub fn high_phase_ovs(&self) -> HIGH_PHASE_OVS_R {
        HIGH_PHASE_OVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period. The valid range is \\[7, 15\\]
with input signal median filtering and \\[6, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular (no stretching) interface (IF) low time to guarantee functional correct behavior. With input signal median filtering, the IF low time should be >= 8 IP clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF low time should be >= 7 IP clock cycles and <= 16 IP clock cycles."]
    #[inline(always)]
    pub fn low_phase_ovs(&self) -> LOW_PHASE_OVS_R {
        LOW_PHASE_OVS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
    #[inline(always)]
    pub fn m_ready_data_ack(&self) -> M_READY_DATA_ACK_R {
        M_READY_DATA_ACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn m_not_ready_data_nack(&self) -> M_NOT_READY_DATA_NACK_R {
        M_NOT_READY_DATA_NACK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure."]
    #[inline(always)]
    pub fn s_general_ignore(&self) -> S_GENERAL_IGNORE_R {
        S_GENERAL_IGNORE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
    #[inline(always)]
    pub fn s_ready_addr_ack(&self) -> S_READY_ADDR_ACK_R {
        S_READY_ADDR_ACK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
    #[inline(always)]
    pub fn s_ready_data_ack(&self) -> S_READY_DATA_ACK_R {
        S_READY_DATA_ACK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - For internally clocked logic (EC_AM is '0' and EC_OP is '0') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when: - EC_AM is '0', EC_OP is '0' and non EZ mode. Functionality is as follows: - 1: a received (matching) slave address is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full). For externally clocked logic (EC_AM is '1') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when (NOT used when EC_AM is '1' and EC_OP is '1' and address match and EZ mode): - EC_AM is '1' and EC_OP is '0'. - EC_AM is '1' and general call address match. - EC_AM is '1' and non EZ mode. Functionality is as follows: - 1: a received (matching or general) slave address is always immediately NACK'd. There are two possibilities: 1). the internally clocked logic is enabled (we are in Active system power mode) and it handles the rest of the current transfer. In this case the I2C master will not observe the NACK. 2). the internally clocked logic is not enabled (we are in DeepSleep system power mode). In this case the I2C master will observe the NACK and may retry the transfer in the future (which gives the internally clocked logic the time to wake up from DeepSleep system power mode). - 0: clock stretching is performed (till the internally clocked logic takes over). The internally clocked logic will handle the ongoing transfer as soon as it is enabled."]
    #[inline(always)]
    pub fn s_not_ready_addr_nack(&self) -> S_NOT_READY_ADDR_NACK_R {
        S_NOT_READY_ADDR_NACK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - For internally clocked logic only. Only used when: - non EZ mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn s_not_ready_data_nack(&self) -> S_NOT_READY_DATA_NACK_R {
        S_NOT_READY_DATA_NACK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', the I2C SCL and SDA lines are connected to the I2C SCL and SDA pins. When '1', I2C SCL and SDA lines are routed internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Slave mode enabled ('1') or not ('0')."]
    #[inline(always)]
    pub fn slave_mode(&self) -> SLAVE_MODE_R {
        SLAVE_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period. The valid range is \\[5, 15\\]
with input signal median filtering and \\[4, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular interface (IF) high time to guarantee functional correct behavior. With input signal median filtering, the IF high time should be >= 6 IP clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF high time should be >= 5 IP clock cycles and <= 16 IP clock cycles."]
    #[inline(always)]
    pub fn high_phase_ovs(&mut self) -> HIGH_PHASE_OVS_W {
        HIGH_PHASE_OVS_W { w: self }
    }
    #[doc = "Bits 4:7 - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period. The valid range is \\[7, 15\\]
with input signal median filtering and \\[6, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular (no stretching) interface (IF) low time to guarantee functional correct behavior. With input signal median filtering, the IF low time should be >= 8 IP clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF low time should be >= 7 IP clock cycles and <= 16 IP clock cycles."]
    #[inline(always)]
    pub fn low_phase_ovs(&mut self) -> LOW_PHASE_OVS_W {
        LOW_PHASE_OVS_W { w: self }
    }
    #[doc = "Bit 8 - When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
    #[inline(always)]
    pub fn m_ready_data_ack(&mut self) -> M_READY_DATA_ACK_W {
        M_READY_DATA_ACK_W { w: self }
    }
    #[doc = "Bit 9 - When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn m_not_ready_data_nack(&mut self) -> M_NOT_READY_DATA_NACK_W {
        M_NOT_READY_DATA_NACK_W { w: self }
    }
    #[doc = "Bit 11 - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure."]
    #[inline(always)]
    pub fn s_general_ignore(&mut self) -> S_GENERAL_IGNORE_W {
        S_GENERAL_IGNORE_W { w: self }
    }
    #[doc = "Bit 12 - When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
    #[inline(always)]
    pub fn s_ready_addr_ack(&mut self) -> S_READY_ADDR_ACK_W {
        S_READY_ADDR_ACK_W { w: self }
    }
    #[doc = "Bit 13 - When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
    #[inline(always)]
    pub fn s_ready_data_ack(&mut self) -> S_READY_DATA_ACK_W {
        S_READY_DATA_ACK_W { w: self }
    }
    #[doc = "Bit 14 - For internally clocked logic (EC_AM is '0' and EC_OP is '0') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when: - EC_AM is '0', EC_OP is '0' and non EZ mode. Functionality is as follows: - 1: a received (matching) slave address is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full). For externally clocked logic (EC_AM is '1') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when (NOT used when EC_AM is '1' and EC_OP is '1' and address match and EZ mode): - EC_AM is '1' and EC_OP is '0'. - EC_AM is '1' and general call address match. - EC_AM is '1' and non EZ mode. Functionality is as follows: - 1: a received (matching or general) slave address is always immediately NACK'd. There are two possibilities: 1). the internally clocked logic is enabled (we are in Active system power mode) and it handles the rest of the current transfer. In this case the I2C master will not observe the NACK. 2). the internally clocked logic is not enabled (we are in DeepSleep system power mode). In this case the I2C master will observe the NACK and may retry the transfer in the future (which gives the internally clocked logic the time to wake up from DeepSleep system power mode). - 0: clock stretching is performed (till the internally clocked logic takes over). The internally clocked logic will handle the ongoing transfer as soon as it is enabled."]
    #[inline(always)]
    pub fn s_not_ready_addr_nack(&mut self) -> S_NOT_READY_ADDR_NACK_W {
        S_NOT_READY_ADDR_NACK_W { w: self }
    }
    #[doc = "Bit 15 - For internally clocked logic only. Only used when: - non EZ mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn s_not_ready_data_nack(&mut self) -> S_NOT_READY_DATA_NACK_W {
        S_NOT_READY_DATA_NACK_W { w: self }
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', the I2C SCL and SDA lines are connected to the I2C SCL and SDA pins. When '1', I2C SCL and SDA lines are routed internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bit 30 - Slave mode enabled ('1') or not ('0')."]
    #[inline(always)]
    pub fn slave_mode(&mut self) -> SLAVE_MODE_W {
        SLAVE_MODE_W { w: self }
    }
    #[doc = "Bit 31 - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
    #[inline(always)]
    pub fn master_mode(&mut self) -> MASTER_MODE_W {
        MASTER_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ctrl](index.html) module"]
pub struct I2C_CTRL_SPEC;
impl crate::RegisterSpec for I2C_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_ctrl::R](R) reader structure"]
impl crate::Readable for I2C_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_ctrl::W](W) writer structure"]
impl crate::Writable for I2C_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CTRL to value 0xfb88"]
impl crate::Resettable for I2C_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfb88
    }
}
