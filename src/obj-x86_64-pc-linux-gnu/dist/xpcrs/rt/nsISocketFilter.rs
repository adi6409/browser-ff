//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISocketFilter.idl
//


/// `interface nsISocketFilter : nsISupports`
///

/// ```text
/// /**
///  * Filters are created and run on the parent, and filter all packets, both
///  * ingoing and outgoing. The child must specify the name of a recognized filter
///  * in order to create a socket.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISocketFilter {
    vtable: *const nsISocketFilterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISocketFilter.
unsafe impl XpCom for nsISocketFilter {
    const IID: nsIID = nsID(0xafe2c40c, 0xb9b9, 0x4207,
        [0xb8, 0x98, 0xe5, 0xcd, 0xe1, 0x8c, 0x61, 0x39]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISocketFilter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISocketFilter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISocketFilterCoerce {
    /// Cheaply cast a value of this type from a `nsISocketFilter`.
    fn coerce_from(v: &nsISocketFilter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISocketFilterCoerce for nsISocketFilter {
    #[inline]
    fn coerce_from(v: &nsISocketFilter) -> &Self {
        v
    }
}

impl nsISocketFilter {
    /// Cast this `nsISocketFilter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISocketFilterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISocketFilter {
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
impl<T: nsISupportsCoerce> nsISocketFilterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISocketFilter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISocketFilter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISocketFilterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* bool filterPacket ([const] in NetAddrPtr remote_addr, [array, size_is (len), const] in uint8_t data, in unsigned long len, in long direction); */
    /// Unable to generate binding because `native type mozilla::net::NetAddr unsupported`
    pub FilterPacket: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISocketFilter {

    pub const SF_INCOMING: i64 = 0;


    pub const SF_OUTGOING: i64 = 1;


    /// `bool filterPacket ([const] in NetAddrPtr remote_addr, [array, size_is (len), const] in uint8_t data, in unsigned long len, in long direction);`
    const _FilterPacket: () = ();

}


/// `interface nsISocketFilterHandler : nsISupports`
///

/// ```text
/// /**
///  * Factory of a specified filter.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISocketFilterHandler {
    vtable: *const nsISocketFilterHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISocketFilterHandler.
unsafe impl XpCom for nsISocketFilterHandler {
    const IID: nsIID = nsID(0x81ee76c6, 0x4753, 0x4125,
        [0x9c, 0x8c, 0x29, 0x0e, 0xd9, 0xba, 0x62, 0xfb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISocketFilterHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISocketFilterHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISocketFilterHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsISocketFilterHandler`.
    fn coerce_from(v: &nsISocketFilterHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISocketFilterHandlerCoerce for nsISocketFilterHandler {
    #[inline]
    fn coerce_from(v: &nsISocketFilterHandler) -> &Self {
        v
    }
}

impl nsISocketFilterHandler {
    /// Cast this `nsISocketFilterHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISocketFilterHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISocketFilterHandler {
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
impl<T: nsISupportsCoerce> nsISocketFilterHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISocketFilterHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISocketFilterHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISocketFilterHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsISocketFilter newFilter (); */
    pub NewFilter: unsafe extern "system" fn (this: *const nsISocketFilterHandler, _retval: *mut *const nsISocketFilter) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISocketFilterHandler {


    /// `nsISocketFilter newFilter ();`
    #[inline]
    pub unsafe fn NewFilter(&self, _retval: *mut *const nsISocketFilter) -> ::nserror::nsresult {
        ((*self.vtable).NewFilter)(self, _retval)
    }


}


