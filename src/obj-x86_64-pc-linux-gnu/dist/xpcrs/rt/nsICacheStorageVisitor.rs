//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheStorageVisitor.idl
//


/// `interface nsICacheStorageVisitor : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheStorageVisitor {
    vtable: *const nsICacheStorageVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheStorageVisitor.
unsafe impl XpCom for nsICacheStorageVisitor {
    const IID: nsIID = nsID(0x6cc7c253, 0x93b6, 0x482b,
        [0x8e, 0x9d, 0x1e, 0x04, 0xd8, 0xe9, 0xd6, 0x55]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheStorageVisitor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheStorageVisitor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheStorageVisitorCoerce {
    /// Cheaply cast a value of this type from a `nsICacheStorageVisitor`.
    fn coerce_from(v: &nsICacheStorageVisitor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheStorageVisitorCoerce for nsICacheStorageVisitor {
    #[inline]
    fn coerce_from(v: &nsICacheStorageVisitor) -> &Self {
        v
    }
}

impl nsICacheStorageVisitor {
    /// Cast this `nsICacheStorageVisitor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheStorageVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheStorageVisitor {
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
impl<T: nsISupportsCoerce> nsICacheStorageVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheStorageVisitor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheStorageVisitor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheStorageVisitorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onCacheStorageInfo (in uint32_t aEntryCount, in uint64_t aConsumption, in uint64_t aCapacity, in nsIFile aDiskDirectory); */
    pub OnCacheStorageInfo: unsafe extern "system" fn (this: *const nsICacheStorageVisitor, aEntryCount: uint32_t, aConsumption: uint64_t, aCapacity: uint64_t, aDiskDirectory: *const nsIFile) -> ::nserror::nsresult,

    /* void onCacheEntryInfo (in nsIURI aURI, in ACString aIdEnhance, in int64_t aDataSize, in long aFetchCount, in uint32_t aLastModifiedTime, in uint32_t aExpirationTime, in boolean aPinned, in nsILoadContextInfo aInfo); */
    pub OnCacheEntryInfo: unsafe extern "system" fn (this: *const nsICacheStorageVisitor, aURI: *const nsIURI, aIdEnhance: *const ::nsstring::nsACString, aDataSize: int64_t, aFetchCount: i32, aLastModifiedTime: uint32_t, aExpirationTime: uint32_t, aPinned: bool, aInfo: *const nsILoadContextInfo) -> ::nserror::nsresult,

    /* void onCacheEntryVisitCompleted (); */
    pub OnCacheEntryVisitCompleted: unsafe extern "system" fn (this: *const nsICacheStorageVisitor) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheStorageVisitor {

    /// ```text
    /// /**
    ///    */
    /// ```
    ///

    /// `void onCacheStorageInfo (in uint32_t aEntryCount, in uint64_t aConsumption, in uint64_t aCapacity, in nsIFile aDiskDirectory);`
    #[inline]
    pub unsafe fn OnCacheStorageInfo(&self, aEntryCount: uint32_t, aConsumption: uint64_t, aCapacity: uint64_t, aDiskDirectory: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).OnCacheStorageInfo)(self, aEntryCount, aConsumption, aCapacity, aDiskDirectory)
    }


    /// ```text
    /// /**
    ///    */
    /// ```
    ///

    /// `void onCacheEntryInfo (in nsIURI aURI, in ACString aIdEnhance, in int64_t aDataSize, in long aFetchCount, in uint32_t aLastModifiedTime, in uint32_t aExpirationTime, in boolean aPinned, in nsILoadContextInfo aInfo);`
    #[inline]
    pub unsafe fn OnCacheEntryInfo(&self, aURI: *const nsIURI, aIdEnhance: *const ::nsstring::nsACString, aDataSize: int64_t, aFetchCount: i32, aLastModifiedTime: uint32_t, aExpirationTime: uint32_t, aPinned: bool, aInfo: *const nsILoadContextInfo) -> ::nserror::nsresult {
        ((*self.vtable).OnCacheEntryInfo)(self, aURI, aIdEnhance, aDataSize, aFetchCount, aLastModifiedTime, aExpirationTime, aPinned, aInfo)
    }


    /// ```text
    /// /**
    ///    */
    /// ```
    ///

    /// `void onCacheEntryVisitCompleted ();`
    #[inline]
    pub unsafe fn OnCacheEntryVisitCompleted(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnCacheEntryVisitCompleted)(self, )
    }


}


