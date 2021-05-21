//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationRequestUIGlue.idl
//


/// `interface nsIPresentationRequestUIGlue : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationRequestUIGlue {
    vtable: *const nsIPresentationRequestUIGlueVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationRequestUIGlue.
unsafe impl XpCom for nsIPresentationRequestUIGlue {
    const IID: nsIID = nsID(0xfaa45119, 0x6fb5, 0x496c,
        [0xaa, 0x4c, 0xf7, 0x40, 0x17, 0x7a, 0x38, 0xb5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationRequestUIGlue {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationRequestUIGlue.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationRequestUIGlueCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationRequestUIGlue`.
    fn coerce_from(v: &nsIPresentationRequestUIGlue) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationRequestUIGlueCoerce for nsIPresentationRequestUIGlue {
    #[inline]
    fn coerce_from(v: &nsIPresentationRequestUIGlue) -> &Self {
        v
    }
}

impl nsIPresentationRequestUIGlue {
    /// Cast this `nsIPresentationRequestUIGlue` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationRequestUIGlueCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationRequestUIGlue {
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
impl<T: nsISupportsCoerce> nsIPresentationRequestUIGlueCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationRequestUIGlue) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationRequestUIGlue
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationRequestUIGlueVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Promise sendRequest (in AString url, in AString sessionId, in nsIPresentationDevice device); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub SendRequest: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationRequestUIGlue {


    /// `Promise sendRequest (in AString url, in AString sessionId, in nsIPresentationDevice device);`
    const _SendRequest: () = ();

}


