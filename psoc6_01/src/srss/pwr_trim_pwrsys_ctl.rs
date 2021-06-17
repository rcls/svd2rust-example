#[doc = "Register `PWR_TRIM_PWRSYS_CTL` reader"]
pub struct R(crate::R<PWR_TRIM_PWRSYS_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_TRIM_PWRSYS_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_TRIM_PWRSYS_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_TRIM_PWRSYS_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_TRIM_PWRSYS_CTL` writer"]
pub struct W(crate::W<PWR_TRIM_PWRSYS_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_TRIM_PWRSYS_CTL_SPEC>;
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
impl From<crate::W<PWR_TRIM_PWRSYS_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_TRIM_PWRSYS_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT_REG_TRIM` reader - Trim for the Active-Regulator. This sets the output voltage level. This register is only reset by XRES/POR/BOD/HIBERNATE. Two voltages are supported: 0.9V and 1.1V. The codes for these are stored in SFLASH_LDO_0P9V_TRIM and SFLASH_LDO_1P1V_TRIM, respectively."]
pub struct ACT_REG_TRIM_R(crate::FieldReader<u8, u8>);
impl ACT_REG_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACT_REG_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_REG_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_REG_TRIM` writer - Trim for the Active-Regulator. This sets the output voltage level. This register is only reset by XRES/POR/BOD/HIBERNATE. Two voltages are supported: 0.9V and 1.1V. The codes for these are stored in SFLASH_LDO_0P9V_TRIM and SFLASH_LDO_1P1V_TRIM, respectively."]
pub struct ACT_REG_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REG_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `ACT_REG_BOOST` reader - Controls the tradeoff between output current and internal operating current for the Active Regulator. The maximum output current depends on the silicon implementation, but an application may limit its maximum current to less than that. This may allow a reduction in the internal operating current of the regulator. The regulator internal operating current depends on the boost setting: 2'b00: 50uA 2'b01: 100uA 2'b10: 150uA 2'b11: 200uA The allowed setting is a lookup table based on the chip-specific maximum (set in factory) and an application-specific maximum (set by customer). The defaults are set assuming the application consumes the maximum allowed by the chip. 50mA chip: 2'b00 (default); 100mA chip: 2'b00 (default); 150mA chip: 50..100mA app => 2'b00, 150mA app => 2'b01 (default); 200mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200mA app => 2'b10 (default); 250mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10 (default); 300mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10, 300mA app => 2'b11 (default); This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub struct ACT_REG_BOOST_R(crate::FieldReader<u8, u8>);
impl ACT_REG_BOOST_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACT_REG_BOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_REG_BOOST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_REG_BOOST` writer - Controls the tradeoff between output current and internal operating current for the Active Regulator. The maximum output current depends on the silicon implementation, but an application may limit its maximum current to less than that. This may allow a reduction in the internal operating current of the regulator. The regulator internal operating current depends on the boost setting: 2'b00: 50uA 2'b01: 100uA 2'b10: 150uA 2'b11: 200uA The allowed setting is a lookup table based on the chip-specific maximum (set in factory) and an application-specific maximum (set by customer). The defaults are set assuming the application consumes the maximum allowed by the chip. 50mA chip: 2'b00 (default); 100mA chip: 2'b00 (default); 150mA chip: 50..100mA app => 2'b00, 150mA app => 2'b01 (default); 200mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200mA app => 2'b10 (default); 250mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10 (default); 300mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10, 300mA app => 2'b11 (default); This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub struct ACT_REG_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REG_BOOST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Trim for the Active-Regulator. This sets the output voltage level. This register is only reset by XRES/POR/BOD/HIBERNATE. Two voltages are supported: 0.9V and 1.1V. The codes for these are stored in SFLASH_LDO_0P9V_TRIM and SFLASH_LDO_1P1V_TRIM, respectively."]
    #[inline(always)]
    pub fn act_reg_trim(&self) -> ACT_REG_TRIM_R {
        ACT_REG_TRIM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 30:31 - Controls the tradeoff between output current and internal operating current for the Active Regulator. The maximum output current depends on the silicon implementation, but an application may limit its maximum current to less than that. This may allow a reduction in the internal operating current of the regulator. The regulator internal operating current depends on the boost setting: 2'b00: 50uA 2'b01: 100uA 2'b10: 150uA 2'b11: 200uA The allowed setting is a lookup table based on the chip-specific maximum (set in factory) and an application-specific maximum (set by customer). The defaults are set assuming the application consumes the maximum allowed by the chip. 50mA chip: 2'b00 (default); 100mA chip: 2'b00 (default); 150mA chip: 50..100mA app => 2'b00, 150mA app => 2'b01 (default); 200mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200mA app => 2'b10 (default); 250mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10 (default); 300mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10, 300mA app => 2'b11 (default); This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn act_reg_boost(&self) -> ACT_REG_BOOST_R {
        ACT_REG_BOOST_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for the Active-Regulator. This sets the output voltage level. This register is only reset by XRES/POR/BOD/HIBERNATE. Two voltages are supported: 0.9V and 1.1V. The codes for these are stored in SFLASH_LDO_0P9V_TRIM and SFLASH_LDO_1P1V_TRIM, respectively."]
    #[inline(always)]
    pub fn act_reg_trim(&mut self) -> ACT_REG_TRIM_W {
        ACT_REG_TRIM_W { w: self }
    }
    #[doc = "Bits 30:31 - Controls the tradeoff between output current and internal operating current for the Active Regulator. The maximum output current depends on the silicon implementation, but an application may limit its maximum current to less than that. This may allow a reduction in the internal operating current of the regulator. The regulator internal operating current depends on the boost setting: 2'b00: 50uA 2'b01: 100uA 2'b10: 150uA 2'b11: 200uA The allowed setting is a lookup table based on the chip-specific maximum (set in factory) and an application-specific maximum (set by customer). The defaults are set assuming the application consumes the maximum allowed by the chip. 50mA chip: 2'b00 (default); 100mA chip: 2'b00 (default); 150mA chip: 50..100mA app => 2'b00, 150mA app => 2'b01 (default); 200mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200mA app => 2'b10 (default); 250mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10 (default); 300mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10, 300mA app => 2'b11 (default); This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn act_reg_boost(&mut self) -> ACT_REG_BOOST_W {
        ACT_REG_BOOST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power System Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_trim_pwrsys_ctl](index.html) module"]
pub struct PWR_TRIM_PWRSYS_CTL_SPEC;
impl crate::RegisterSpec for PWR_TRIM_PWRSYS_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_trim_pwrsys_ctl::R](R) reader structure"]
impl crate::Readable for PWR_TRIM_PWRSYS_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_trim_pwrsys_ctl::W](W) writer structure"]
impl crate::Writable for PWR_TRIM_PWRSYS_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_TRIM_PWRSYS_CTL to value 0x17"]
impl crate::Resettable for PWR_TRIM_PWRSYS_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x17
    }
}
