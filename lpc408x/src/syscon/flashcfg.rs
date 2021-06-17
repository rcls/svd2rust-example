#[doc = "Register `FLASHCFG` reader"]
pub struct R(crate::R<FLASHCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHCFG` writer"]
pub struct W(crate::W<FLASHCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHCFG_SPEC>;
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
impl From<crate::W<FLASHCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASHTIM_A {
    #[doc = "0: Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock with power boost off."]
    FLASH_ACCESSES_USE_1 = 0,
    #[doc = "1: Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    FLASH_ACCESSES_USE_2 = 1,
    #[doc = "2: Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    FLASH_ACCESSES_USE_3 = 2,
    #[doc = "3: Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    FLASH_ACCESSES_USE_4 = 3,
    #[doc = "4: Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    FLASH_ACCESSES_USE_5 = 4,
    #[doc = "5: Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    FLASH_ACCESSES_USE_6 = 5,
}
impl From<FLASHTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHTIM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASHTIM` reader - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved."]
pub struct FLASHTIM_R(crate::FieldReader<u8, FLASHTIM_A>);
impl FLASHTIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLASHTIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASHTIM_A> {
        match self.bits {
            0 => Some(FLASHTIM_A::FLASH_ACCESSES_USE_1),
            1 => Some(FLASHTIM_A::FLASH_ACCESSES_USE_2),
            2 => Some(FLASHTIM_A::FLASH_ACCESSES_USE_3),
            3 => Some(FLASHTIM_A::FLASH_ACCESSES_USE_4),
            4 => Some(FLASHTIM_A::FLASH_ACCESSES_USE_5),
            5 => Some(FLASHTIM_A::FLASH_ACCESSES_USE_6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSES_USE_1`"]
    #[inline(always)]
    pub fn is_flash_accesses_use_1(&self) -> bool {
        **self == FLASHTIM_A::FLASH_ACCESSES_USE_1
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSES_USE_2`"]
    #[inline(always)]
    pub fn is_flash_accesses_use_2(&self) -> bool {
        **self == FLASHTIM_A::FLASH_ACCESSES_USE_2
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSES_USE_3`"]
    #[inline(always)]
    pub fn is_flash_accesses_use_3(&self) -> bool {
        **self == FLASHTIM_A::FLASH_ACCESSES_USE_3
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSES_USE_4`"]
    #[inline(always)]
    pub fn is_flash_accesses_use_4(&self) -> bool {
        **self == FLASHTIM_A::FLASH_ACCESSES_USE_4
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSES_USE_5`"]
    #[inline(always)]
    pub fn is_flash_accesses_use_5(&self) -> bool {
        **self == FLASHTIM_A::FLASH_ACCESSES_USE_5
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSES_USE_6`"]
    #[inline(always)]
    pub fn is_flash_accesses_use_6(&self) -> bool {
        **self == FLASHTIM_A::FLASH_ACCESSES_USE_6
    }
}
impl core::ops::Deref for FLASHTIM_R {
    type Target = crate::FieldReader<u8, FLASHTIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHTIM` writer - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved."]
pub struct FLASHTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHTIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn flash_accesses_use_1(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASH_ACCESSES_USE_1)
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn flash_accesses_use_2(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASH_ACCESSES_USE_2)
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn flash_accesses_use_3(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASH_ACCESSES_USE_3)
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    #[inline(always)]
    pub fn flash_accesses_use_4(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASH_ACCESSES_USE_4)
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn flash_accesses_use_5(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASH_ACCESSES_USE_5)
    }
    #[doc = "Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    #[inline(always)]
    pub fn flash_accesses_use_6(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASH_ACCESSES_USE_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> FLASHTIM_W {
        FLASHTIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcfg](index.html) module"]
pub struct FLASHCFG_SPEC;
impl crate::RegisterSpec for FLASHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashcfg::R](R) reader structure"]
impl crate::Readable for FLASHCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashcfg::W](W) writer structure"]
impl crate::Writable for FLASHCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHCFG to value 0"]
impl crate::Resettable for FLASHCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
