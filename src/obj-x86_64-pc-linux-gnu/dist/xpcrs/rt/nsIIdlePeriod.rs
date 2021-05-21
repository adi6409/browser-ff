//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIIdlePeriod.idl
//


/// `interface nsIIdlePeriod : nsISupports`
///

/// ```text
/// /**
///  * An instance implementing nsIIdlePeriod is used by an associated
///  * nsIThread to estimate when it is likely that it will receive an
///  * event.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIdlePeriod {
    vtable: *const nsIIdlePeriodVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIdlePeriod.
unsafe impl XpCom for nsIIdlePeriod {
    const IID: nsIID = nsID(0x21dd35a2, 0xeae9, 0x4bd8,
        [0xb4, 0x70, 0x0d, 0xfa, 0x35, 0xa0, 0xe3, 0xb9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIdlePeriod {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIdlePeriod.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIdlePeriodCoerce {
    /// Cheaply cast a value of this type from a `nsIIdlePeriod`.
    fn coerce_from(v: &nsIIdlePeriod) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIdlePeriodCoerce for nsIIdlePeriod {
    #[inline]
    fn coerce_from(v: &nsIIdlePeriod) -> &Self {
        v
    }
}

impl nsIIdlePeriod {
    /// Cast this `nsIIdlePeriod` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIdlePeriodCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIdlePeriod {
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
impl<T: nsISupportsCoerce> nsIIdlePeriodCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdlePeriod) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIdlePeriod
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIdlePeriodVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* TimeStamp getIdlePeriodHint (); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetIdlePeriodHint: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIdlePeriod {

    /// ```text
    /// /**
    ///      * Return an estimate of a point in time in the future when we
    ///      * think that the associated thread will become busy. Should
    ///      * return TimeStamp() (i.e. the null time) or a time less than
    ///      * TimeStamp::Now() if the thread is currently busy or will become
    ///      * busy very soon.
    ///      */
    /// ```
    ///

    /// `TimeStamp getIdlePeriodHint ();`
    const _GetIdlePeriodHint: () = ();

}


