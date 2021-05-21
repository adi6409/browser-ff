//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/storage/nsIStorageActivityService.idl
//


/// `interface nsIStorageActivityService : nsISupports`
///

/// ```text
/// /**
///  * nsIStorageActivityService is a service that can be used to know which
///  * origins have been active in a time range. This information can be used to
///  * implement "Clear Recent History" or similar features.
///  *
///  * If you are implementing a new Storage component, you should use
///  * QuotaManager. But if you don't do it, remember to call
///  * StorageActivityService methods in order to inform this service about
///  * 'writing' operations executed by origins.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStorageActivityService {
    vtable: *const nsIStorageActivityServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStorageActivityService.
unsafe impl XpCom for nsIStorageActivityService {
    const IID: nsIID = nsID(0xfd1310ba, 0xd1be, 0x4327,
        [0x98, 0x8e, 0x92, 0xb3, 0x9f, 0xcf, 0xf6, 0xf4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStorageActivityService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStorageActivityService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStorageActivityServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIStorageActivityService`.
    fn coerce_from(v: &nsIStorageActivityService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStorageActivityServiceCoerce for nsIStorageActivityService {
    #[inline]
    fn coerce_from(v: &nsIStorageActivityService) -> &Self {
        v
    }
}

impl nsIStorageActivityService {
    /// Cast this `nsIStorageActivityService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStorageActivityServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStorageActivityService {
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
impl<T: nsISupportsCoerce> nsIStorageActivityServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStorageActivityService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStorageActivityService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStorageActivityServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIArray getActiveOrigins (in PRTime from, in PRTime to); */
    pub GetActiveOrigins: unsafe extern "system" fn (this: *const nsIStorageActivityService, from: PRTime, to: PRTime, _retval: *mut*const nsIArray) -> ::nserror::nsresult,

    /* void moveOriginInTime (in nsIPrincipal origin, in PRTime when); */
    pub MoveOriginInTime: unsafe extern "system" fn (this: *const nsIStorageActivityService, origin: *const nsIPrincipal, when: PRTime) -> ::nserror::nsresult,

    /* void testOnlyReset (); */
    pub TestOnlyReset: unsafe extern "system" fn (this: *const nsIStorageActivityService) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStorageActivityService {


    /// `nsIArray getActiveOrigins (in PRTime from, in PRTime to);`
    #[inline]
    pub unsafe fn GetActiveOrigins(&self, from: PRTime, to: PRTime, _retval: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetActiveOrigins)(self, from, to, _retval)
    }



    /// `void moveOriginInTime (in nsIPrincipal origin, in PRTime when);`
    #[inline]
    pub unsafe fn MoveOriginInTime(&self, origin: *const nsIPrincipal, when: PRTime) -> ::nserror::nsresult {
        ((*self.vtable).MoveOriginInTime)(self, origin, when)
    }



    /// `void testOnlyReset ();`
    #[inline]
    pub unsafe fn TestOnlyReset(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).TestOnlyReset)(self, )
    }


}


