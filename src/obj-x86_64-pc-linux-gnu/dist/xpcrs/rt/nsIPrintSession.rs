//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPrintSession.idl
//


/// `interface nsIPrintSession : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrintSession {
    vtable: *const nsIPrintSessionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrintSession.
unsafe impl XpCom for nsIPrintSession {
    const IID: nsIID = nsID(0x424ae4bb, 0x10ca, 0x4f35,
        [0xb8, 0x4e, 0xea, 0xb8, 0x93, 0x32, 0x2d, 0xf4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrintSession {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrintSession.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrintSessionCoerce {
    /// Cheaply cast a value of this type from a `nsIPrintSession`.
    fn coerce_from(v: &nsIPrintSession) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrintSessionCoerce for nsIPrintSession {
    #[inline]
    fn coerce_from(v: &nsIPrintSession) -> &Self {
        v
    }
}

impl nsIPrintSession {
    /// Cast this `nsIPrintSession` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrintSessionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrintSession {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIPrintSessionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintSession) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrintSession
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrintSessionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [nostdcall,notxpcom] attribute RemotePrintJobChildPtr remotePrintJob; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetRemotePrintJob: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute RemotePrintJobChildPtr remotePrintJob; */
    /// Unable to generate binding because `native type mozilla::layout::RemotePrintJobChild unsupported`
    pub SetRemotePrintJob: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrintSession {

    /// ```text
    /// /**
    ///    * The remote print job is used for printing via the parent process.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute RemotePrintJobChildPtr remotePrintJob;`
    const _GetRemotePrintJob: () = ();

    /// ```text
    /// /**
    ///    * The remote print job is used for printing via the parent process.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute RemotePrintJobChildPtr remotePrintJob;`
    const _SetRemotePrintJob: () = ();

}


