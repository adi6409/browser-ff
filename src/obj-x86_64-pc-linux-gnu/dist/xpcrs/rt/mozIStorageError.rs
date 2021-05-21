//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageError.idl
//


/// `interface mozIStorageError : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageError {
    vtable: *const mozIStorageErrorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageError.
unsafe impl XpCom for mozIStorageError {
    const IID: nsIID = nsID(0x1f350f96, 0x7023, 0x434a,
        [0x88, 0x64, 0x40, 0xa1, 0xc4, 0x93, 0xaa, 0xc1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageError {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageError.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageErrorCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageError`.
    fn coerce_from(v: &mozIStorageError) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageErrorCoerce for mozIStorageError {
    #[inline]
    fn coerce_from(v: &mozIStorageError) -> &Self {
        v
    }
}

impl mozIStorageError {
    /// Cast this `mozIStorageError` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageErrorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageError {
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
impl<T: nsISupportsCoerce> mozIStorageErrorCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageError) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageError
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageErrorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute long result; */
    pub GetResult: unsafe extern "system" fn (this: *const mozIStorageError, aResult: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String message; */
    pub GetMessage: unsafe extern "system" fn (this: *const mozIStorageError, aMessage: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageError {
    /// ```text
    /// /**
    ///    * General SQL error or missing database.
    ///    */
    /// ```
    ///

    pub const ERROR: i64 = 1;

    /// ```text
    /// /**
    ///    * Internal logic error.
    ///    */
    /// ```
    ///

    pub const INTERNAL: i64 = 2;

    /// ```text
    /// /**
    ///    * Access permission denied.
    ///    */
    /// ```
    ///

    pub const PERM: i64 = 3;

    /// ```text
    /// /**
    ///    * A callback routine requested an abort.
    ///    */
    /// ```
    ///

    pub const ABORT: i64 = 4;

    /// ```text
    /// /**
    ///    * The database file is locked.
    ///    */
    /// ```
    ///

    pub const BUSY: i64 = 5;

    /// ```text
    /// /**
    ///    * A table in the database is locked.
    ///    */
    /// ```
    ///

    pub const LOCKED: i64 = 6;

    /// ```text
    /// /**
    ///    * An allocation failed.
    ///    */
    /// ```
    ///

    pub const NOMEM: i64 = 7;

    /// ```text
    /// /**
    ///    * Attempt to write to a readonly database.
    ///    */
    /// ```
    ///

    pub const READONLY: i64 = 8;

    /// ```text
    /// /**
    ///    * Operation was terminated by an interrupt.
    ///    */
    /// ```
    ///

    pub const INTERRUPT: i64 = 9;

    /// ```text
    /// /**
    ///    * Some kind of disk I/O error occurred.
    ///    */
    /// ```
    ///

    pub const IOERR: i64 = 10;

    /// ```text
    /// /**
    ///    * The database disk image is malformed.
    ///    */
    /// ```
    ///

    pub const CORRUPT: i64 = 11;

    /// ```text
    /// /**
    ///    * An insertion failed because the database is full.
    ///    */
    /// ```
    ///

    pub const FULL: i64 = 13;

    /// ```text
    /// /**
    ///    * Unable to open the database file.
    ///    */
    /// ```
    ///

    pub const CANTOPEN: i64 = 14;

    /// ```text
    /// /**
    ///    * The database is empty.
    ///    */
    /// ```
    ///

    pub const EMPTY: i64 = 16;

    /// ```text
    /// /**
    ///    * The database scheme changed.
    ///    */
    /// ```
    ///

    pub const SCHEMA: i64 = 17;

    /// ```text
    /// /**
    ///    * A string or blob exceeds the size limit.
    ///    */
    /// ```
    ///

    pub const TOOBIG: i64 = 18;

    /// ```text
    /// /**
    ///    * Abort due to a constraint violation.
    ///    */
    /// ```
    ///

    pub const CONSTRAINT: i64 = 19;

    /// ```text
    /// /**
    ///    * Data type mismatch.
    ///    */
    /// ```
    ///

    pub const MISMATCH: i64 = 20;

    /// ```text
    /// /**
    ///    * Library used incorrectly.
    ///    */
    /// ```
    ///

    pub const MISUSE: i64 = 21;

    /// ```text
    /// /**
    ///    * Uses OS features not supported on the host system.
    ///    */
    /// ```
    ///

    pub const NOLFS: i64 = 22;

    /// ```text
    /// /**
    ///    * Authorization denied.
    ///    */
    /// ```
    ///

    pub const AUTH: i64 = 23;

    /// ```text
    /// /**
    ///    * Auxiliary database format error.
    ///    */
    /// ```
    ///

    pub const FORMAT: i64 = 24;

    /// ```text
    /// /**
    ///    * Attempt to bind a parameter using an out-of-range index or nonexistent
    ///    * named parameter name.
    ///    */
    /// ```
    ///

    pub const RANGE: i64 = 25;

    /// ```text
    /// /**
    ///    * File opened that is not a database file.
    ///    */
    /// ```
    ///

    pub const NOTADB: i64 = 26;

    /// ```text
    /// /**
    ///    * Indicates what type of error occurred.
    ///    */
    /// ```
    ///

    /// `readonly attribute long result;`
    #[inline]
    pub unsafe fn GetResult(&self, aResult: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetResult)(self, aResult)
    }


    /// ```text
    /// /**
    ///    * An error string the gives more details, if available.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String message;`
    #[inline]
    pub unsafe fn GetMessage(&self, aMessage: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetMessage)(self, aMessage)
    }


}


