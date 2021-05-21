//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIInputStreamPriority.idl
//


/// `interface nsIInputStreamPriority : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInputStreamPriority {
    vtable: *const nsIInputStreamPriorityVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInputStreamPriority.
unsafe impl XpCom for nsIInputStreamPriority {
    const IID: nsIID = nsID(0xdaa45b24, 0x98ee, 0x4eb2,
        [0x9c, 0xec, 0xaa, 0xd0, 0xbc, 0x02, 0x3e, 0x9d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInputStreamPriority {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInputStreamPriority.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInputStreamPriorityCoerce {
    /// Cheaply cast a value of this type from a `nsIInputStreamPriority`.
    fn coerce_from(v: &nsIInputStreamPriority) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInputStreamPriorityCoerce for nsIInputStreamPriority {
    #[inline]
    fn coerce_from(v: &nsIInputStreamPriority) -> &Self {
        v
    }
}

impl nsIInputStreamPriority {
    /// Cast this `nsIInputStreamPriority` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInputStreamPriorityCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInputStreamPriority {
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
impl<T: nsISupportsCoerce> nsIInputStreamPriorityCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStreamPriority) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInputStreamPriority
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInputStreamPriorityVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute unsigned long priority; */
    pub GetPriority: unsafe extern "system" fn (this: *const nsIInputStreamPriority, aPriority: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long priority; */
    pub SetPriority: unsafe extern "system" fn (this: *const nsIInputStreamPriority, aPriority: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInputStreamPriority {

    /// ```text
    /// /**
    ///      * An input stream implementing this interface will dispatch runnable
    ///      * events with this priority. See nsIRunnablePriority.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long priority;`
    #[inline]
    pub unsafe fn GetPriority(&self, aPriority: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPriority)(self, aPriority)
    }


    /// ```text
    /// /**
    ///      * An input stream implementing this interface will dispatch runnable
    ///      * events with this priority. See nsIRunnablePriority.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long priority;`
    #[inline]
    pub unsafe fn SetPriority(&self, aPriority: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetPriority)(self, aPriority)
    }


}


