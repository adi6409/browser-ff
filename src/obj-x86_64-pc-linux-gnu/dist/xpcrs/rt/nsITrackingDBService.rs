//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/antitracking/nsITrackingDBService.idl
//


/// `interface nsITrackingDBService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITrackingDBService {
    vtable: *const nsITrackingDBServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITrackingDBService.
unsafe impl XpCom for nsITrackingDBService {
    const IID: nsIID = nsID(0x650934db, 0x1939, 0x4424,
        [0xbe, 0x26, 0x6f, 0xfb, 0x03, 0x75, 0x42, 0x4d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITrackingDBService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITrackingDBService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITrackingDBServiceCoerce {
    /// Cheaply cast a value of this type from a `nsITrackingDBService`.
    fn coerce_from(v: &nsITrackingDBService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITrackingDBServiceCoerce for nsITrackingDBService {
    #[inline]
    fn coerce_from(v: &nsITrackingDBService) -> &Self {
        v
    }
}

impl nsITrackingDBService {
    /// Cast this `nsITrackingDBService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITrackingDBServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITrackingDBService {
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
impl<T: nsISupportsCoerce> nsITrackingDBServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITrackingDBService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITrackingDBService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITrackingDBServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void recordContentBlockingLog (in ACString data); */
    pub RecordContentBlockingLog: unsafe extern "system" fn (this: *const nsITrackingDBService, data: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* Promise saveEvents (in AString data); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub SaveEvents: *const ::libc::c_void,

    /* Promise clearAll (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub ClearAll: *const ::libc::c_void,

    /* Promise clearSince (in int64_t since); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub ClearSince: *const ::libc::c_void,

    /* Promise getEventsByDateRange (in int64_t dateFrom, in int64_t dateTo); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetEventsByDateRange: *const ::libc::c_void,

    /* Promise sumAllEvents (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub SumAllEvents: *const ::libc::c_void,

    /* Promise getEarliestRecordedDate (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetEarliestRecordedDate: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITrackingDBService {

    pub const OTHER_COOKIES_BLOCKED_ID: i64 = 0;


    pub const TRACKERS_ID: i64 = 1;


    pub const TRACKING_COOKIES_ID: i64 = 2;


    pub const CRYPTOMINERS_ID: i64 = 3;


    pub const FINGERPRINTERS_ID: i64 = 4;


    pub const SOCIAL_ID: i64 = 5;

    /// ```text
    /// /**
    ///    * Record entries from a content blocking log in the tracking database.
    ///    * This function is typically called at the end of the document lifecycle,
    ///    * since calling it multiple times results in multiple new entries.
    ///    *
    ///    * @param data    a json string containing the content blocking log.
    ///    */
    /// ```
    ///

    /// `void recordContentBlockingLog (in ACString data);`
    #[inline]
    pub unsafe fn RecordContentBlockingLog(&self, data: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).RecordContentBlockingLog)(self, data)
    }


    /// ```text
    /// /**
    ///    * Save new events in the content blocking database
    ///    * @param data    a json string containing the content blocking log.
    ///    */
    /// ```
    ///

    /// `Promise saveEvents (in AString data);`
    const _SaveEvents: () = ();

    /// ```text
    /// /**
    ///    * Clear all content blocking database entries.
    ///    */
    /// ```
    ///

    /// `Promise clearAll ();`
    const _ClearAll: () = ();

    /// ```text
    /// /**
    ///    * Clear all content blocking database entries added since the specified time.
    ///    * @param since   a unix timestamp representing the number of milliseconds from
    ///    *                Jan 1, 1970 00:00:00 UTC.
    ///    */
    /// ```
    ///

    /// `Promise clearSince (in int64_t since);`
    const _ClearSince: () = ();

    /// ```text
    /// /**
    ///    * Fetch events from the content blocking database
    ///    * @param dateFrom   a unix timestamp.
    ///    * @param dateTo     a unix timestamp.
    ///    */
    /// ```
    ///

    /// `Promise getEventsByDateRange (in int64_t dateFrom, in int64_t dateTo);`
    const _GetEventsByDateRange: () = ();

    /// ```text
    /// /**
    ///    * Return a count of all tracking events.
    ///    */
    /// ```
    ///

    /// `Promise sumAllEvents ();`
    const _SumAllEvents: () = ();

    /// ```text
    /// /**
    ///    * Return the earliest recorded date.
    ///    */
    /// ```
    ///

    /// `Promise getEarliestRecordedDate ();`
    const _GetEarliestRecordedDate: () = ();

}


