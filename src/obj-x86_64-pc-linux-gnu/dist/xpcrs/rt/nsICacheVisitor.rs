//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheVisitor.idl
//


/// `interface nsICacheVisitor : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheVisitor {
    vtable: *const nsICacheVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheVisitor.
unsafe impl XpCom for nsICacheVisitor {
    const IID: nsIID = nsID(0xf8c08c4b, 0xd778, 0x49d1,
        [0xa5, 0x9b, 0x86, 0x6f, 0xdc, 0x50, 0x0d, 0x95]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheVisitor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheVisitor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheVisitorCoerce {
    /// Cheaply cast a value of this type from a `nsICacheVisitor`.
    fn coerce_from(v: &nsICacheVisitor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheVisitorCoerce for nsICacheVisitor {
    #[inline]
    fn coerce_from(v: &nsICacheVisitor) -> &Self {
        v
    }
}

impl nsICacheVisitor {
    /// Cast this `nsICacheVisitor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheVisitor {
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
impl<T: nsISupportsCoerce> nsICacheVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheVisitor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheVisitor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheVisitorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean visitDevice (in string deviceID, in nsICacheDeviceInfo deviceInfo); */
    pub VisitDevice: unsafe extern "system" fn (this: *const nsICacheVisitor, deviceID: *const libc::c_char, deviceInfo: *const nsICacheDeviceInfo, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean visitEntry (in string deviceID, in nsICacheEntryInfo entryInfo); */
    pub VisitEntry: unsafe extern "system" fn (this: *const nsICacheVisitor, deviceID: *const libc::c_char, entryInfo: *const nsICacheEntryInfo, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheVisitor {

    /// ```text
    /// /**
    ///      * Called to provide information about a cache device.
    ///      *
    ///      * @param deviceID - specifies the device being visited.
    ///      * @param deviceInfo - specifies information about this device.
    ///      *
    ///      * @return true to start visiting all entries for this device.
    ///      * @return false to advance to the next device.
    ///      */
    /// ```
    ///

    /// `boolean visitDevice (in string deviceID, in nsICacheDeviceInfo deviceInfo);`
    #[inline]
    pub unsafe fn VisitDevice(&self, deviceID: *const libc::c_char, deviceInfo: *const nsICacheDeviceInfo, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).VisitDevice)(self, deviceID, deviceInfo, _retval)
    }


    /// ```text
    /// /**
    ///      * Called to provide information about a cache entry.
    ///      *
    ///      * @param deviceID - specifies the device being visited.
    ///      * @param entryInfo - specifies information about this entry.
    ///      *
    ///      * @return true to visit the next entry on the current device, or if the
    ///      *   end of the device has been reached, advance to the next device.
    ///      * @return false to advance to the next device.
    ///      */
    /// ```
    ///

    /// `boolean visitEntry (in string deviceID, in nsICacheEntryInfo entryInfo);`
    #[inline]
    pub unsafe fn VisitEntry(&self, deviceID: *const libc::c_char, entryInfo: *const nsICacheEntryInfo, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).VisitEntry)(self, deviceID, entryInfo, _retval)
    }


}


/// `interface nsICacheDeviceInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheDeviceInfo {
    vtable: *const nsICacheDeviceInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheDeviceInfo.
unsafe impl XpCom for nsICacheDeviceInfo {
    const IID: nsIID = nsID(0x31d1c294, 0x1dd2, 0x11b2,
        [0xbe, 0x3a, 0xc7, 0x92, 0x30, 0xdc, 0xa2, 0x97]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheDeviceInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheDeviceInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheDeviceInfoCoerce {
    /// Cheaply cast a value of this type from a `nsICacheDeviceInfo`.
    fn coerce_from(v: &nsICacheDeviceInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheDeviceInfoCoerce for nsICacheDeviceInfo {
    #[inline]
    fn coerce_from(v: &nsICacheDeviceInfo) -> &Self {
        v
    }
}

impl nsICacheDeviceInfo {
    /// Cast this `nsICacheDeviceInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheDeviceInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheDeviceInfo {
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
impl<T: nsISupportsCoerce> nsICacheDeviceInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheDeviceInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheDeviceInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheDeviceInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString description; */
    pub GetDescription: unsafe extern "system" fn (this: *const nsICacheDeviceInfo, aDescription: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString usageReport; */
    pub GetUsageReport: unsafe extern "system" fn (this: *const nsICacheDeviceInfo, aUsageReport: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long entryCount; */
    pub GetEntryCount: unsafe extern "system" fn (this: *const nsICacheDeviceInfo, aEntryCount: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long totalSize; */
    pub GetTotalSize: unsafe extern "system" fn (this: *const nsICacheDeviceInfo, aTotalSize: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long maximumSize; */
    pub GetMaximumSize: unsafe extern "system" fn (this: *const nsICacheDeviceInfo, aMaximumSize: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheDeviceInfo {

    /// ```text
    /// /**
    ///      * Get a human readable description of the cache device.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString description;`
    #[inline]
    pub unsafe fn GetDescription(&self, aDescription: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDescription)(self, aDescription)
    }


    /// ```text
    /// /**
    ///      * Get a usage report, statistics, miscellaneous data about
    ///      * the cache device.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString usageReport;`
    #[inline]
    pub unsafe fn GetUsageReport(&self, aUsageReport: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetUsageReport)(self, aUsageReport)
    }


    /// ```text
    /// /**
    ///      * Get the number of stored cache entries.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long entryCount;`
    #[inline]
    pub unsafe fn GetEntryCount(&self, aEntryCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetEntryCount)(self, aEntryCount)
    }


    /// ```text
    /// /**
    ///      * Get the total size of the stored cache entries.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long totalSize;`
    #[inline]
    pub unsafe fn GetTotalSize(&self, aTotalSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTotalSize)(self, aTotalSize)
    }


    /// ```text
    /// /**
    ///      * Get the upper limit of the size of the data the cache can store.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long maximumSize;`
    #[inline]
    pub unsafe fn GetMaximumSize(&self, aMaximumSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMaximumSize)(self, aMaximumSize)
    }


}


/// `interface nsICacheEntryInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheEntryInfo {
    vtable: *const nsICacheEntryInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheEntryInfo.
unsafe impl XpCom for nsICacheEntryInfo {
    const IID: nsIID = nsID(0xfab51c92, 0x95c3, 0x4468,
        [0xb3, 0x17, 0x7d, 0xe4, 0xd7, 0x58, 0x82, 0x54]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheEntryInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheEntryInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheEntryInfoCoerce {
    /// Cheaply cast a value of this type from a `nsICacheEntryInfo`.
    fn coerce_from(v: &nsICacheEntryInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheEntryInfoCoerce for nsICacheEntryInfo {
    #[inline]
    fn coerce_from(v: &nsICacheEntryInfo) -> &Self {
        v
    }
}

impl nsICacheEntryInfo {
    /// Cast this `nsICacheEntryInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheEntryInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheEntryInfo {
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
impl<T: nsISupportsCoerce> nsICacheEntryInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheEntryInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheEntryInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheEntryInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString clientID; */
    pub GetClientID: unsafe extern "system" fn (this: *const nsICacheEntryInfo, aClientID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString deviceID; */
    pub GetDeviceID: unsafe extern "system" fn (this: *const nsICacheEntryInfo, aDeviceID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString key; */
    pub GetKey: unsafe extern "system" fn (this: *const nsICacheEntryInfo, aKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute long fetchCount; */
    pub GetFetchCount: unsafe extern "system" fn (this: *const nsICacheEntryInfo, aFetchCount: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute uint32_t lastFetched; */
    pub GetLastFetched: unsafe extern "system" fn (this: *const nsICacheEntryInfo, aLastFetched: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t lastModified; */
    pub GetLastModified: unsafe extern "system" fn (this: *const nsICacheEntryInfo, aLastModified: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t expirationTime; */
    pub GetExpirationTime: unsafe extern "system" fn (this: *const nsICacheEntryInfo, aExpirationTime: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute unsigned long dataSize; */
    pub GetDataSize: unsafe extern "system" fn (this: *const nsICacheEntryInfo, aDataSize: *mut u32) -> ::nserror::nsresult,

    /* boolean isStreamBased (); */
    pub IsStreamBased: unsafe extern "system" fn (this: *const nsICacheEntryInfo, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheEntryInfo {

    /// ```text
    /// /**
    ///      * Get the client id associated with this cache entry.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString clientID;`
    #[inline]
    pub unsafe fn GetClientID(&self, aClientID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetClientID)(self, aClientID)
    }


    /// ```text
    /// /**
    ///      * Get the id for the device that stores this cache entry.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString deviceID;`
    #[inline]
    pub unsafe fn GetDeviceID(&self, aDeviceID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDeviceID)(self, aDeviceID)
    }


    /// ```text
    /// /**
    ///      * Get the key identifying the cache entry.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString key;`
    #[inline]
    pub unsafe fn GetKey(&self, aKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetKey)(self, aKey)
    }


    /// ```text
    /// /**
    ///      * Get the number of times the cache entry has been opened.
    ///      */
    /// ```
    ///

    /// `readonly attribute long fetchCount;`
    #[inline]
    pub unsafe fn GetFetchCount(&self, aFetchCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetFetchCount)(self, aFetchCount)
    }


    /// ```text
    /// /**
    ///      * Get the last time the cache entry was opened (in seconds since the Epoch).
    ///      */
    /// ```
    ///

    /// `readonly attribute uint32_t lastFetched;`
    #[inline]
    pub unsafe fn GetLastFetched(&self, aLastFetched: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLastFetched)(self, aLastFetched)
    }


    /// ```text
    /// /**
    ///      * Get the last time the cache entry was modified (in seconds since the Epoch).
    ///      */
    /// ```
    ///

    /// `readonly attribute uint32_t lastModified;`
    #[inline]
    pub unsafe fn GetLastModified(&self, aLastModified: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLastModified)(self, aLastModified)
    }


    /// ```text
    /// /**
    ///      * Get the expiration time of the cache entry (in seconds since the Epoch).
    ///      */
    /// ```
    ///

    /// `readonly attribute uint32_t expirationTime;`
    #[inline]
    pub unsafe fn GetExpirationTime(&self, aExpirationTime: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetExpirationTime)(self, aExpirationTime)
    }


    /// ```text
    /// /**
    ///      * Get the cache entry data size.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long dataSize;`
    #[inline]
    pub unsafe fn GetDataSize(&self, aDataSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetDataSize)(self, aDataSize)
    }


    /// ```text
    /// /**
    ///      * Find out whether or not the cache entry is stream based.
    ///      */
    /// ```
    ///

    /// `boolean isStreamBased ();`
    #[inline]
    pub unsafe fn IsStreamBased(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsStreamBased)(self, _retval)
    }


}


