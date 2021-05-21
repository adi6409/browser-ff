//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/autoplay/nsIAutoplay.idl
//


/// `interface nsIAutoplay : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAutoplay {
    vtable: *const nsIAutoplayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAutoplay.
unsafe impl XpCom for nsIAutoplay {
    const IID: nsIID = nsID(0x048a24f6, 0xc4d6, 0x47bc,
        [0xbe, 0xa2, 0xf6, 0x03, 0x8d, 0x1d, 0xb8, 0x0a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAutoplay {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAutoplay.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAutoplayCoerce {
    /// Cheaply cast a value of this type from a `nsIAutoplay`.
    fn coerce_from(v: &nsIAutoplay) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAutoplayCoerce for nsIAutoplay {
    #[inline]
    fn coerce_from(v: &nsIAutoplay) -> &Self {
        v
    }
}

impl nsIAutoplay {
    /// Cast this `nsIAutoplay` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAutoplayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAutoplay {
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
impl<T: nsISupportsCoerce> nsIAutoplayCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoplay) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAutoplay
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAutoplayVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAutoplay {

    pub const ALLOWED: i64 = 0;


    pub const BLOCKED: i64 = 1;


    pub const BLOCKED_ALL: i64 = 5;


}


