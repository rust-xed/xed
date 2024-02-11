use xed_sys::*;

crate::macros::xed_enum! {
    pub enum Flag {
        /// Overflow flag
        OF => XED_FLAG_of,

        /// Sign flag
        SF => XED_FLAG_sf,

        /// Zero flag
        ZF => XED_FLAG_zf,

        /// Auxiliary flag
        AF => XED_FLAG_af,

        /// Parity flag
        PF => XED_FLAG_pf,

        /// Carry flag
        CF => XED_FLAG_cf,

        /// Direction flag
        DF => XED_FLAG_df,

        /// Virtual interrupt flag
        VIF => XED_FLAG_vif,

        /// I/O privilege level
        IOPL => XED_FLAG_iopl,

        /// Interrupt flag
        IF => XED_FLAG_if,

        /// Virtual-8086 mode
        VM => XED_FLAG_vm,

        /// Resume flag
        RF => XED_FLAG_rf,

        /// Nested task
        NT => XED_FLAG_nt,

        /// Traf flag
        TF => XED_FLAG_tf,

        /// ID flag
        ID => XED_FLAG_id,

        /// Virtual interrupt pending
        VIP => XED_FLAG_vip,

        /// x86 FC0 flag
        FC0 => XED_FLAG_fc0,

        /// x87 FC1 flag
        FC1 => XED_FLAG_fc1,

        /// x87 FC2 flag
        FC2 => XED_FLAG_fc2,

        /// x87 FC3 flag
        FC3 => XED_FLAG_fc3
    }
}
