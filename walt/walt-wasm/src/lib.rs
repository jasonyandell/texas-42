//! EXPLORATORY WASM surface — walt level-1 as a decision oracle for a
//! browser client (the plunge game). Sits below every evidentiary tier;
//! estimates, never receipts; not a P-A21 statement.
//!
//! One entry point, strings in / strings out: a line-oriented request
//! (see `api`) and a JSON response. Three request kinds:
//!
//!   - `play`    — evaluate every legal play at the viewer's information
//!     state (level-1, saturation-tie refinement, banked-correct PiKey)
//!     and choose; replies also carry walt's independently derived trick
//!     leader and team points so the client can assert rules conformance
//!     on every decision, exactly like the arena bridge.
//!   - `bid`     — the baseline auction rule over bid curves: price all
//!     nine declarations at the minimum viable bid over common random
//!     worlds; pass if the best is below theta (default 1/2), else walk
//!     the best declaration up while P(make b) >= theta.
//!   - `declare` — name trump for a won contract: price all nine
//!     declarations at the contract bid, saturation ties refined on
//!     fresh larger samples, never index-broken.
//!
//! Straight points-and-marks 42 only (pip trumps 0..6, doubles, no-trump);
//! the client falls back to its own AI for anything else. No floats, no
//! unsafe — the wasm boundary works through owned thread-local buffers.

pub mod api;

#[cfg(target_arch = "wasm32")]
// The only "unsafe" here is the `#[no_mangle]` export attribute (stable
// symbol names for the host); there are no unsafe blocks in this crate.
#[allow(unsafe_code)]
mod abi {
    //! The wasm ABI: no unsafe, no imports. The host writes the request
    //! into a buffer this module owns, calls `walt_call`, then reads the
    //! response out of a second owned buffer.
    //!
    //! Protocol (all offsets into the exported linear memory):
    //!   1. `walt_in_prepare(len)` -> in_ptr  (sizes the input buffer)
    //!   2. host writes `len` request bytes (UTF-8) at `in_ptr`
    //!   3. `walt_call()` -> out_len
    //!   4. `walt_out_ptr()` -> out_ptr  (re-read AFTER walt_call: memory
    //!      may have grown and moved)
    //!   5. host reads `out_len` response bytes at `out_ptr`

    use std::cell::RefCell;

    thread_local! {
        static INBUF: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
        static OUTBUF: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    }

    #[no_mangle]
    pub extern "C" fn walt_in_prepare(len: u32) -> u32 {
        INBUF.with(|b| {
            let mut v = b.borrow_mut();
            v.clear();
            v.resize(len as usize, 0);
            v.as_ptr() as u32
        })
    }

    #[no_mangle]
    pub extern "C" fn walt_call() -> u32 {
        let req = INBUF.with(|b| String::from_utf8_lossy(&b.borrow()).into_owned());
        let resp = crate::api::handle(&req);
        OUTBUF.with(|b| {
            let mut v = b.borrow_mut();
            *v = resp.into_bytes();
            v.len() as u32
        })
    }

    #[no_mangle]
    pub extern "C" fn walt_out_ptr() -> u32 {
        OUTBUF.with(|b| b.borrow().as_ptr() as u32)
    }
}
