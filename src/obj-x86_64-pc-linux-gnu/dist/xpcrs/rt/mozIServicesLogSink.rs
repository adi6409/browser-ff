//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/interfaces/mozIServicesLogSink.idl
//


/// `interface mozIServicesLogSink : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIServicesLogSink {
    vtable: *const mozIServicesLogSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIServicesLogSink.
unsafe impl XpCom for mozIServicesLogSink {
    const IID: nsIID = nsID(0xc92bfe0d, 0x50b7, 0x4a7f,
        [0x96, 0x86, 0xfe, 0x53, 0x35, 0xa6, 0x96, 0xb9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIServicesLogSink {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIServicesLogSink.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIServicesLogSinkCoerce {
    /// Cheaply cast a value of this type from a `mozIServicesLogSink`.
    fn coerce_from(v: &mozIServicesLogSink) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIServicesLogSinkCoerce for mozIServicesLogSink {
    #[inline]
    fn coerce_from(v: &mozIServicesLogSink) -> &Self {
        v
    }
}

impl mozIServicesLogSink {
    /// Cast this `mozIServicesLogSink` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIServicesLogSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIServicesLogSink {
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
impl<T: nsISupportsCoerce> mozIServicesLogSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIServicesLogSink) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIServicesLogSink
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIServicesLogSinkVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute short maxLevel; */
    pub GetMaxLevel: unsafe extern "system" fn (this: *const mozIServicesLogSink, aMaxLevel: *mut i16) -> ::nserror::nsresult,

    /* attribute short maxLevel; */
    pub SetMaxLevel: unsafe extern "system" fn (this: *const mozIServicesLogSink, aMaxLevel: i16) -> ::nserror::nsresult,

    /* void error (in AString message); */
    pub Error: unsafe extern "system" fn (this: *const mozIServicesLogSink, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void warn (in AString message); */
    pub Warn: unsafe extern "system" fn (this: *const mozIServicesLogSink, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void debug (in AString message); */
    pub Debug: unsafe extern "system" fn (this: *const mozIServicesLogSink, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void trace (in AString message); */
    pub Trace: unsafe extern "system" fn (this: *const mozIServicesLogSink, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void info (in AString message); */
    pub Info: unsafe extern "system" fn (this: *const mozIServicesLogSink, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIServicesLogSink {

    pub const LEVEL_OFF: i64 = 0;


    pub const LEVEL_ERROR: i64 = 1;


    pub const LEVEL_WARN: i64 = 2;


    pub const LEVEL_INFO: i64 = 3;


    pub const LEVEL_DEBUG: i64 = 4;


    pub const LEVEL_TRACE: i64 = 5;


    /// `attribute short maxLevel;`
    #[inline]
    pub unsafe fn GetMaxLevel(&self, aMaxLevel: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxLevel)(self, aMaxLevel)
    }



    /// `attribute short maxLevel;`
    #[inline]
    pub unsafe fn SetMaxLevel(&self, aMaxLevel: i16) -> ::nserror::nsresult {
        ((*self.vtable).SetMaxLevel)(self, aMaxLevel)
    }



    /// `void error (in AString message);`
    #[inline]
    pub unsafe fn Error(&self, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Error)(self, message)
    }



    /// `void warn (in AString message);`
    #[inline]
    pub unsafe fn Warn(&self, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Warn)(self, message)
    }



    /// `void debug (in AString message);`
    #[inline]
    pub unsafe fn Debug(&self, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Debug)(self, message)
    }



    /// `void trace (in AString message);`
    #[inline]
    pub unsafe fn Trace(&self, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Trace)(self, message)
    }



    /// `void info (in AString message);`
    #[inline]
    pub unsafe fn Info(&self, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Info)(self, message)
    }


}


