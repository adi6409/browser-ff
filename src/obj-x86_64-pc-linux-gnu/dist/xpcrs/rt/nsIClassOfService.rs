//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIClassOfService.idl
//


/// `interface nsIClassOfService : nsISupports`
///

/// ```text
/// /**
///  * nsIClassOfService.idl
///  *
///  * Used to express class dependencies and characteristics - complimentary to
///  * nsISupportsPriority which is used to express weight
///  *
///  * Channels that implement this interface may make use of this
///  * information in different ways.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIClassOfService {
    vtable: *const nsIClassOfServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIClassOfService.
unsafe impl XpCom for nsIClassOfService {
    const IID: nsIID = nsID(0x1ccb58ec, 0x5e07, 0x4cf9,
        [0xa3, 0x0d, 0xac, 0x54, 0x90, 0xd2, 0x3b, 0x41]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIClassOfService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIClassOfService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIClassOfServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIClassOfService`.
    fn coerce_from(v: &nsIClassOfService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIClassOfServiceCoerce for nsIClassOfService {
    #[inline]
    fn coerce_from(v: &nsIClassOfService) -> &Self {
        v
    }
}

impl nsIClassOfService {
    /// Cast this `nsIClassOfService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIClassOfServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIClassOfService {
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
impl<T: nsISupportsCoerce> nsIClassOfServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClassOfService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIClassOfService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIClassOfServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute unsigned long classFlags; */
    pub GetClassFlags: unsafe extern "system" fn (this: *const nsIClassOfService, aClassFlags: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long classFlags; */
    pub SetClassFlags: unsafe extern "system" fn (this: *const nsIClassOfService, aClassFlags: u32) -> ::nserror::nsresult,

    /* void clearClassFlags (in unsigned long flags); */
    pub ClearClassFlags: unsafe extern "system" fn (this: *const nsIClassOfService, flags: u32) -> ::nserror::nsresult,

    /* void addClassFlags (in unsigned long flags); */
    pub AddClassFlags: unsafe extern "system" fn (this: *const nsIClassOfService, flags: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIClassOfService {

    pub const Leader: i64 = 1;


    pub const Follower: i64 = 2;


    pub const Speculative: i64 = 4;


    pub const Background: i64 = 8;


    pub const Unblocked: i64 = 16;


    pub const Throttleable: i64 = 32;


    pub const UrgentStart: i64 = 64;


    pub const DontThrottle: i64 = 128;


    pub const Tail: i64 = 256;


    pub const TailAllowed: i64 = 512;


    pub const TailForbidden: i64 = 1024;


    /// `attribute unsigned long classFlags;`
    #[inline]
    pub unsafe fn GetClassFlags(&self, aClassFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetClassFlags)(self, aClassFlags)
    }



    /// `attribute unsigned long classFlags;`
    #[inline]
    pub unsafe fn SetClassFlags(&self, aClassFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetClassFlags)(self, aClassFlags)
    }



    /// `void clearClassFlags (in unsigned long flags);`
    #[inline]
    pub unsafe fn ClearClassFlags(&self, flags: u32) -> ::nserror::nsresult {
        ((*self.vtable).ClearClassFlags)(self, flags)
    }



    /// `void addClassFlags (in unsigned long flags);`
    #[inline]
    pub unsafe fn AddClassFlags(&self, flags: u32) -> ::nserror::nsresult {
        ((*self.vtable).AddClassFlags)(self, flags)
    }


}


