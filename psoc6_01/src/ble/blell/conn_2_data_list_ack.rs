#[doc = "Register `CONN_2_DATA_LIST_ACK` reader"]
pub struct R(crate::R<CONN_2_DATA_LIST_ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_2_DATA_LIST_ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_2_DATA_LIST_ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_2_DATA_LIST_ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_2_DATA_LIST_ACK` writer"]
pub struct W(crate::W<CONN_2_DATA_LIST_ACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_2_DATA_LIST_ACK_SPEC>;
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
impl From<crate::W<CONN_2_DATA_LIST_ACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_2_DATA_LIST_ACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIST_INDEX__TX_ACK_3_0_C1` reader - Write: Indicates the buffer index for which the ACK bit is being updated by firmware. The default number of buffers in the IP is 5. The index range is 0-3. Read: Reads TX_ACK\\[3:0\\]
If a particular bit is set, then the packet in the selected buffer has been transmitted (at least once) by the hardware and hardware is waiting for acknowledgement. Example1 : If the read value is : 0x03, then packets in FIFO-0 and FIFO-1 are acknowledged by the remote device. These acknowledgements are pending to be processed by firmware. Example2 : If the read value is : 0x02, then packet FIFO-1 is acknowledged by the remote device. This acknowledgement is pending to be processed by firmware."]
pub struct LIST_INDEX__TX_ACK_3_0_C1_R(crate::FieldReader<u8, u8>);
impl LIST_INDEX__TX_ACK_3_0_C1_R {
    pub(crate) fn new(bits: u8) -> Self {
        LIST_INDEX__TX_ACK_3_0_C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIST_INDEX__TX_ACK_3_0_C1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIST_INDEX__TX_ACK_3_0_C1` writer - Write: Indicates the buffer index for which the ACK bit is being updated by firmware. The default number of buffers in the IP is 5. The index range is 0-3. Read: Reads TX_ACK\\[3:0\\]
If a particular bit is set, then the packet in the selected buffer has been transmitted (at least once) by the hardware and hardware is waiting for acknowledgement. Example1 : If the read value is : 0x03, then packets in FIFO-0 and FIFO-1 are acknowledged by the remote device. These acknowledgements are pending to be processed by firmware. Example2 : If the read value is : 0x02, then packet FIFO-1 is acknowledged by the remote device. This acknowledgement is pending to be processed by firmware."]
pub struct LIST_INDEX__TX_ACK_3_0_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> LIST_INDEX__TX_ACK_3_0_C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `SET_CLEAR_C1` writer - Write: Firmware uses the field to clear and ACK bit in the hardware to indicate that the acknowledgement for the transmit packet has been received and processed by firmware. Firmware clears the ACK bit in the hardware by writing in this register only after the acknowledgement is processed successfully by firmware. For clearing ack for a packet transmitted in fifo-index : '3', firm-ware will write '3' in the 'list-index' field and set this bit (BIT7) to 0. This is the indication that the corresponding packet buffer identi-fied by List-Index is cleared of previous transmission and can be re-used for another packet from now on. The ACK bit in hardware is set by hardware when it has success-fully transmitted a packet."]
pub struct SET_CLEAR_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_CLEAR_C1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Write: Indicates the buffer index for which the ACK bit is being updated by firmware. The default number of buffers in the IP is 5. The index range is 0-3. Read: Reads TX_ACK\\[3:0\\]
If a particular bit is set, then the packet in the selected buffer has been transmitted (at least once) by the hardware and hardware is waiting for acknowledgement. Example1 : If the read value is : 0x03, then packets in FIFO-0 and FIFO-1 are acknowledged by the remote device. These acknowledgements are pending to be processed by firmware. Example2 : If the read value is : 0x02, then packet FIFO-1 is acknowledged by the remote device. This acknowledgement is pending to be processed by firmware."]
    #[inline(always)]
    pub fn list_index__tx_ack_3_0_c1(&self) -> LIST_INDEX__TX_ACK_3_0_C1_R {
        LIST_INDEX__TX_ACK_3_0_C1_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Write: Indicates the buffer index for which the ACK bit is being updated by firmware. The default number of buffers in the IP is 5. The index range is 0-3. Read: Reads TX_ACK\\[3:0\\]
If a particular bit is set, then the packet in the selected buffer has been transmitted (at least once) by the hardware and hardware is waiting for acknowledgement. Example1 : If the read value is : 0x03, then packets in FIFO-0 and FIFO-1 are acknowledged by the remote device. These acknowledgements are pending to be processed by firmware. Example2 : If the read value is : 0x02, then packet FIFO-1 is acknowledged by the remote device. This acknowledgement is pending to be processed by firmware."]
    #[inline(always)]
    pub fn list_index__tx_ack_3_0_c1(&mut self) -> LIST_INDEX__TX_ACK_3_0_C1_W {
        LIST_INDEX__TX_ACK_3_0_C1_W { w: self }
    }
    #[doc = "Bit 7 - Write: Firmware uses the field to clear and ACK bit in the hardware to indicate that the acknowledgement for the transmit packet has been received and processed by firmware. Firmware clears the ACK bit in the hardware by writing in this register only after the acknowledgement is processed successfully by firmware. For clearing ack for a packet transmitted in fifo-index : '3', firm-ware will write '3' in the 'list-index' field and set this bit (BIT7) to 0. This is the indication that the corresponding packet buffer identi-fied by List-Index is cleared of previous transmission and can be re-used for another packet from now on. The ACK bit in hardware is set by hardware when it has success-fully transmitted a packet."]
    #[inline(always)]
    pub fn set_clear_c1(&mut self) -> SET_CLEAR_C1_W {
        SET_CLEAR_C1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data list ack update and status for connection 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_2_data_list_ack](index.html) module"]
pub struct CONN_2_DATA_LIST_ACK_SPEC;
impl crate::RegisterSpec for CONN_2_DATA_LIST_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_2_data_list_ack::R](R) reader structure"]
impl crate::Readable for CONN_2_DATA_LIST_ACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_2_data_list_ack::W](W) writer structure"]
impl crate::Writable for CONN_2_DATA_LIST_ACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_2_DATA_LIST_ACK to value 0"]
impl crate::Resettable for CONN_2_DATA_LIST_ACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
