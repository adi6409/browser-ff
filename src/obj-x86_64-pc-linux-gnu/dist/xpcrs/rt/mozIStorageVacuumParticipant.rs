//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageVacuumParticipant.idl
//


/// `interface mozIStorageVacuumParticipant : nsISupports`
///

/// ```text
/// /**
///  * This interface contains the information that the Storage service needs to
///  * vacuum a database.  This interface is created as a service through the
///  * category manager with the category "vacuum-participant".
///  * Please see https://developer.mozilla.org/en/mozIStorageVacuumParticipant for
///  * more information.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageVacuumParticipant {
    vtable: *const mozIStorageVacuumParticipantVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageVacuumParticipant.
unsafe impl XpCom for mozIStorageVacuumParticipant {
    const IID: nsIID = nsID(0x8f367508, 0x1d9a, 0x4d3f,
        [0xbe, 0x0c, 0xac, 0x11, 0xb6, 0xdd, 0x7d, 0xbf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageVacuumParticipant {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageVacuumParticipant.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageVacuumParticipantCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageVacuumParticipant`.
    fn coerce_from(v: &mozIStorageVacuumParticipant) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageVacuumParticipantCoerce for mozIStorageVacuumParticipant {
    #[inline]
    fn coerce_from(v: &mozIStorageVacuumParticipant) -> &Self {
        v
    }
}

impl mozIStorageVacuumParticipant {
    /// Cast this `mozIStorageVacuumParticipant` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageVacuumParticipantCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageVacuumParticipant {
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
impl<T: nsISupportsCoerce> mozIStorageVacuumParticipantCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageVacuumParticipant) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageVacuumParticipant
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageVacuumParticipantVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute long expectedDatabasePageSize; */
    pub GetExpectedDatabasePageSize: unsafe extern "system" fn (this: *const mozIStorageVacuumParticipant, aExpectedDatabasePageSize: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute mozIStorageConnection databaseConnection; */
    pub GetDatabaseConnection: unsafe extern "system" fn (this: *const mozIStorageVacuumParticipant, aDatabaseConnection: *mut*const mozIStorageConnection) -> ::nserror::nsresult,

    /* boolean onBeginVacuum (); */
    pub OnBeginVacuum: unsafe extern "system" fn (this: *const mozIStorageVacuumParticipant, _retval: *mut bool) -> ::nserror::nsresult,

    /* void onEndVacuum (in boolean aSucceeded); */
    pub OnEndVacuum: unsafe extern "system" fn (this: *const mozIStorageVacuumParticipant, aSucceeded: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageVacuumParticipant {

    /// ```text
    /// /**
    ///    * The expected page size in bytes for the database.  The vacuum manager will
    ///    * try to correct the page size during idle based on this value.
    ///    *
    ///    * @note If the database is using the WAL journal mode, the page size won't
    ///   *        be changed to the requested value.  See bug 634374.
    ///    * @note Valid page size values are powers of 2 between 512 and 65536.
    ///    *       The suggested value is mozIStorageConnection::defaultPageSize.
    ///    */
    /// ```
    ///

    /// `readonly attribute long expectedDatabasePageSize;`
    #[inline]
    pub unsafe fn GetExpectedDatabasePageSize(&self, aExpectedDatabasePageSize: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetExpectedDatabasePageSize)(self, aExpectedDatabasePageSize)
    }


    /// ```text
    /// /**
    ///    * Connection to the database file to be vacuumed.
    ///    */
    /// ```
    ///

    /// `readonly attribute mozIStorageConnection databaseConnection;`
    #[inline]
    pub unsafe fn GetDatabaseConnection(&self, aDatabaseConnection: *mut*const mozIStorageConnection) -> ::nserror::nsresult {
        ((*self.vtable).GetDatabaseConnection)(self, aDatabaseConnection)
    }


    /// ```text
    /// /**
    ///    * Notifies when a vacuum operation begins.  Listeners should avoid using the
    ///    * database till onEndVacuum is received.
    ///    *
    ///    * @return true to proceed with the vacuum, false if the participant wants to
    ///    *         opt-out for now, it will be retried later.  Useful when participant
    ///    *         is running some other heavy operation that can't be interrupted.
    ///    *
    ///    * @note When a vacuum operation starts or ends it will also dispatch a global
    ///    *       "heavy-io-task" notification through the observer service with the
    ///    *       data argument being either "vacuum-begin" or "vacuum-end".
    ///    */
    /// ```
    ///

    /// `boolean onBeginVacuum ();`
    #[inline]
    pub unsafe fn OnBeginVacuum(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).OnBeginVacuum)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Notifies when a vacuum operation ends.
    ///    *
    ///    * @param aSucceeded
    ///    *        reports if the vacuum succeeded or failed.
    ///    */
    /// ```
    ///

    /// `void onEndVacuum (in boolean aSucceeded);`
    #[inline]
    pub unsafe fn OnEndVacuum(&self, aSucceeded: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnEndVacuum)(self, aSucceeded)
    }


}


