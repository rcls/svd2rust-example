#[doc = "Register `PEER_ADDR_H` reader"]
pub struct R(crate::R<PEER_ADDR_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEER_ADDR_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEER_ADDR_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEER_ADDR_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEER_ADDR_H` writer"]
pub struct W(crate::W<PEER_ADDR_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEER_ADDR_H_SPEC>;
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
impl From<crate::W<PEER_ADDR_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEER_ADDR_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEER_ADDR_H` reader - Higher 16 bit of 48-bit address of the peer device. The peer address registers are used for multiple purposes. The register is written by firmware to provide the peer address to be used for a hardware procedure. When firmware reads the register, it reads back peer address values updated by hardware. While doing directed Advertising, the firmware writes the peer address of the device specified by the Di-rect_Address parameter of the LE_Set_Advertising_Parameters command. In non MMMS mode, While device is configured as an initiator without white list filtering, the peer address specified in the peer_address field of the create connection command is programmed into this register, which is used by hard-ware procedures. In non MMMS mode, While device is configured as an initiator and white list is enabled, firmware can read this register to get the address of the peer device from which connectable ADV packet was received and to which the connection is created. When a connection is created as a slave, the firmware can read this register to get the address of the peer de-vice to which connection is created."]
pub struct PEER_ADDR_H_R(crate::FieldReader<u16, u16>);
impl PEER_ADDR_H_R {
    pub(crate) fn new(bits: u16) -> Self {
        PEER_ADDR_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEER_ADDR_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEER_ADDR_H` writer - Higher 16 bit of 48-bit address of the peer device. The peer address registers are used for multiple purposes. The register is written by firmware to provide the peer address to be used for a hardware procedure. When firmware reads the register, it reads back peer address values updated by hardware. While doing directed Advertising, the firmware writes the peer address of the device specified by the Di-rect_Address parameter of the LE_Set_Advertising_Parameters command. In non MMMS mode, While device is configured as an initiator without white list filtering, the peer address specified in the peer_address field of the create connection command is programmed into this register, which is used by hard-ware procedures. In non MMMS mode, While device is configured as an initiator and white list is enabled, firmware can read this register to get the address of the peer device from which connectable ADV packet was received and to which the connection is created. When a connection is created as a slave, the firmware can read this register to get the address of the peer de-vice to which connection is created."]
pub struct PEER_ADDR_H_W<'a> {
    w: &'a mut W,
}
impl<'a> PEER_ADDR_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Higher 16 bit of 48-bit address of the peer device. The peer address registers are used for multiple purposes. The register is written by firmware to provide the peer address to be used for a hardware procedure. When firmware reads the register, it reads back peer address values updated by hardware. While doing directed Advertising, the firmware writes the peer address of the device specified by the Di-rect_Address parameter of the LE_Set_Advertising_Parameters command. In non MMMS mode, While device is configured as an initiator without white list filtering, the peer address specified in the peer_address field of the create connection command is programmed into this register, which is used by hard-ware procedures. In non MMMS mode, While device is configured as an initiator and white list is enabled, firmware can read this register to get the address of the peer device from which connectable ADV packet was received and to which the connection is created. When a connection is created as a slave, the firmware can read this register to get the address of the peer de-vice to which connection is created."]
    #[inline(always)]
    pub fn peer_addr_h(&self) -> PEER_ADDR_H_R {
        PEER_ADDR_H_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Higher 16 bit of 48-bit address of the peer device. The peer address registers are used for multiple purposes. The register is written by firmware to provide the peer address to be used for a hardware procedure. When firmware reads the register, it reads back peer address values updated by hardware. While doing directed Advertising, the firmware writes the peer address of the device specified by the Di-rect_Address parameter of the LE_Set_Advertising_Parameters command. In non MMMS mode, While device is configured as an initiator without white list filtering, the peer address specified in the peer_address field of the create connection command is programmed into this register, which is used by hard-ware procedures. In non MMMS mode, While device is configured as an initiator and white list is enabled, firmware can read this register to get the address of the peer device from which connectable ADV packet was received and to which the connection is created. When a connection is created as a slave, the firmware can read this register to get the address of the peer de-vice to which connection is created."]
    #[inline(always)]
    pub fn peer_addr_h(&mut self) -> PEER_ADDR_H_W {
        PEER_ADDR_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Higher 16 bit address of the peer device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_addr_h](index.html) module"]
pub struct PEER_ADDR_H_SPEC;
impl crate::RegisterSpec for PEER_ADDR_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peer_addr_h::R](R) reader structure"]
impl crate::Readable for PEER_ADDR_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peer_addr_h::W](W) writer structure"]
impl crate::Writable for PEER_ADDR_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEER_ADDR_H to value 0"]
impl crate::Resettable for PEER_ADDR_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
