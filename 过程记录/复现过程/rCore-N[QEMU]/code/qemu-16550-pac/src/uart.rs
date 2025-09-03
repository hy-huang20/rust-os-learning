#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 0x01],
    _reserved_1_dlh: [u8; 0x01],
    _reserved_2_fcr: [u8; 0x01],
    lcr: Lcr,
    mcr: Mcr,
    lsr: Lsr,
    msr: Msr,
    sch: Sch,
}
impl RegisterBlock {
    #[doc = "0x00 - UART Divisor Latch Low Register"]
    #[inline(always)]
    pub const fn dll(&self) -> &Dll {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - UART Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - UART Receive Buffer Register"]
    #[inline(always)]
    pub const fn rbr(&self) -> &Rbr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x01 - UART Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1).cast() }
    }
    #[doc = "0x01 - UART Divisor Latch High Register"]
    #[inline(always)]
    pub const fn dlh(&self) -> &Dlh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1).cast() }
    }
    #[doc = "0x02 - UART FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x02 - UART Interrupt Identity Register"]
    #[inline(always)]
    pub const fn iir(&self) -> &Iir {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x03 - UART Line Control Register"]
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    #[doc = "0x04 - UART Modem Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x05 - UART Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0x06 - UART Modem Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    #[doc = "0x07 - UART Scratch Register"]
    #[inline(always)]
    pub const fn sch(&self) -> &Sch {
        &self.sch
    }
}
#[doc = "rbr (r) register accessor: UART Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr`] module"]
#[doc(alias = "rbr")]
pub type Rbr = crate::Reg<rbr::RbrSpec>;
#[doc = "UART Receive Buffer Register"]
pub mod rbr;
#[doc = "thr (w) register accessor: UART Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`] module"]
#[doc(alias = "thr")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "UART Transmit Holding Register"]
pub mod thr;
#[doc = "dll (rw) register accessor: UART Divisor Latch Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll`] module"]
#[doc(alias = "dll")]
pub type Dll = crate::Reg<dll::DllSpec>;
#[doc = "UART Divisor Latch Low Register"]
pub mod dll;
#[doc = "dlh (rw) register accessor: UART Divisor Latch High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dlh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlh`] module"]
#[doc(alias = "dlh")]
pub type Dlh = crate::Reg<dlh::DlhSpec>;
#[doc = "UART Divisor Latch High Register"]
pub mod dlh;
#[doc = "ier (rw) register accessor: UART Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "ier")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "UART Interrupt Enable Register"]
pub mod ier;
#[doc = "iir (r) register accessor: UART Interrupt Identity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`] module"]
#[doc(alias = "iir")]
pub type Iir = crate::Reg<iir::IirSpec>;
#[doc = "UART Interrupt Identity Register"]
pub mod iir;
#[doc = "fcr (w) register accessor: UART FIFO Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"]
#[doc(alias = "fcr")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "UART FIFO Control Register"]
pub mod fcr;
#[doc = "lcr (rw) register accessor: UART Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`] module"]
#[doc(alias = "lcr")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
#[doc = "UART Line Control Register"]
pub mod lcr;
#[doc = "mcr (rw) register accessor: UART Modem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`] module"]
#[doc(alias = "mcr")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "UART Modem Control Register"]
pub mod mcr;
#[doc = "lsr (r) register accessor: UART Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`] module"]
#[doc(alias = "lsr")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "UART Line Status Register"]
pub mod lsr;
#[doc = "msr (r) register accessor: UART Modem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`] module"]
#[doc(alias = "msr")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "UART Modem Status Register"]
pub mod msr;
#[doc = "sch (rw) register accessor: UART Scratch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sch`] module"]
#[doc(alias = "sch")]
pub type Sch = crate::Reg<sch::SchSpec>;
#[doc = "UART Scratch Register"]
pub mod sch;
