//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsISupportsPrimitives.idl
//


/// `interface nsISupportsPrimitive : nsISupports`
///

/// ```text
/// /**
///  * Primitive base interface.
///  *
///  * These first three are pointer types and do data copying
///  * using the nsIMemory. Be careful!
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsPrimitive {
    vtable: *const nsISupportsPrimitiveVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsPrimitive.
unsafe impl XpCom for nsISupportsPrimitive {
    const IID: nsIID = nsID(0xd0d4b136, 0x1dd1, 0x11b2,
        [0x93, 0x71, 0xf0, 0x72, 0x7e, 0xf8, 0x27, 0xc0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsPrimitive {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsPrimitive.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsPrimitiveCoerce {
    /// Cheaply cast a value of this type from a `nsISupportsPrimitive`.
    fn coerce_from(v: &nsISupportsPrimitive) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsPrimitiveCoerce for nsISupportsPrimitive {
    #[inline]
    fn coerce_from(v: &nsISupportsPrimitive) -> &Self {
        v
    }
}

impl nsISupportsPrimitive {
    /// Cast this `nsISupportsPrimitive` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsPrimitiveCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsPrimitive {
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
impl<T: nsISupportsCoerce> nsISupportsPrimitiveCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPrimitive) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsPrimitive
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsPrimitiveVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short type; */
    pub GetType: unsafe extern "system" fn (this: *const nsISupportsPrimitive, aType: *mut u16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsPrimitive {

    pub const TYPE_ID: i64 = 1;


    pub const TYPE_CSTRING: i64 = 2;


    pub const TYPE_STRING: i64 = 3;


    pub const TYPE_PRBOOL: i64 = 4;


    pub const TYPE_PRUINT8: i64 = 5;


    pub const TYPE_PRUINT16: i64 = 6;


    pub const TYPE_PRUINT32: i64 = 7;


    pub const TYPE_PRUINT64: i64 = 8;


    pub const TYPE_PRTIME: i64 = 9;


    pub const TYPE_CHAR: i64 = 10;


    pub const TYPE_PRINT16: i64 = 11;


    pub const TYPE_PRINT32: i64 = 12;


    pub const TYPE_PRINT64: i64 = 13;


    pub const TYPE_FLOAT: i64 = 14;


    pub const TYPE_DOUBLE: i64 = 15;


    pub const TYPE_INTERFACE_POINTER: i64 = 17;


    /// `readonly attribute unsigned short type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


}


/// `interface nsISupportsID : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for nsID structures
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsID {
    vtable: *const nsISupportsIDVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsID.
unsafe impl XpCom for nsISupportsID {
    const IID: nsIID = nsID(0xd18290a0, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsID {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsID.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsIDCoerce {
    /// Cheaply cast a value of this type from a `nsISupportsID`.
    fn coerce_from(v: &nsISupportsID) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsIDCoerce for nsISupportsID {
    #[inline]
    fn coerce_from(v: &nsISupportsID) -> &Self {
        v
    }
}

impl nsISupportsID {
    /// Cast this `nsISupportsID` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsIDCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsID {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsIDCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsID) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsID
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsIDVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute nsIDPtr data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsID, aData: *mut *mut nsID) -> ::nserror::nsresult,

    /* attribute nsIDPtr data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsID, aData: *const nsID) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsID, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsID {


    /// `attribute nsIDPtr data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut *mut nsID) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute nsIDPtr data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: *const nsID) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsCString : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for ASCII strings
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsCString {
    vtable: *const nsISupportsCStringVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsCString.
unsafe impl XpCom for nsISupportsCString {
    const IID: nsIID = nsID(0xd65ff270, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsCString {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsCString.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsCStringCoerce {
    /// Cheaply cast a value of this type from a `nsISupportsCString`.
    fn coerce_from(v: &nsISupportsCString) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsCStringCoerce for nsISupportsCString {
    #[inline]
    fn coerce_from(v: &nsISupportsCString) -> &Self {
        v
    }
}

impl nsISupportsCString {
    /// Cast this `nsISupportsCString` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsCStringCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsCString {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsCStringCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsCString) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsCString
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsCStringVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute ACString data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsCString, aData: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsCString, aData: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsCString, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsCString {


    /// `attribute ACString data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute ACString data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsString : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for Unicode strings
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsString {
    vtable: *const nsISupportsStringVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsString.
unsafe impl XpCom for nsISupportsString {
    const IID: nsIID = nsID(0xd79dc970, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsString {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsString.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsStringCoerce {
    /// Cheaply cast a value of this type from a `nsISupportsString`.
    fn coerce_from(v: &nsISupportsString) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsStringCoerce for nsISupportsString {
    #[inline]
    fn coerce_from(v: &nsISupportsString) -> &Self {
        v
    }
}

impl nsISupportsString {
    /// Cast this `nsISupportsString` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsStringCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsString {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsStringCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsString) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsString
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsStringVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute AString data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsString, aData: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsString, aData: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* wstring toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsString, _retval: *mut *const i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsString {


    /// `attribute AString data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute AString data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `wstring toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const i16) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsPRBool : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * The rest are truly primitive and are passed by value
///  */
/// /**
///  * Scriptable storage for booleans
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsPRBool {
    vtable: *const nsISupportsPRBoolVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsPRBool.
unsafe impl XpCom for nsISupportsPRBool {
    const IID: nsIID = nsID(0xddc3b490, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsPRBool {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsPRBool.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsPRBoolCoerce {
    /// Cheaply cast a value of this type from a `nsISupportsPRBool`.
    fn coerce_from(v: &nsISupportsPRBool) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsPRBoolCoerce for nsISupportsPRBool {
    #[inline]
    fn coerce_from(v: &nsISupportsPRBool) -> &Self {
        v
    }
}

impl nsISupportsPRBool {
    /// Cast this `nsISupportsPRBool` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsPRBoolCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsPRBool {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRBoolCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRBool) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsPRBool
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsPRBoolVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute boolean data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsPRBool, aData: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsPRBool, aData: bool) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsPRBool, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsPRBool {


    /// `attribute boolean data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute boolean data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsPRUint8 : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for 8-bit integers
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsPRUint8 {
    vtable: *const nsISupportsPRUint8VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsPRUint8.
unsafe impl XpCom for nsISupportsPRUint8 {
    const IID: nsIID = nsID(0xdec2e4e0, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsPRUint8 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsPRUint8.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsPRUint8Coerce {
    /// Cheaply cast a value of this type from a `nsISupportsPRUint8`.
    fn coerce_from(v: &nsISupportsPRUint8) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsPRUint8Coerce for nsISupportsPRUint8 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint8) -> &Self {
        v
    }
}

impl nsISupportsPRUint8 {
    /// Cast this `nsISupportsPRUint8` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsPRUint8Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsPRUint8 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRUint8Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint8) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsPRUint8
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsPRUint8VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute uint8_t data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsPRUint8, aData: *mut uint8_t) -> ::nserror::nsresult,

    /* attribute uint8_t data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsPRUint8, aData: uint8_t) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsPRUint8, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsPRUint8 {


    /// `attribute uint8_t data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute uint8_t data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsPRUint16 : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for unsigned 16-bit integers
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsPRUint16 {
    vtable: *const nsISupportsPRUint16VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsPRUint16.
unsafe impl XpCom for nsISupportsPRUint16 {
    const IID: nsIID = nsID(0xdfacb090, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsPRUint16 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsPRUint16.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsPRUint16Coerce {
    /// Cheaply cast a value of this type from a `nsISupportsPRUint16`.
    fn coerce_from(v: &nsISupportsPRUint16) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsPRUint16Coerce for nsISupportsPRUint16 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint16) -> &Self {
        v
    }
}

impl nsISupportsPRUint16 {
    /// Cast this `nsISupportsPRUint16` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsPRUint16Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsPRUint16 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRUint16Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint16) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsPRUint16
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsPRUint16VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute uint16_t data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsPRUint16, aData: *mut uint16_t) -> ::nserror::nsresult,

    /* attribute uint16_t data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsPRUint16, aData: uint16_t) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsPRUint16, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsPRUint16 {


    /// `attribute uint16_t data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute uint16_t data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsPRUint32 : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for unsigned 32-bit integers
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsPRUint32 {
    vtable: *const nsISupportsPRUint32VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsPRUint32.
unsafe impl XpCom for nsISupportsPRUint32 {
    const IID: nsIID = nsID(0xe01dc470, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsPRUint32 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsPRUint32.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsPRUint32Coerce {
    /// Cheaply cast a value of this type from a `nsISupportsPRUint32`.
    fn coerce_from(v: &nsISupportsPRUint32) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsPRUint32Coerce for nsISupportsPRUint32 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint32) -> &Self {
        v
    }
}

impl nsISupportsPRUint32 {
    /// Cast this `nsISupportsPRUint32` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsPRUint32Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsPRUint32 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRUint32Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint32) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsPRUint32
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsPRUint32VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute uint32_t data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsPRUint32, aData: *mut uint32_t) -> ::nserror::nsresult,

    /* attribute uint32_t data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsPRUint32, aData: uint32_t) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsPRUint32, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsPRUint32 {


    /// `attribute uint32_t data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute uint32_t data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsPRUint64 : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for 64-bit integers
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsPRUint64 {
    vtable: *const nsISupportsPRUint64VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsPRUint64.
unsafe impl XpCom for nsISupportsPRUint64 {
    const IID: nsIID = nsID(0xe13567c0, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsPRUint64 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsPRUint64.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsPRUint64Coerce {
    /// Cheaply cast a value of this type from a `nsISupportsPRUint64`.
    fn coerce_from(v: &nsISupportsPRUint64) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsPRUint64Coerce for nsISupportsPRUint64 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint64) -> &Self {
        v
    }
}

impl nsISupportsPRUint64 {
    /// Cast this `nsISupportsPRUint64` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsPRUint64Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsPRUint64 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRUint64Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint64) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsPRUint64
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsPRUint64VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute uint64_t data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsPRUint64, aData: *mut uint64_t) -> ::nserror::nsresult,

    /* attribute uint64_t data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsPRUint64, aData: uint64_t) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsPRUint64, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsPRUint64 {


    /// `attribute uint64_t data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute uint64_t data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsPRTime : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for NSPR date/time values
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsPRTime {
    vtable: *const nsISupportsPRTimeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsPRTime.
unsafe impl XpCom for nsISupportsPRTime {
    const IID: nsIID = nsID(0xe2563630, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsPRTime {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsPRTime.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsPRTimeCoerce {
    /// Cheaply cast a value of this type from a `nsISupportsPRTime`.
    fn coerce_from(v: &nsISupportsPRTime) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsPRTimeCoerce for nsISupportsPRTime {
    #[inline]
    fn coerce_from(v: &nsISupportsPRTime) -> &Self {
        v
    }
}

impl nsISupportsPRTime {
    /// Cast this `nsISupportsPRTime` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsPRTimeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsPRTime {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRTimeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRTime) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsPRTime
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsPRTimeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute PRTime data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsPRTime, aData: *mut PRTime) -> ::nserror::nsresult,

    /* attribute PRTime data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsPRTime, aData: PRTime) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsPRTime, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsPRTime {


    /// `attribute PRTime data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute PRTime data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: PRTime) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsChar : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for single character values
///  * (often used to store an ASCII character)
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsChar {
    vtable: *const nsISupportsCharVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsChar.
unsafe impl XpCom for nsISupportsChar {
    const IID: nsIID = nsID(0xe2b05e40, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsChar {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsChar.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsCharCoerce {
    /// Cheaply cast a value of this type from a `nsISupportsChar`.
    fn coerce_from(v: &nsISupportsChar) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsCharCoerce for nsISupportsChar {
    #[inline]
    fn coerce_from(v: &nsISupportsChar) -> &Self {
        v
    }
}

impl nsISupportsChar {
    /// Cast this `nsISupportsChar` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsCharCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsChar {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsCharCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsChar) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsChar
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsCharVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute char data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsChar, aData: *mut libc::c_char) -> ::nserror::nsresult,

    /* attribute char data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsChar, aData: libc::c_char) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsChar, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsChar {


    /// `attribute char data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute char data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsPRInt16 : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for 16-bit integers
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsPRInt16 {
    vtable: *const nsISupportsPRInt16VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsPRInt16.
unsafe impl XpCom for nsISupportsPRInt16 {
    const IID: nsIID = nsID(0xe30d94b0, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsPRInt16 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsPRInt16.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsPRInt16Coerce {
    /// Cheaply cast a value of this type from a `nsISupportsPRInt16`.
    fn coerce_from(v: &nsISupportsPRInt16) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsPRInt16Coerce for nsISupportsPRInt16 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRInt16) -> &Self {
        v
    }
}

impl nsISupportsPRInt16 {
    /// Cast this `nsISupportsPRInt16` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsPRInt16Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsPRInt16 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRInt16Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRInt16) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsPRInt16
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsPRInt16VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute int16_t data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsPRInt16, aData: *mut int16_t) -> ::nserror::nsresult,

    /* attribute int16_t data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsPRInt16, aData: int16_t) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsPRInt16, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsPRInt16 {


    /// `attribute int16_t data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut int16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute int16_t data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: int16_t) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsPRInt32 : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for 32-bit integers
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsPRInt32 {
    vtable: *const nsISupportsPRInt32VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsPRInt32.
unsafe impl XpCom for nsISupportsPRInt32 {
    const IID: nsIID = nsID(0xe36c5250, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsPRInt32 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsPRInt32.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsPRInt32Coerce {
    /// Cheaply cast a value of this type from a `nsISupportsPRInt32`.
    fn coerce_from(v: &nsISupportsPRInt32) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsPRInt32Coerce for nsISupportsPRInt32 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRInt32) -> &Self {
        v
    }
}

impl nsISupportsPRInt32 {
    /// Cast this `nsISupportsPRInt32` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsPRInt32Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsPRInt32 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRInt32Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRInt32) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsPRInt32
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsPRInt32VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute int32_t data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsPRInt32, aData: *mut int32_t) -> ::nserror::nsresult,

    /* attribute int32_t data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsPRInt32, aData: int32_t) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsPRInt32, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsPRInt32 {


    /// `attribute int32_t data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute int32_t data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsPRInt64 : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for 64-bit integers
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsPRInt64 {
    vtable: *const nsISupportsPRInt64VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsPRInt64.
unsafe impl XpCom for nsISupportsPRInt64 {
    const IID: nsIID = nsID(0xe3cb0ff0, 0x4a1c, 0x11d3,
        [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsPRInt64 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsPRInt64.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsPRInt64Coerce {
    /// Cheaply cast a value of this type from a `nsISupportsPRInt64`.
    fn coerce_from(v: &nsISupportsPRInt64) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsPRInt64Coerce for nsISupportsPRInt64 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRInt64) -> &Self {
        v
    }
}

impl nsISupportsPRInt64 {
    /// Cast this `nsISupportsPRInt64` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsPRInt64Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsPRInt64 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRInt64Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRInt64) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsPRInt64
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsPRInt64VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute int64_t data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsPRInt64, aData: *mut int64_t) -> ::nserror::nsresult,

    /* attribute int64_t data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsPRInt64, aData: int64_t) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsPRInt64, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsPRInt64 {


    /// `attribute int64_t data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute int64_t data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: int64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsFloat : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for floating point numbers
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsFloat {
    vtable: *const nsISupportsFloatVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsFloat.
unsafe impl XpCom for nsISupportsFloat {
    const IID: nsIID = nsID(0xabeaa390, 0x4ac0, 0x11d3,
        [0xba, 0xea, 0x00, 0x80, 0x5f, 0x8a, 0x5d, 0xd7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsFloat {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsFloat.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsFloatCoerce {
    /// Cheaply cast a value of this type from a `nsISupportsFloat`.
    fn coerce_from(v: &nsISupportsFloat) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsFloatCoerce for nsISupportsFloat {
    #[inline]
    fn coerce_from(v: &nsISupportsFloat) -> &Self {
        v
    }
}

impl nsISupportsFloat {
    /// Cast this `nsISupportsFloat` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsFloatCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsFloat {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsFloatCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsFloat) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsFloat
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsFloatVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute float data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsFloat, aData: *mut libc::c_float) -> ::nserror::nsresult,

    /* attribute float data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsFloat, aData: libc::c_float) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsFloat, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsFloat {


    /// `attribute float data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute float data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsDouble : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for doubles
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsDouble {
    vtable: *const nsISupportsDoubleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsDouble.
unsafe impl XpCom for nsISupportsDouble {
    const IID: nsIID = nsID(0xb32523a0, 0x4ac0, 0x11d3,
        [0xba, 0xea, 0x00, 0x80, 0x5f, 0x8a, 0x5d, 0xd7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsDouble {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsDouble.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsDoubleCoerce {
    /// Cheaply cast a value of this type from a `nsISupportsDouble`.
    fn coerce_from(v: &nsISupportsDouble) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsDoubleCoerce for nsISupportsDouble {
    #[inline]
    fn coerce_from(v: &nsISupportsDouble) -> &Self {
        v
    }
}

impl nsISupportsDouble {
    /// Cast this `nsISupportsDouble` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsDoubleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsDouble {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsDoubleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsDouble) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsDouble
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsDoubleVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute double data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsDouble, aData: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsDouble, aData: libc::c_double) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsDouble, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsDouble {


    /// `attribute double data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute double data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsISupportsInterfacePointer : nsISupportsPrimitive`
///

/// ```text
/// /**
///  * Scriptable storage for other XPCOM objects
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsInterfacePointer {
    vtable: *const nsISupportsInterfacePointerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsInterfacePointer.
unsafe impl XpCom for nsISupportsInterfacePointer {
    const IID: nsIID = nsID(0x995ea724, 0x1dd1, 0x11b2,
        [0x92, 0x11, 0xc2, 0x1b, 0xdd, 0x3e, 0x7e, 0xd0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsInterfacePointer {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsInterfacePointer.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsInterfacePointerCoerce {
    /// Cheaply cast a value of this type from a `nsISupportsInterfacePointer`.
    fn coerce_from(v: &nsISupportsInterfacePointer) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsInterfacePointerCoerce for nsISupportsInterfacePointer {
    #[inline]
    fn coerce_from(v: &nsISupportsInterfacePointer) -> &Self {
        v
    }
}

impl nsISupportsInterfacePointer {
    /// Cast this `nsISupportsInterfacePointer` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsInterfacePointerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsInterfacePointer {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsPrimitiveCoerce> nsISupportsInterfacePointerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsInterfacePointer) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsInterfacePointer
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsInterfacePointerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute nsISupports data; */
    pub GetData: unsafe extern "system" fn (this: *const nsISupportsInterfacePointer, aData: *mut *const nsISupports) -> ::nserror::nsresult,

    /* attribute nsISupports data; */
    pub SetData: unsafe extern "system" fn (this: *const nsISupportsInterfacePointer, aData: *const nsISupports) -> ::nserror::nsresult,

    /* attribute nsIDPtr dataIID; */
    pub GetDataIID: unsafe extern "system" fn (this: *const nsISupportsInterfacePointer, aDataIID: *mut *mut nsID) -> ::nserror::nsresult,

    /* attribute nsIDPtr dataIID; */
    pub SetDataIID: unsafe extern "system" fn (this: *const nsISupportsInterfacePointer, aDataIID: *const nsID) -> ::nserror::nsresult,

    /* string toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsISupportsInterfacePointer, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsInterfacePointer {


    /// `attribute nsISupports data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }



    /// `attribute nsISupports data;`
    #[inline]
    pub unsafe fn SetData(&self, aData: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aData)
    }



    /// `attribute nsIDPtr dataIID;`
    #[inline]
    pub unsafe fn GetDataIID(&self, aDataIID: *mut *mut nsID) -> ::nserror::nsresult {
        ((*self.vtable).GetDataIID)(self, aDataIID)
    }



    /// `attribute nsIDPtr dataIID;`
    #[inline]
    pub unsafe fn SetDataIID(&self, aDataIID: *const nsID) -> ::nserror::nsresult {
        ((*self.vtable).SetDataIID)(self, aDataIID)
    }



    /// `string toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


