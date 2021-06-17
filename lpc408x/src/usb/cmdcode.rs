#[doc = "Register `CMDCODE` writer"]
pub struct W(crate::W<CMDCODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDCODE_SPEC>;
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
impl From<crate::W<CMDCODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDCODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The command phase:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_PHASE_AW {
    #[doc = "2: Read"]
    READ = 2,
    #[doc = "1: Write"]
    WRITE = 1,
    #[doc = "5: Command"]
    COMMAND = 5,
}
impl From<CMD_PHASE_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMD_PHASE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD_PHASE` writer - The command phase:"]
pub struct CMD_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_PHASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_PHASE_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(CMD_PHASE_AW::READ)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(CMD_PHASE_AW::WRITE)
    }
    #[doc = "Command"]
    #[inline(always)]
    pub fn command(self) -> &'a mut W {
        self.variant(CMD_PHASE_AW::COMMAND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CMD_CODE_WDATA` writer - This is a multi-purpose field. When CMD_PHASE is Command or Read, this field contains the code for the command (CMD_CODE). When CMD_PHASE is Write, this field contains the command write data (CMD_WDATA)."]
pub struct CMD_CODE_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_CODE_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 8:15 - The command phase:"]
    #[inline(always)]
    pub fn cmd_phase(&mut self) -> CMD_PHASE_W {
        CMD_PHASE_W { w: self }
    }
    #[doc = "Bits 16:23 - This is a multi-purpose field. When CMD_PHASE is Command or Read, this field contains the code for the command (CMD_CODE). When CMD_PHASE is Write, this field contains the command write data (CMD_WDATA)."]
    #[inline(always)]
    pub fn cmd_code_wdata(&mut self) -> CMD_CODE_WDATA_W {
        CMD_CODE_WDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Command Code\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdcode](index.html) module"]
pub struct CMDCODE_SPEC;
impl crate::RegisterSpec for CMDCODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmdcode::W](W) writer structure"]
impl crate::Writable for CMDCODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDCODE to value 0"]
impl crate::Resettable for CMDCODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
