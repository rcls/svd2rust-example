#[doc = "Register `ENDOFTABLE` reader"]
pub struct R(crate::R<ENDOFTABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDOFTABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDOFTABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDOFTABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDOFTABLE` writer"]
pub struct W(crate::W<ENDOFTABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDOFTABLE_SPEC>;
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
impl From<crate::W<ENDOFTABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDOFTABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDOFTABLE` reader - The address above the last active address in the last active AF table. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register. If the eFCAN bit in the AFMR is 0, the largest value that should be written to this register is 0x800, which allows the last word (address 0x7FC) in AF Lookup Table RAM to be used. If the eFCAN bit in the AFMR is 1, this value marks the start of the area of Acceptance Filter RAM, into which the Acceptance Filter will automatically receive messages for selected IDs on selected CAN buses. In this case, the maximum value that should be written to this register is 0x800 minus 6 times the value in SFF_sa. This allows 12 bytes of message storage between this address and the end of Acceptance Filter RAM, for each Standard ID that is specified between the start of Acceptance Filter RAM, and the next active AF table."]
pub struct ENDOFTABLE_R(crate::FieldReader<u16, u16>);
impl ENDOFTABLE_R {
    pub(crate) fn new(bits: u16) -> Self {
        ENDOFTABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDOFTABLE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDOFTABLE` writer - The address above the last active address in the last active AF table. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register. If the eFCAN bit in the AFMR is 0, the largest value that should be written to this register is 0x800, which allows the last word (address 0x7FC) in AF Lookup Table RAM to be used. If the eFCAN bit in the AFMR is 1, this value marks the start of the area of Acceptance Filter RAM, into which the Acceptance Filter will automatically receive messages for selected IDs on selected CAN buses. In this case, the maximum value that should be written to this register is 0x800 minus 6 times the value in SFF_sa. This allows 12 bytes of message storage between this address and the end of Acceptance Filter RAM, for each Standard ID that is specified between the start of Acceptance Filter RAM, and the next active AF table."]
pub struct ENDOFTABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDOFTABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 2)) | ((value as u32 & 0x03ff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:11 - The address above the last active address in the last active AF table. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register. If the eFCAN bit in the AFMR is 0, the largest value that should be written to this register is 0x800, which allows the last word (address 0x7FC) in AF Lookup Table RAM to be used. If the eFCAN bit in the AFMR is 1, this value marks the start of the area of Acceptance Filter RAM, into which the Acceptance Filter will automatically receive messages for selected IDs on selected CAN buses. In this case, the maximum value that should be written to this register is 0x800 minus 6 times the value in SFF_sa. This allows 12 bytes of message storage between this address and the end of Acceptance Filter RAM, for each Standard ID that is specified between the start of Acceptance Filter RAM, and the next active AF table."]
    #[inline(always)]
    pub fn endoftable(&self) -> ENDOFTABLE_R {
        ENDOFTABLE_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:11 - The address above the last active address in the last active AF table. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register. If the eFCAN bit in the AFMR is 0, the largest value that should be written to this register is 0x800, which allows the last word (address 0x7FC) in AF Lookup Table RAM to be used. If the eFCAN bit in the AFMR is 1, this value marks the start of the area of Acceptance Filter RAM, into which the Acceptance Filter will automatically receive messages for selected IDs on selected CAN buses. In this case, the maximum value that should be written to this register is 0x800 minus 6 times the value in SFF_sa. This allows 12 bytes of message storage between this address and the end of Acceptance Filter RAM, for each Standard ID that is specified between the start of Acceptance Filter RAM, and the next active AF table."]
    #[inline(always)]
    pub fn endoftable(&mut self) -> ENDOFTABLE_W {
        ENDOFTABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "End of AF Tables register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endoftable](index.html) module"]
pub struct ENDOFTABLE_SPEC;
impl crate::RegisterSpec for ENDOFTABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endoftable::R](R) reader structure"]
impl crate::Readable for ENDOFTABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endoftable::W](W) writer structure"]
impl crate::Writable for ENDOFTABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENDOFTABLE to value 0"]
impl crate::Resettable for ENDOFTABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
