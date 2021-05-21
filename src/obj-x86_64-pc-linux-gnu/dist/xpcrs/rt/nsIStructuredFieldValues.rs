//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/http-sfv/nsIStructuredFieldValues.idl
//


/// `interface nsISFVBareItem : nsISupports`
///

/// ```text
/// /**
///  * Conceptually, there are three types of structured field (header) values:
///  *
///  * Item - can be an Integer, Decimal, String, Token, Byte Sequence, or Boolean.
///  * It can have associated Parameters.
///  * List - array of zero or more members, each of which can be an Item or an InnerList,
///  * both of which can be Parameterized.
///  * Dictionary - ordered map of name-value pairs, where the names are short textual strings
///  * and the values are Items or arrays of Items (represented with InnerList),
///  * both of which can be Parameterized. There can be zero or more members,
///  * and their names are unique in the scope of the Dictionary they occur within.
///  *
///  *
///  * There's also a few primitive types used to construct structured field values:
///  * - BareItem used as Item's value or as a parameter value in Parameters.
///  * - Parameters are an ordered map of key-value pairs that are associated with an Item or InnerList.
///  * The keys are unique within the scope the Parameters they occur within, and the values are BareItem.
///  * - InnerList is an array of zero or more Items. Can have Parameters.
///  * - ListEntry represents either Item or InnerList as a member of List or as member-value in Dictionary.
///  */
/// /**
///  * nsISFVBareItem is a building block for Item header value (nsISFVItem) and Parameters (nsISFVParams).
///  * It can be of type BOOL, STRING, DECIMAL, INTEGER, TOKEN, BYTE_SEQUENCE.
///  * Each type is represented by its own interface which is used to create
///  * a bare item of that type.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVBareItem {
    vtable: *const nsISFVBareItemVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVBareItem.
unsafe impl XpCom for nsISFVBareItem {
    const IID: nsIID = nsID(0x7072853f, 0x215b, 0x4a8a,
        [0x92, 0xe5, 0x97, 0x32, 0xbc, 0xcc, 0x37, 0x7b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVBareItem {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVBareItem.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVBareItemCoerce {
    /// Cheaply cast a value of this type from a `nsISFVBareItem`.
    fn coerce_from(v: &nsISFVBareItem) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVBareItemCoerce for nsISFVBareItem {
    #[inline]
    fn coerce_from(v: &nsISFVBareItem) -> &Self {
        v
    }
}

impl nsISFVBareItem {
    /// Cast this `nsISFVBareItem` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVBareItemCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVBareItem {
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
impl<T: nsISupportsCoerce> nsISFVBareItemCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVBareItem) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVBareItem
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVBareItemVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute long long type; */
    pub GetType: unsafe extern "system" fn (this: *const nsISFVBareItem, aType: *mut i64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVBareItem {

    pub const BOOL: i64 = 1;


    pub const STRING: i64 = 2;


    pub const DECIMAL: i64 = 3;


    pub const INTEGER: i64 = 4;


    pub const TOKEN: i64 = 5;


    pub const BYTE_SEQUENCE: i64 = 6;

    /// ```text
    /// /**
    ///      * Returns value associated with type of bare item.
    ///      * Used to identify type of bare item without querying for interface
    ///      * (like nsISFVString, etc).
    ///      */
    /// ```
    ///

    /// `readonly attribute long long type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


}


/// `interface nsISFVInteger : nsISFVBareItem`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVInteger {
    vtable: *const nsISFVIntegerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVInteger.
unsafe impl XpCom for nsISFVInteger {
    const IID: nsIID = nsID(0x843eea44, 0x990a, 0x422c,
        [0xbb, 0xf2, 0x2a, 0xa4, 0xba, 0x9e, 0xe4, 0xd2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVInteger {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVInteger.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVIntegerCoerce {
    /// Cheaply cast a value of this type from a `nsISFVInteger`.
    fn coerce_from(v: &nsISFVInteger) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVIntegerCoerce for nsISFVInteger {
    #[inline]
    fn coerce_from(v: &nsISFVInteger) -> &Self {
        v
    }
}

impl nsISFVInteger {
    /// Cast this `nsISFVInteger` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVIntegerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVInteger {
    type Target = nsISFVBareItem;
    #[inline]
    fn deref(&self) -> &nsISFVBareItem {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISFVBareItemCoerce> nsISFVIntegerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVInteger) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVInteger
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVIntegerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISFVBareItemVTable,

    /* attribute long long value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsISFVInteger, aValue: *mut i64) -> ::nserror::nsresult,

    /* attribute long long value; */
    pub SetValue: unsafe extern "system" fn (this: *const nsISFVInteger, aValue: i64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVInteger {


    /// `attribute long long value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }



    /// `attribute long long value;`
    #[inline]
    pub unsafe fn SetValue(&self, aValue: i64) -> ::nserror::nsresult {
        ((*self.vtable).SetValue)(self, aValue)
    }


}


/// `interface nsISFVString : nsISFVBareItem`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVString {
    vtable: *const nsISFVStringVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVString.
unsafe impl XpCom for nsISFVString {
    const IID: nsIID = nsID(0xdf6a0787, 0x7caa, 0x4fef,
        [0xb1, 0x45, 0x08, 0xc1, 0x10, 0x4c, 0x2f, 0xde]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVString {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVString.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVStringCoerce {
    /// Cheaply cast a value of this type from a `nsISFVString`.
    fn coerce_from(v: &nsISFVString) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVStringCoerce for nsISFVString {
    #[inline]
    fn coerce_from(v: &nsISFVString) -> &Self {
        v
    }
}

impl nsISFVString {
    /// Cast this `nsISFVString` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVStringCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVString {
    type Target = nsISFVBareItem;
    #[inline]
    fn deref(&self) -> &nsISFVBareItem {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISFVBareItemCoerce> nsISFVStringCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVString) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVString
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVStringVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISFVBareItemVTable,

    /* attribute ACString value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsISFVString, aValue: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString value; */
    pub SetValue: unsafe extern "system" fn (this: *const nsISFVString, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVString {


    /// `attribute ACString value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }



    /// `attribute ACString value;`
    #[inline]
    pub unsafe fn SetValue(&self, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetValue)(self, aValue)
    }


}


/// `interface nsISFVBool : nsISFVBareItem`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVBool {
    vtable: *const nsISFVBoolVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVBool.
unsafe impl XpCom for nsISFVBool {
    const IID: nsIID = nsID(0xd263c6d7, 0x4123, 0x4c39,
        [0xa1, 0x21, 0xcc, 0xf8, 0x74, 0xa1, 0x90, 0x12]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVBool {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVBool.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVBoolCoerce {
    /// Cheaply cast a value of this type from a `nsISFVBool`.
    fn coerce_from(v: &nsISFVBool) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVBoolCoerce for nsISFVBool {
    #[inline]
    fn coerce_from(v: &nsISFVBool) -> &Self {
        v
    }
}

impl nsISFVBool {
    /// Cast this `nsISFVBool` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVBoolCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVBool {
    type Target = nsISFVBareItem;
    #[inline]
    fn deref(&self) -> &nsISFVBareItem {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISFVBareItemCoerce> nsISFVBoolCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVBool) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVBool
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVBoolVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISFVBareItemVTable,

    /* attribute boolean value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsISFVBool, aValue: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean value; */
    pub SetValue: unsafe extern "system" fn (this: *const nsISFVBool, aValue: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVBool {


    /// `attribute boolean value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }



    /// `attribute boolean value;`
    #[inline]
    pub unsafe fn SetValue(&self, aValue: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetValue)(self, aValue)
    }


}


/// `interface nsISFVDecimal : nsISFVBareItem`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVDecimal {
    vtable: *const nsISFVDecimalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVDecimal.
unsafe impl XpCom for nsISFVDecimal {
    const IID: nsIID = nsID(0x1098da8b, 0xb4df, 0x4526,
        [0xb9, 0x85, 0x53, 0xdb, 0xd4, 0x16, 0x0a, 0xd2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVDecimal {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVDecimal.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVDecimalCoerce {
    /// Cheaply cast a value of this type from a `nsISFVDecimal`.
    fn coerce_from(v: &nsISFVDecimal) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVDecimalCoerce for nsISFVDecimal {
    #[inline]
    fn coerce_from(v: &nsISFVDecimal) -> &Self {
        v
    }
}

impl nsISFVDecimal {
    /// Cast this `nsISFVDecimal` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVDecimalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVDecimal {
    type Target = nsISFVBareItem;
    #[inline]
    fn deref(&self) -> &nsISFVBareItem {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISFVBareItemCoerce> nsISFVDecimalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVDecimal) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVDecimal
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVDecimalVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISFVBareItemVTable,

    /* attribute double value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsISFVDecimal, aValue: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double value; */
    pub SetValue: unsafe extern "system" fn (this: *const nsISFVDecimal, aValue: libc::c_double) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVDecimal {


    /// `attribute double value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }



    /// `attribute double value;`
    #[inline]
    pub unsafe fn SetValue(&self, aValue: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetValue)(self, aValue)
    }


}


/// `interface nsISFVToken : nsISFVBareItem`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVToken {
    vtable: *const nsISFVTokenVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVToken.
unsafe impl XpCom for nsISFVToken {
    const IID: nsIID = nsID(0x8ad33d52, 0xb9b2, 0x4a17,
        [0x8a, 0xa8, 0x99, 0x12, 0x50, 0xfc, 0x12, 0x14]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVToken {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVToken.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVTokenCoerce {
    /// Cheaply cast a value of this type from a `nsISFVToken`.
    fn coerce_from(v: &nsISFVToken) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVTokenCoerce for nsISFVToken {
    #[inline]
    fn coerce_from(v: &nsISFVToken) -> &Self {
        v
    }
}

impl nsISFVToken {
    /// Cast this `nsISFVToken` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVTokenCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVToken {
    type Target = nsISFVBareItem;
    #[inline]
    fn deref(&self) -> &nsISFVBareItem {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISFVBareItemCoerce> nsISFVTokenCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVToken) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVToken
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVTokenVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISFVBareItemVTable,

    /* attribute ACString value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsISFVToken, aValue: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString value; */
    pub SetValue: unsafe extern "system" fn (this: *const nsISFVToken, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVToken {


    /// `attribute ACString value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }



    /// `attribute ACString value;`
    #[inline]
    pub unsafe fn SetValue(&self, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetValue)(self, aValue)
    }


}


/// `interface nsISFVByteSeq : nsISFVBareItem`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVByteSeq {
    vtable: *const nsISFVByteSeqVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVByteSeq.
unsafe impl XpCom for nsISFVByteSeq {
    const IID: nsIID = nsID(0x887eaef0, 0x19fe, 0x42bc,
        [0x9a, 0x42, 0x9f, 0xf7, 0x73, 0xaa, 0x8f, 0xea]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVByteSeq {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVByteSeq.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVByteSeqCoerce {
    /// Cheaply cast a value of this type from a `nsISFVByteSeq`.
    fn coerce_from(v: &nsISFVByteSeq) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVByteSeqCoerce for nsISFVByteSeq {
    #[inline]
    fn coerce_from(v: &nsISFVByteSeq) -> &Self {
        v
    }
}

impl nsISFVByteSeq {
    /// Cast this `nsISFVByteSeq` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVByteSeqCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVByteSeq {
    type Target = nsISFVBareItem;
    #[inline]
    fn deref(&self) -> &nsISFVBareItem {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISFVBareItemCoerce> nsISFVByteSeqCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVByteSeq) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVByteSeq
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVByteSeqVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISFVBareItemVTable,

    /* attribute ACString value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsISFVByteSeq, aValue: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString value; */
    pub SetValue: unsafe extern "system" fn (this: *const nsISFVByteSeq, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVByteSeq {


    /// `attribute ACString value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }



    /// `attribute ACString value;`
    #[inline]
    pub unsafe fn SetValue(&self, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetValue)(self, aValue)
    }


}


/// `interface nsISFVParams : nsISupports`
///

/// ```text
/// /**
///  * nsISFVParams represents parameters, key-value pairs of ACString and nsISFVBareItem,
///  * which parametrize Item type header or InnerList type withing List type header.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVParams {
    vtable: *const nsISFVParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVParams.
unsafe impl XpCom for nsISFVParams {
    const IID: nsIID = nsID(0xb1a397d7, 0x3333, 0x43e7,
        [0x99, 0x3a, 0xfb, 0xe8, 0xab, 0x90, 0xee, 0x96]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVParams {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVParams.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVParamsCoerce {
    /// Cheaply cast a value of this type from a `nsISFVParams`.
    fn coerce_from(v: &nsISFVParams) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVParamsCoerce for nsISFVParams {
    #[inline]
    fn coerce_from(v: &nsISFVParams) -> &Self {
        v
    }
}

impl nsISFVParams {
    /// Cast this `nsISFVParams` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVParams {
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
impl<T: nsISupportsCoerce> nsISFVParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVParams) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVParams
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVParamsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsISFVBareItem get (in ACString key); */
    pub Get: unsafe extern "system" fn (this: *const nsISFVParams, key: *const ::nsstring::nsACString, _retval: *mut *const nsISFVBareItem) -> ::nserror::nsresult,

    /* void set (in ACString key, in nsISFVBareItem item); */
    pub Set: unsafe extern "system" fn (this: *const nsISFVParams, key: *const ::nsstring::nsACString, item: *const nsISFVBareItem) -> ::nserror::nsresult,

    /* void delete (in ACString key); */
    pub Delete: unsafe extern "system" fn (this: *const nsISFVParams, key: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* Array<ACString> keys (); */
    pub Keys: unsafe extern "system" fn (this: *const nsISFVParams, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVParams {

    /// ```text
    /// /**
    ///      * Return value (nsISFVBareItem) stored for key, if it is present
    ///      *
    ///      * @throws NS_ERROR_UNEXPECTED if the key does not exist in parameters.
    ///      */
    /// ```
    ///

    /// `nsISFVBareItem get (in ACString key);`
    #[inline]
    pub unsafe fn Get(&self, key: *const ::nsstring::nsACString, _retval: *mut *const nsISFVBareItem) -> ::nserror::nsresult {
        ((*self.vtable).Get)(self, key, _retval)
    }


    /// ```text
    /// /**
    ///      * Insert a new key-value pair.
    ///      * If an equivalent key already exists: the key remains and retains in its place in the order,
    ///      * its corresponding value is updated with the new value.
    ///      *
    ///      * @throws NS_ERROR_UNEXPECTED if supplied item does not implement nsISFVBareItem interface.
    ///      */
    /// ```
    ///

    /// `void set (in ACString key, in nsISFVBareItem item);`
    #[inline]
    pub unsafe fn Set(&self, key: *const ::nsstring::nsACString, item: *const nsISFVBareItem) -> ::nserror::nsresult {
        ((*self.vtable).Set)(self, key, item)
    }


    /// ```text
    /// /**
    ///      * Remove the key-value pair equivalent to key.
    ///      *
    ///      * @throws NS_ERROR_UNEXPECTED upon attempt to delete  key that does not exist in parameters.
    ///      */
    /// ```
    ///

    /// `void delete (in ACString key);`
    #[inline]
    pub unsafe fn Delete(&self, key: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Delete)(self, key)
    }


    /// ```text
    /// /**
    ///      * Returns array of keys.
    ///      */
    /// ```
    ///

    /// `Array<ACString> keys ();`
    #[inline]
    pub unsafe fn Keys(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).Keys)(self, _retval)
    }


}


/// `interface nsISFVParametrizable : nsISupports`
///

/// ```text
/// /**
///  * nsISFVParametrizable is implemented for types that
///  * can be parametrized with nsISFVParams
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVParametrizable {
    vtable: *const nsISFVParametrizableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVParametrizable.
unsafe impl XpCom for nsISFVParametrizable {
    const IID: nsIID = nsID(0x6c0399f8, 0x01de, 0x4b25,
        [0xb3, 0x39, 0x68, 0xe3, 0x5e, 0x8d, 0x2e, 0x49]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVParametrizable {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVParametrizable.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVParametrizableCoerce {
    /// Cheaply cast a value of this type from a `nsISFVParametrizable`.
    fn coerce_from(v: &nsISFVParametrizable) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVParametrizableCoerce for nsISFVParametrizable {
    #[inline]
    fn coerce_from(v: &nsISFVParametrizable) -> &Self {
        v
    }
}

impl nsISFVParametrizable {
    /// Cast this `nsISFVParametrizable` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVParametrizableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVParametrizable {
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
impl<T: nsISupportsCoerce> nsISFVParametrizableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVParametrizable) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVParametrizable
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVParametrizableVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsISFVParams params; */
    pub GetParams: unsafe extern "system" fn (this: *const nsISFVParametrizable, aParams: *mut *const nsISFVParams) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVParametrizable {


    /// `readonly attribute nsISFVParams params;`
    #[inline]
    pub unsafe fn GetParams(&self, aParams: *mut *const nsISFVParams) -> ::nserror::nsresult {
        ((*self.vtable).GetParams)(self, aParams)
    }


}


/// `interface nsISFVItemOrInnerList : nsISFVParametrizable`
///

/// ```text
/// /**
///  * nsISFVItemOrInnerList represents member in nsISFVList
///  * or member-value in nsISFVDictionary.
///  * nsISFVItemOrInnerList is implemented for
///  * nsISFVItem or nsISFVInnerList, both of which are used
///  * to create nsISFVList and nsISFVDictionary.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVItemOrInnerList {
    vtable: *const nsISFVItemOrInnerListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVItemOrInnerList.
unsafe impl XpCom for nsISFVItemOrInnerList {
    const IID: nsIID = nsID(0x99ac1b56, 0xb5b3, 0x44e7,
        [0xad, 0x96, 0xdb, 0x74, 0x44, 0xaa, 0xe4, 0xb2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVItemOrInnerList {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVItemOrInnerList.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVItemOrInnerListCoerce {
    /// Cheaply cast a value of this type from a `nsISFVItemOrInnerList`.
    fn coerce_from(v: &nsISFVItemOrInnerList) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVItemOrInnerListCoerce for nsISFVItemOrInnerList {
    #[inline]
    fn coerce_from(v: &nsISFVItemOrInnerList) -> &Self {
        v
    }
}

impl nsISFVItemOrInnerList {
    /// Cast this `nsISFVItemOrInnerList` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVItemOrInnerListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVItemOrInnerList {
    type Target = nsISFVParametrizable;
    #[inline]
    fn deref(&self) -> &nsISFVParametrizable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISFVParametrizableCoerce> nsISFVItemOrInnerListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVItemOrInnerList) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVItemOrInnerList
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVItemOrInnerListVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISFVParametrizableVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVItemOrInnerList {


}


/// `interface nsISFVSerialize : nsISupports`
///

/// ```text
/// /**
///  * nsISFVSerialize indicates that object can be serialized into ACString.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVSerialize {
    vtable: *const nsISFVSerializeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVSerialize.
unsafe impl XpCom for nsISFVSerialize {
    const IID: nsIID = nsID(0x28b9215d, 0xc131, 0x413c,
        [0x94, 0x82, 0x00, 0x04, 0xa3, 0x71, 0xa5, 0xec]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVSerialize {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVSerialize.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVSerializeCoerce {
    /// Cheaply cast a value of this type from a `nsISFVSerialize`.
    fn coerce_from(v: &nsISFVSerialize) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVSerializeCoerce for nsISFVSerialize {
    #[inline]
    fn coerce_from(v: &nsISFVSerialize) -> &Self {
        v
    }
}

impl nsISFVSerialize {
    /// Cast this `nsISFVSerialize` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVSerializeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVSerialize {
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
impl<T: nsISupportsCoerce> nsISFVSerializeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVSerialize) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVSerialize
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVSerializeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString serialize (); */
    pub Serialize: unsafe extern "system" fn (this: *const nsISFVSerialize, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVSerialize {


    /// `ACString serialize ();`
    #[inline]
    pub unsafe fn Serialize(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Serialize)(self, _retval)
    }


}


/// `interface nsISFVItem : nsISFVItemOrInnerList`
///

/// ```text
/// /**
///  * nsISFVItem represents Item structured header value.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVItem {
    vtable: *const nsISFVItemVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVItem.
unsafe impl XpCom for nsISFVItem {
    const IID: nsIID = nsID(0xabe8826b, 0x6af7, 0x4e54,
        [0xbd, 0x2c, 0x46, 0xab, 0x23, 0x17, 0x00, 0xce]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVItem {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVItem.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVItemCoerce {
    /// Cheaply cast a value of this type from a `nsISFVItem`.
    fn coerce_from(v: &nsISFVItem) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVItemCoerce for nsISFVItem {
    #[inline]
    fn coerce_from(v: &nsISFVItem) -> &Self {
        v
    }
}

impl nsISFVItem {
    /// Cast this `nsISFVItem` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVItemCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVItem {
    type Target = nsISFVItemOrInnerList;
    #[inline]
    fn deref(&self) -> &nsISFVItemOrInnerList {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISFVItemOrInnerListCoerce> nsISFVItemCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVItem) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVItem
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVItemVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISFVItemOrInnerListVTable,

    /* readonly attribute nsISFVBareItem value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsISFVItem, aValue: *mut *const nsISFVBareItem) -> ::nserror::nsresult,

    /* ACString serialize (); */
    pub Serialize: unsafe extern "system" fn (this: *const nsISFVItem, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVItem {


    /// `readonly attribute nsISFVBareItem value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut *const nsISFVBareItem) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }



    /// `ACString serialize ();`
    #[inline]
    pub unsafe fn Serialize(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Serialize)(self, _retval)
    }


}


/// `interface nsISFVInnerList : nsISFVItemOrInnerList`
///

/// ```text
/// /**
///  * nsISFVInnerList can be used as a member of nsISFVList
///  * or a member-value of nsISFVDictionary.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVInnerList {
    vtable: *const nsISFVInnerListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVInnerList.
unsafe impl XpCom for nsISFVInnerList {
    const IID: nsIID = nsID(0xb2e52be2, 0x8488, 0x41b2,
        [0x9e, 0xe2, 0x3c, 0x48, 0xd9, 0x2d, 0x09, 0x5c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVInnerList {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVInnerList.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVInnerListCoerce {
    /// Cheaply cast a value of this type from a `nsISFVInnerList`.
    fn coerce_from(v: &nsISFVInnerList) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVInnerListCoerce for nsISFVInnerList {
    #[inline]
    fn coerce_from(v: &nsISFVInnerList) -> &Self {
        v
    }
}

impl nsISFVInnerList {
    /// Cast this `nsISFVInnerList` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVInnerListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVInnerList {
    type Target = nsISFVItemOrInnerList;
    #[inline]
    fn deref(&self) -> &nsISFVItemOrInnerList {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISFVItemOrInnerListCoerce> nsISFVInnerListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVInnerList) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVInnerList
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVInnerListVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISFVItemOrInnerListVTable,

    /* attribute Array<nsISFVItem> items; */
    pub GetItems: unsafe extern "system" fn (this: *const nsISFVInnerList, aItems: *mut thin_vec::ThinVec<RefPtr<nsISFVItem>>) -> ::nserror::nsresult,

    /* attribute Array<nsISFVItem> items; */
    pub SetItems: unsafe extern "system" fn (this: *const nsISFVInnerList, aItems: *const thin_vec::ThinVec<RefPtr<nsISFVItem>>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVInnerList {


    /// `attribute Array<nsISFVItem> items;`
    #[inline]
    pub unsafe fn GetItems(&self, aItems: *mut thin_vec::ThinVec<RefPtr<nsISFVItem>>) -> ::nserror::nsresult {
        ((*self.vtable).GetItems)(self, aItems)
    }



    /// `attribute Array<nsISFVItem> items;`
    #[inline]
    pub unsafe fn SetItems(&self, aItems: *const thin_vec::ThinVec<RefPtr<nsISFVItem>>) -> ::nserror::nsresult {
        ((*self.vtable).SetItems)(self, aItems)
    }


}


/// `interface nsISFVList : nsISFVSerialize`
///

/// ```text
/// /**
///  * nsISFVList represents List structured header value.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVList {
    vtable: *const nsISFVListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVList.
unsafe impl XpCom for nsISFVList {
    const IID: nsIID = nsID(0x02bb92a6, 0xd1de, 0x449c,
        [0xb5, 0x4f, 0xd1, 0x37, 0xf3, 0x0c, 0x61, 0x3d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVList {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVList.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVListCoerce {
    /// Cheaply cast a value of this type from a `nsISFVList`.
    fn coerce_from(v: &nsISFVList) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVListCoerce for nsISFVList {
    #[inline]
    fn coerce_from(v: &nsISFVList) -> &Self {
        v
    }
}

impl nsISFVList {
    /// Cast this `nsISFVList` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVList {
    type Target = nsISFVSerialize;
    #[inline]
    fn deref(&self) -> &nsISFVSerialize {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISFVSerializeCoerce> nsISFVListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVList) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVList
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVListVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISFVSerializeVTable,

    /* attribute Array<nsISFVItemOrInnerList> members; */
    pub GetMembers: unsafe extern "system" fn (this: *const nsISFVList, aMembers: *mut thin_vec::ThinVec<RefPtr<nsISFVItemOrInnerList>>) -> ::nserror::nsresult,

    /* attribute Array<nsISFVItemOrInnerList> members; */
    pub SetMembers: unsafe extern "system" fn (this: *const nsISFVList, aMembers: *const thin_vec::ThinVec<RefPtr<nsISFVItemOrInnerList>>) -> ::nserror::nsresult,

    /* void parseMore (in ACString header); */
    pub ParseMore: unsafe extern "system" fn (this: *const nsISFVList, header: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVList {

    /// ```text
    /// /**
    ///      * Returns array of members.
    ///      * QueryInterface can be used on a member to get more specific type.
    ///      */
    /// ```
    ///

    /// `attribute Array<nsISFVItemOrInnerList> members;`
    #[inline]
    pub unsafe fn GetMembers(&self, aMembers: *mut thin_vec::ThinVec<RefPtr<nsISFVItemOrInnerList>>) -> ::nserror::nsresult {
        ((*self.vtable).GetMembers)(self, aMembers)
    }


    /// ```text
    /// /**
    ///      * Returns array of members.
    ///      * QueryInterface can be used on a member to get more specific type.
    ///      */
    /// ```
    ///

    /// `attribute Array<nsISFVItemOrInnerList> members;`
    #[inline]
    pub unsafe fn SetMembers(&self, aMembers: *const thin_vec::ThinVec<RefPtr<nsISFVItemOrInnerList>>) -> ::nserror::nsresult {
        ((*self.vtable).SetMembers)(self, aMembers)
    }


    /// ```text
    /// /**
    ///      * In case when header value is split across lines, it's possible
    ///      * this method parses supplied line and merges it with members of existing object.
    ///      */
    /// ```
    ///

    /// `void parseMore (in ACString header);`
    #[inline]
    pub unsafe fn ParseMore(&self, header: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ParseMore)(self, header)
    }


}


/// `interface nsISFVDictionary : nsISFVSerialize`
///

/// ```text
/// /**
///  * nsISFVDictionary represents nsISFVDictionary structured header value.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVDictionary {
    vtable: *const nsISFVDictionaryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVDictionary.
unsafe impl XpCom for nsISFVDictionary {
    const IID: nsIID = nsID(0x6642a7fe, 0x7026, 0x4eba,
        [0xb7, 0x30, 0x05, 0xe2, 0x30, 0xee, 0x34, 0x37]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVDictionary {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVDictionary.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVDictionaryCoerce {
    /// Cheaply cast a value of this type from a `nsISFVDictionary`.
    fn coerce_from(v: &nsISFVDictionary) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVDictionaryCoerce for nsISFVDictionary {
    #[inline]
    fn coerce_from(v: &nsISFVDictionary) -> &Self {
        v
    }
}

impl nsISFVDictionary {
    /// Cast this `nsISFVDictionary` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVDictionaryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVDictionary {
    type Target = nsISFVSerialize;
    #[inline]
    fn deref(&self) -> &nsISFVSerialize {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISFVSerializeCoerce> nsISFVDictionaryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVDictionary) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVDictionary
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVDictionaryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISFVSerializeVTable,

    /* nsISFVItemOrInnerList get (in ACString key); */
    pub Get: unsafe extern "system" fn (this: *const nsISFVDictionary, key: *const ::nsstring::nsACString, _retval: *mut *const nsISFVItemOrInnerList) -> ::nserror::nsresult,

    /* void set (in ACString key, in nsISFVItemOrInnerList member_value); */
    pub Set: unsafe extern "system" fn (this: *const nsISFVDictionary, key: *const ::nsstring::nsACString, member_value: *const nsISFVItemOrInnerList) -> ::nserror::nsresult,

    /* void delete (in ACString key); */
    pub Delete: unsafe extern "system" fn (this: *const nsISFVDictionary, key: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* Array<ACString> keys (); */
    pub Keys: unsafe extern "system" fn (this: *const nsISFVDictionary, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* void parseMore (in ACString header); */
    pub ParseMore: unsafe extern "system" fn (this: *const nsISFVDictionary, header: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVDictionary {

    /// ```text
    /// /**
    ///      * Return value (nsISFVItemOrInnerList) stored for key, if it is present.
    ///      * QueryInterface can be used on a value to get more specific type.
    ///      *
    ///      * @throws NS_ERROR_UNEXPECTED if the key does not exist in parameters.
    ///      */
    /// ```
    ///

    /// `nsISFVItemOrInnerList get (in ACString key);`
    #[inline]
    pub unsafe fn Get(&self, key: *const ::nsstring::nsACString, _retval: *mut *const nsISFVItemOrInnerList) -> ::nserror::nsresult {
        ((*self.vtable).Get)(self, key, _retval)
    }


    /// ```text
    /// /**
    ///      * Insert a new key-value pair.
    ///      * If an equivalent key already exists: the key remains and retains in its place in the order,
    ///      * its corresponding value is updated with the new value.
    ///      *
    ///      * @throws NS_ERROR_UNEXPECTED if supplied item does not implement nsISFVItemOrInnerList interface.
    ///      */
    /// ```
    ///

    /// `void set (in ACString key, in nsISFVItemOrInnerList member_value);`
    #[inline]
    pub unsafe fn Set(&self, key: *const ::nsstring::nsACString, member_value: *const nsISFVItemOrInnerList) -> ::nserror::nsresult {
        ((*self.vtable).Set)(self, key, member_value)
    }


    /// ```text
    /// /**
    ///      * Remove the key-value pair equivalent to key.
    ///      *
    ///      * @throws NS_ERROR_UNEXPECTED upon attempt to delete  key that does not exist in parameters.
    ///      */
    /// ```
    ///

    /// `void delete (in ACString key);`
    #[inline]
    pub unsafe fn Delete(&self, key: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Delete)(self, key)
    }


    /// ```text
    /// /**
    ///      * Returns array of keys.
    ///      */
    /// ```
    ///

    /// `Array<ACString> keys ();`
    #[inline]
    pub unsafe fn Keys(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).Keys)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * In case when header value is split across lines, it's possible
    ///      * this method parses supplied line and merges it with members of existing object.
    ///      */
    /// ```
    ///

    /// `void parseMore (in ACString header);`
    #[inline]
    pub unsafe fn ParseMore(&self, header: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ParseMore)(self, header)
    }


}


/// `interface nsISFVService : nsISupports`
///

/// ```text
/// /**
///  * nsISFVService provides a set of functions for working with HTTP header value as an object.
///  * It exposes functions for creating object from string containing header value,
///  * as well as individual components for manual structured header object creation.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISFVService {
    vtable: *const nsISFVServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISFVService.
unsafe impl XpCom for nsISFVService {
    const IID: nsIID = nsID(0x049f4be1, 0x2f22, 0x4438,
        [0xa8, 0xda, 0x51, 0x85, 0x52, 0xed, 0x39, 0x0c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISFVService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISFVService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISFVServiceCoerce {
    /// Cheaply cast a value of this type from a `nsISFVService`.
    fn coerce_from(v: &nsISFVService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISFVServiceCoerce for nsISFVService {
    #[inline]
    fn coerce_from(v: &nsISFVService) -> &Self {
        v
    }
}

impl nsISFVService {
    /// Cast this `nsISFVService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISFVServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISFVService {
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
impl<T: nsISupportsCoerce> nsISFVServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISFVService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISFVService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISFVServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsISFVDictionary parseDictionary (in ACString header); */
    pub ParseDictionary: unsafe extern "system" fn (this: *const nsISFVService, header: *const ::nsstring::nsACString, _retval: *mut *const nsISFVDictionary) -> ::nserror::nsresult,

    /* nsISFVList parseList (in ACString header); */
    pub ParseList: unsafe extern "system" fn (this: *const nsISFVService, header: *const ::nsstring::nsACString, _retval: *mut *const nsISFVList) -> ::nserror::nsresult,

    /* nsISFVItem parseItem (in ACString header); */
    pub ParseItem: unsafe extern "system" fn (this: *const nsISFVService, header: *const ::nsstring::nsACString, _retval: *mut *const nsISFVItem) -> ::nserror::nsresult,

    /* nsISFVInteger newInteger (in long long value); */
    pub NewInteger: unsafe extern "system" fn (this: *const nsISFVService, value: i64, _retval: *mut *const nsISFVInteger) -> ::nserror::nsresult,

    /* nsISFVBool newBool (in bool value); */
    pub NewBool: unsafe extern "system" fn (this: *const nsISFVService, value: bool, _retval: *mut *const nsISFVBool) -> ::nserror::nsresult,

    /* nsISFVDecimal newDecimal (in double value); */
    pub NewDecimal: unsafe extern "system" fn (this: *const nsISFVService, value: libc::c_double, _retval: *mut *const nsISFVDecimal) -> ::nserror::nsresult,

    /* nsISFVString newString (in ACString value); */
    pub NewString: unsafe extern "system" fn (this: *const nsISFVService, value: *const ::nsstring::nsACString, _retval: *mut *const nsISFVString) -> ::nserror::nsresult,

    /* nsISFVByteSeq newByteSequence (in ACString value); */
    pub NewByteSequence: unsafe extern "system" fn (this: *const nsISFVService, value: *const ::nsstring::nsACString, _retval: *mut *const nsISFVByteSeq) -> ::nserror::nsresult,

    /* nsISFVToken newToken (in ACString value); */
    pub NewToken: unsafe extern "system" fn (this: *const nsISFVService, value: *const ::nsstring::nsACString, _retval: *mut *const nsISFVToken) -> ::nserror::nsresult,

    /* nsISFVParams newParameters (); */
    pub NewParameters: unsafe extern "system" fn (this: *const nsISFVService, _retval: *mut *const nsISFVParams) -> ::nserror::nsresult,

    /* nsISFVInnerList newInnerList (in Array<nsISFVItem> items, in nsISFVParams params); */
    pub NewInnerList: unsafe extern "system" fn (this: *const nsISFVService, items: *const thin_vec::ThinVec<RefPtr<nsISFVItem>>, params: *const nsISFVParams, _retval: *mut *const nsISFVInnerList) -> ::nserror::nsresult,

    /* nsISFVItem newItem (in nsISFVBareItem value, in nsISFVParams params); */
    pub NewItem: unsafe extern "system" fn (this: *const nsISFVService, value: *const nsISFVBareItem, params: *const nsISFVParams, _retval: *mut *const nsISFVItem) -> ::nserror::nsresult,

    /* nsISFVList newList (in Array<nsISFVItemOrInnerList> members); */
    pub NewList: unsafe extern "system" fn (this: *const nsISFVService, members: *const thin_vec::ThinVec<RefPtr<nsISFVItemOrInnerList>>, _retval: *mut *const nsISFVList) -> ::nserror::nsresult,

    /* nsISFVDictionary newDictionary (); */
    pub NewDictionary: unsafe extern "system" fn (this: *const nsISFVService, _retval: *mut *const nsISFVDictionary) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISFVService {

    /// ```text
    /// /**
    ///      * Parses provided string into Dictionary header value (nsISFVDictionary).
    ///      *
    ///      * @throws NS_ERROR_FAILURE if parsing fails.
    ///      */
    /// ```
    ///

    /// `nsISFVDictionary parseDictionary (in ACString header);`
    #[inline]
    pub unsafe fn ParseDictionary(&self, header: *const ::nsstring::nsACString, _retval: *mut *const nsISFVDictionary) -> ::nserror::nsresult {
        ((*self.vtable).ParseDictionary)(self, header, _retval)
    }


    /// ```text
    /// /**
    ///      * Parses provided string into List header value (nsISFVList).
    ///      *
    ///      * @throws NS_ERROR_FAILURE if parsing fails.
    ///      */
    /// ```
    ///

    /// `nsISFVList parseList (in ACString header);`
    #[inline]
    pub unsafe fn ParseList(&self, header: *const ::nsstring::nsACString, _retval: *mut *const nsISFVList) -> ::nserror::nsresult {
        ((*self.vtable).ParseList)(self, header, _retval)
    }


    /// ```text
    /// /**
    ///      * Parses provided string into Item header value (nsISFVItem).
    ///      *
    ///      * @throws NS_ERROR_FAILURE if parsing fails.
    ///      */
    /// ```
    ///

    /// `nsISFVItem parseItem (in ACString header);`
    #[inline]
    pub unsafe fn ParseItem(&self, header: *const ::nsstring::nsACString, _retval: *mut *const nsISFVItem) -> ::nserror::nsresult {
        ((*self.vtable).ParseItem)(self, header, _retval)
    }


    /// ```text
    /// /**
    ///      * The following functions create bare item of specific type.
    ///      */
    /// ```
    ///

    /// `nsISFVInteger newInteger (in long long value);`
    #[inline]
    pub unsafe fn NewInteger(&self, value: i64, _retval: *mut *const nsISFVInteger) -> ::nserror::nsresult {
        ((*self.vtable).NewInteger)(self, value, _retval)
    }



    /// `nsISFVBool newBool (in bool value);`
    #[inline]
    pub unsafe fn NewBool(&self, value: bool, _retval: *mut *const nsISFVBool) -> ::nserror::nsresult {
        ((*self.vtable).NewBool)(self, value, _retval)
    }



    /// `nsISFVDecimal newDecimal (in double value);`
    #[inline]
    pub unsafe fn NewDecimal(&self, value: libc::c_double, _retval: *mut *const nsISFVDecimal) -> ::nserror::nsresult {
        ((*self.vtable).NewDecimal)(self, value, _retval)
    }



    /// `nsISFVString newString (in ACString value);`
    #[inline]
    pub unsafe fn NewString(&self, value: *const ::nsstring::nsACString, _retval: *mut *const nsISFVString) -> ::nserror::nsresult {
        ((*self.vtable).NewString)(self, value, _retval)
    }



    /// `nsISFVByteSeq newByteSequence (in ACString value);`
    #[inline]
    pub unsafe fn NewByteSequence(&self, value: *const ::nsstring::nsACString, _retval: *mut *const nsISFVByteSeq) -> ::nserror::nsresult {
        ((*self.vtable).NewByteSequence)(self, value, _retval)
    }



    /// `nsISFVToken newToken (in ACString value);`
    #[inline]
    pub unsafe fn NewToken(&self, value: *const ::nsstring::nsACString, _retval: *mut *const nsISFVToken) -> ::nserror::nsresult {
        ((*self.vtable).NewToken)(self, value, _retval)
    }


    /// ```text
    /// /**
    ///      * Creates nsISFVParams with no parameters. In other words, it's an empty map byt default.
    ///      */
    /// ```
    ///

    /// `nsISFVParams newParameters ();`
    #[inline]
    pub unsafe fn NewParameters(&self, _retval: *mut *const nsISFVParams) -> ::nserror::nsresult {
        ((*self.vtable).NewParameters)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Creates nsISFVInnerList from nsISFVItem array and nsISFVParams.
    ///      */
    /// ```
    ///

    /// `nsISFVInnerList newInnerList (in Array<nsISFVItem> items, in nsISFVParams params);`
    #[inline]
    pub unsafe fn NewInnerList(&self, items: *const thin_vec::ThinVec<RefPtr<nsISFVItem>>, params: *const nsISFVParams, _retval: *mut *const nsISFVInnerList) -> ::nserror::nsresult {
        ((*self.vtable).NewInnerList)(self, items, params, _retval)
    }


    /// ```text
    /// /**
    ///      * Creates nsISFVItem, which represents Item header value, from nsISFVBareItem and associated nsISFVParams.
    ///      */
    /// ```
    ///

    /// `nsISFVItem newItem (in nsISFVBareItem value, in nsISFVParams params);`
    #[inline]
    pub unsafe fn NewItem(&self, value: *const nsISFVBareItem, params: *const nsISFVParams, _retval: *mut *const nsISFVItem) -> ::nserror::nsresult {
        ((*self.vtable).NewItem)(self, value, params, _retval)
    }


    /// ```text
    /// /**
    ///      * Creates nsISFVList, which represents List header value, from array of nsISFVItemOrInnerList.
    ///      * nsISFVItemOrInnerList represens either Item (nsISFVItem) or Inner List (nsISFVInnerList).
    ///      */
    /// ```
    ///

    /// `nsISFVList newList (in Array<nsISFVItemOrInnerList> members);`
    #[inline]
    pub unsafe fn NewList(&self, members: *const thin_vec::ThinVec<RefPtr<nsISFVItemOrInnerList>>, _retval: *mut *const nsISFVList) -> ::nserror::nsresult {
        ((*self.vtable).NewList)(self, members, _retval)
    }


    /// ```text
    /// /**
    ///      * Creates nsISFVDictionary representing Dictionary header value. It is empty by default.
    ///      */
    /// ```
    ///

    /// `nsISFVDictionary newDictionary ();`
    #[inline]
    pub unsafe fn NewDictionary(&self, _retval: *mut *const nsISFVDictionary) -> ::nserror::nsresult {
        ((*self.vtable).NewDictionary)(self, _retval)
    }


}


