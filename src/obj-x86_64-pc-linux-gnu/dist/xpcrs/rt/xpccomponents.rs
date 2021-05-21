//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/xpccomponents.idl
//


/// `interface nsIXPCComponents_Interfaces : nsISupports`
///

/// ```text
/// /**
/// * interface of Components.interfaces
/// * (interesting stuff only reflected into JavaScript)
/// */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPCComponents_Interfaces {
    vtable: *const nsIXPCComponents_InterfacesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPCComponents_Interfaces.
unsafe impl XpCom for nsIXPCComponents_Interfaces {
    const IID: nsIID = nsID(0xb8c31bba, 0x79db, 0x4a1d,
        [0x93, 0x0d, 0x4c, 0xdd, 0x68, 0x71, 0x3f, 0x9e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPCComponents_Interfaces {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPCComponents_Interfaces.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPCComponents_InterfacesCoerce {
    /// Cheaply cast a value of this type from a `nsIXPCComponents_Interfaces`.
    fn coerce_from(v: &nsIXPCComponents_Interfaces) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPCComponents_InterfacesCoerce for nsIXPCComponents_Interfaces {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Interfaces) -> &Self {
        v
    }
}

impl nsIXPCComponents_Interfaces {
    /// Cast this `nsIXPCComponents_Interfaces` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPCComponents_InterfacesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPCComponents_Interfaces {
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
impl<T: nsISupportsCoerce> nsIXPCComponents_InterfacesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Interfaces) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPCComponents_Interfaces
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPCComponents_InterfacesVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPCComponents_Interfaces {


}


/// `interface nsIXPCComponents_Classes : nsISupports`
///

/// ```text
/// /**
/// * interface of Components.classes
/// * (interesting stuff only reflected into JavaScript)
/// */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPCComponents_Classes {
    vtable: *const nsIXPCComponents_ClassesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPCComponents_Classes.
unsafe impl XpCom for nsIXPCComponents_Classes {
    const IID: nsIID = nsID(0x978ff520, 0xd26c, 0x11d2,
        [0x98, 0x42, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPCComponents_Classes {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPCComponents_Classes.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPCComponents_ClassesCoerce {
    /// Cheaply cast a value of this type from a `nsIXPCComponents_Classes`.
    fn coerce_from(v: &nsIXPCComponents_Classes) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPCComponents_ClassesCoerce for nsIXPCComponents_Classes {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Classes) -> &Self {
        v
    }
}

impl nsIXPCComponents_Classes {
    /// Cast this `nsIXPCComponents_Classes` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPCComponents_ClassesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPCComponents_Classes {
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
impl<T: nsISupportsCoerce> nsIXPCComponents_ClassesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Classes) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPCComponents_Classes
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPCComponents_ClassesVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPCComponents_Classes {


}


/// `interface nsIXPCComponents_Results : nsISupports`
///

/// ```text
/// /**
/// * interface of Components.results
/// * (interesting stuff only reflected into JavaScript)
/// */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPCComponents_Results {
    vtable: *const nsIXPCComponents_ResultsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPCComponents_Results.
unsafe impl XpCom for nsIXPCComponents_Results {
    const IID: nsIID = nsID(0x2fc229a0, 0x5860, 0x11d3,
        [0x98, 0x99, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPCComponents_Results {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPCComponents_Results.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPCComponents_ResultsCoerce {
    /// Cheaply cast a value of this type from a `nsIXPCComponents_Results`.
    fn coerce_from(v: &nsIXPCComponents_Results) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPCComponents_ResultsCoerce for nsIXPCComponents_Results {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Results) -> &Self {
        v
    }
}

impl nsIXPCComponents_Results {
    /// Cast this `nsIXPCComponents_Results` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPCComponents_ResultsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPCComponents_Results {
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
impl<T: nsISupportsCoerce> nsIXPCComponents_ResultsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Results) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPCComponents_Results
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPCComponents_ResultsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPCComponents_Results {


}


/// `interface nsIXPCComponents_ID : nsISupports`
///

/// ```text
/// /**
/// * interface of Components.ID
/// * (interesting stuff only reflected into JavaScript)
/// */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPCComponents_ID {
    vtable: *const nsIXPCComponents_IDVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPCComponents_ID.
unsafe impl XpCom for nsIXPCComponents_ID {
    const IID: nsIID = nsID(0x7994a6e0, 0xe028, 0x11d3,
        [0x8f, 0x5d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPCComponents_ID {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPCComponents_ID.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPCComponents_IDCoerce {
    /// Cheaply cast a value of this type from a `nsIXPCComponents_ID`.
    fn coerce_from(v: &nsIXPCComponents_ID) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPCComponents_IDCoerce for nsIXPCComponents_ID {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_ID) -> &Self {
        v
    }
}

impl nsIXPCComponents_ID {
    /// Cast this `nsIXPCComponents_ID` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPCComponents_IDCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPCComponents_ID {
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
impl<T: nsISupportsCoerce> nsIXPCComponents_IDCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_ID) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPCComponents_ID
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPCComponents_IDVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPCComponents_ID {


}


/// `interface nsIXPCComponents_Exception : nsISupports`
///

/// ```text
/// /**
/// * interface of Components.Exception
/// * (interesting stuff only reflected into JavaScript)
/// */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPCComponents_Exception {
    vtable: *const nsIXPCComponents_ExceptionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPCComponents_Exception.
unsafe impl XpCom for nsIXPCComponents_Exception {
    const IID: nsIID = nsID(0x5bf039c0, 0xe028, 0x11d3,
        [0x8f, 0x5d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPCComponents_Exception {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPCComponents_Exception.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPCComponents_ExceptionCoerce {
    /// Cheaply cast a value of this type from a `nsIXPCComponents_Exception`.
    fn coerce_from(v: &nsIXPCComponents_Exception) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPCComponents_ExceptionCoerce for nsIXPCComponents_Exception {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Exception) -> &Self {
        v
    }
}

impl nsIXPCComponents_Exception {
    /// Cast this `nsIXPCComponents_Exception` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPCComponents_ExceptionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPCComponents_Exception {
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
impl<T: nsISupportsCoerce> nsIXPCComponents_ExceptionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Exception) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPCComponents_Exception
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPCComponents_ExceptionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPCComponents_Exception {


}


/// `interface nsIXPCComponents_Constructor : nsISupports`
///

/// ```text
/// /**
/// * interface of Components.Constructor
/// * (interesting stuff only reflected into JavaScript)
/// */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPCComponents_Constructor {
    vtable: *const nsIXPCComponents_ConstructorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPCComponents_Constructor.
unsafe impl XpCom for nsIXPCComponents_Constructor {
    const IID: nsIID = nsID(0x88655640, 0xe028, 0x11d3,
        [0x8f, 0x5d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPCComponents_Constructor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPCComponents_Constructor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPCComponents_ConstructorCoerce {
    /// Cheaply cast a value of this type from a `nsIXPCComponents_Constructor`.
    fn coerce_from(v: &nsIXPCComponents_Constructor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPCComponents_ConstructorCoerce for nsIXPCComponents_Constructor {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Constructor) -> &Self {
        v
    }
}

impl nsIXPCComponents_Constructor {
    /// Cast this `nsIXPCComponents_Constructor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPCComponents_ConstructorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPCComponents_Constructor {
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
impl<T: nsISupportsCoerce> nsIXPCComponents_ConstructorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Constructor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPCComponents_Constructor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPCComponents_ConstructorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPCComponents_Constructor {


}


/// `interface nsIXPCComponents_utils_Sandbox : nsISupports`
///

/// ```text
/// /**
/// * interface of object returned by Components.utils.Sandbox.
/// */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPCComponents_utils_Sandbox {
    vtable: *const nsIXPCComponents_utils_SandboxVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPCComponents_utils_Sandbox.
unsafe impl XpCom for nsIXPCComponents_utils_Sandbox {
    const IID: nsIID = nsID(0x4f8ae0dc, 0xd266, 0x4a32,
        [0x87, 0x5b, 0x6a, 0x9d, 0xe7, 0x1a, 0x8c, 0xe9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPCComponents_utils_Sandbox {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPCComponents_utils_Sandbox.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPCComponents_utils_SandboxCoerce {
    /// Cheaply cast a value of this type from a `nsIXPCComponents_utils_Sandbox`.
    fn coerce_from(v: &nsIXPCComponents_utils_Sandbox) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPCComponents_utils_SandboxCoerce for nsIXPCComponents_utils_Sandbox {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_utils_Sandbox) -> &Self {
        v
    }
}

impl nsIXPCComponents_utils_Sandbox {
    /// Cast this `nsIXPCComponents_utils_Sandbox` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPCComponents_utils_SandboxCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPCComponents_utils_Sandbox {
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
impl<T: nsISupportsCoerce> nsIXPCComponents_utils_SandboxCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_utils_Sandbox) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPCComponents_utils_Sandbox
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPCComponents_utils_SandboxVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPCComponents_utils_Sandbox {


}


/// `interface nsIScheduledGCCallback : nsISupports`
///

/// ```text
/// /**
///  * interface for callback to be passed to Cu.schedulePreciseGC
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScheduledGCCallback {
    vtable: *const nsIScheduledGCCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScheduledGCCallback.
unsafe impl XpCom for nsIScheduledGCCallback {
    const IID: nsIID = nsID(0x71000535, 0xb0fd, 0x44d1,
        [0x8c, 0xe0, 0x90, 0x97, 0x60, 0xe3, 0x95, 0x3c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScheduledGCCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScheduledGCCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScheduledGCCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIScheduledGCCallback`.
    fn coerce_from(v: &nsIScheduledGCCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScheduledGCCallbackCoerce for nsIScheduledGCCallback {
    #[inline]
    fn coerce_from(v: &nsIScheduledGCCallback) -> &Self {
        v
    }
}

impl nsIScheduledGCCallback {
    /// Cast this `nsIScheduledGCCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScheduledGCCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScheduledGCCallback {
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
impl<T: nsISupportsCoerce> nsIScheduledGCCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScheduledGCCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScheduledGCCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScheduledGCCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void callback (); */
    pub Callback: unsafe extern "system" fn (this: *const nsIScheduledGCCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScheduledGCCallback {


    /// `void callback ();`
    #[inline]
    pub unsafe fn Callback(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Callback)(self, )
    }


}


/// `interface nsIXPCComponents_Utils : nsISupports`
///

/// ```text
/// /**
/// * interface of Components.utils
/// */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPCComponents_Utils {
    vtable: *const nsIXPCComponents_UtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPCComponents_Utils.
unsafe impl XpCom for nsIXPCComponents_Utils {
    const IID: nsIID = nsID(0x86003fe3, 0xee9a, 0x4620,
        [0x91, 0xdc, 0xee, 0xf8, 0xb1, 0xe5, 0x88, 0x15]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPCComponents_Utils {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPCComponents_Utils.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPCComponents_UtilsCoerce {
    /// Cheaply cast a value of this type from a `nsIXPCComponents_Utils`.
    fn coerce_from(v: &nsIXPCComponents_Utils) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPCComponents_UtilsCoerce for nsIXPCComponents_Utils {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Utils) -> &Self {
        v
    }
}

impl nsIXPCComponents_Utils {
    /// Cast this `nsIXPCComponents_Utils` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPCComponents_UtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPCComponents_Utils {
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
impl<T: nsISupportsCoerce> nsIXPCComponents_UtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Utils) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPCComponents_Utils
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPCComponents_UtilsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void reportError (in jsval error, [optional] in jsval stack); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub ReportError: *const ::libc::c_void,

    /* readonly attribute nsIXPCComponents_utils_Sandbox Sandbox; */
    pub GetSandbox: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, aSandbox: *mut *const nsIXPCComponents_utils_Sandbox) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval createServicesCache (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub CreateServicesCache: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] jsval evalInSandbox (in AString source, in jsval sandbox, [optional] in jsval version, [optional] in AUTF8String filename, [optional] in long lineNo, [optional] in bool enforceFilenameRestrictions); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub EvalInSandbox: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getUAWidgetScope (in nsIPrincipal principal); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetUAWidgetScope: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getSandboxMetadata (in jsval sandbox); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetSandboxMetadata: *const ::libc::c_void,

    /* [implicit_jscontext] void setSandboxMetadata (in jsval sandbox, in jsval metadata); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub SetSandboxMetadata: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] jsval import (in AUTF8String aResourceURI, [optional] in jsval targetObj); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Import: *const ::libc::c_void,

    /* boolean isModuleLoaded (in AUTF8String aResourceURI); */
    pub IsModuleLoaded: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, aResourceURI: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void unload (in AUTF8String registryLocation); */
    pub Unload: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, registryLocation: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [implicit_jscontext] void importGlobalProperties (in jsval aPropertyList); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub ImportGlobalProperties: *const ::libc::c_void,

    /* [implicit_jscontext] xpcIJSWeakReference getWeakReference (in jsval obj); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetWeakReference: *const ::libc::c_void,

    /* [implicit_jscontext] void forceGC (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub ForceGC: *const ::libc::c_void,

    /* void forceCC ([optional] in nsICycleCollectorListener aListener); */
    pub ForceCC: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, aListener: *const nsICycleCollectorListener) -> ::nserror::nsresult,

    /* nsICycleCollectorListener createCCLogger (); */
    pub CreateCCLogger: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, _retval: *mut*const nsICycleCollectorListener) -> ::nserror::nsresult,

    /* void finishCC (); */
    pub FinishCC: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils) -> ::nserror::nsresult,

    /* void ccSlice (in long long budget); */
    pub CcSlice: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, budget: i64) -> ::nserror::nsresult,

    /* long getMaxCCSliceTimeSinceClear (); */
    pub GetMaxCCSliceTimeSinceClear: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, _retval: *mut i32) -> ::nserror::nsresult,

    /* void clearMaxCCTime (); */
    pub ClearMaxCCTime: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils) -> ::nserror::nsresult,

    /* [implicit_jscontext] void forceShrinkingGC (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub ForceShrinkingGC: *const ::libc::c_void,

    /* void schedulePreciseGC (in nsIScheduledGCCallback callback); */
    pub SchedulePreciseGC: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, callback: *const nsIScheduledGCCallback) -> ::nserror::nsresult,

    /* void schedulePreciseShrinkingGC (in nsIScheduledGCCallback callback); */
    pub SchedulePreciseShrinkingGC: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, callback: *const nsIScheduledGCCallback) -> ::nserror::nsresult,

    /* void unlinkGhostWindows (); */
    pub UnlinkGhostWindows: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils) -> ::nserror::nsresult,

    /* void intentionallyLeak (); */
    pub IntentionallyLeak: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval getJSTestingFunctions (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetJSTestingFunctions: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getFunctionSourceLocation (in jsval func); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetFunctionSourceLocation: *const ::libc::c_void,

    /* [implicit_jscontext] jsval callFunctionWithAsyncStack (in jsval function, in nsIStackFrame stack, in AString asyncCause); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub CallFunctionWithAsyncStack: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getGlobalForObject (in jsval obj); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetGlobalForObject: *const ::libc::c_void,

    /* [implicit_jscontext] boolean isProxy (in jsval vobject); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub IsProxy: *const ::libc::c_void,

    /* [implicit_jscontext] jsval exportFunction (in jsval vfunction, in jsval vscope, [optional] in jsval voptions); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub ExportFunction: *const ::libc::c_void,

    /* [implicit_jscontext] jsval createObjectIn (in jsval vobj, [optional] in jsval voptions); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub CreateObjectIn: *const ::libc::c_void,

    /* [implicit_jscontext] void makeObjectPropsNormal (in jsval vobj); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub MakeObjectPropsNormal: *const ::libc::c_void,

    /* bool isDeadWrapper (in jsval obj); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub IsDeadWrapper: *const ::libc::c_void,

    /* bool isRemoteProxy (in jsval val); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub IsRemoteProxy: *const ::libc::c_void,

    /* [implicit_jscontext] void recomputeWrappers ([optional] in jsval vobj); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub RecomputeWrappers: *const ::libc::c_void,

    /* [implicit_jscontext] void setWantXrays (in jsval vscope); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub SetWantXrays: *const ::libc::c_void,

    /* [implicit_jscontext] void dispatch (in jsval runnable, [optional] in jsval scope); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Dispatch: *const ::libc::c_void,

    /* [implicit_jscontext] attribute boolean strict_mode; */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetStrict_mode: *const ::libc::c_void,

    /* [implicit_jscontext] attribute boolean strict_mode; */
    /// Unable to generate binding because `jscontext is unsupported`
    pub SetStrict_mode: *const ::libc::c_void,

    /* readonly attribute boolean isInAutomation; */
    pub GetIsInAutomation: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, aIsInAutomation: *mut bool) -> ::nserror::nsresult,

    /* void exitIfInAutomation (); */
    pub ExitIfInAutomation: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils) -> ::nserror::nsresult,

    /* void crashIfNotInAutomation (); */
    pub CrashIfNotInAutomation: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils) -> ::nserror::nsresult,

    /* [implicit_jscontext] void setGCZeal (in long zeal); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub SetGCZeal: *const ::libc::c_void,

    /* [implicit_jscontext] void nukeSandbox (in jsval obj); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub NukeSandbox: *const ::libc::c_void,

    /* [implicit_jscontext] void blockScriptForGlobal (in jsval global); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub BlockScriptForGlobal: *const ::libc::c_void,

    /* [implicit_jscontext] void unblockScriptForGlobal (in jsval global); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub UnblockScriptForGlobal: *const ::libc::c_void,

    /* bool isOpaqueWrapper (in jsval obj); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub IsOpaqueWrapper: *const ::libc::c_void,

    /* bool isXrayWrapper (in jsval obj); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub IsXrayWrapper: *const ::libc::c_void,

    /* [implicit_jscontext] jsval waiveXrays (in jsval aVal); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub WaiveXrays: *const ::libc::c_void,

    /* [implicit_jscontext] jsval unwaiveXrays (in jsval aVal); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub UnwaiveXrays: *const ::libc::c_void,

    /* [implicit_jscontext] string getClassName (in jsval aObj, in bool aUnwrap); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetClassName: *const ::libc::c_void,

    /* nsIClassInfo getDOMClassInfo (in AString aClassName); */
    pub GetDOMClassInfo: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, aClassName: *const ::nsstring::nsAString, _retval: *mut*const nsIClassInfo) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval getIncumbentGlobal ([optional] in jsval callback); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetIncumbentGlobal: *const ::libc::c_void,

    /* [implicit_jscontext] nsISupports generateXPCWrappedJS (in jsval obj, [optional] in jsval scope); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GenerateXPCWrappedJS: *const ::libc::c_void,

    /* PRTime getWatchdogTimestamp (in AString aCategory); */
    pub GetWatchdogTimestamp: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, aCategory: *const ::nsstring::nsAString, _retval: *mut PRTime) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval getJSEngineTelemetryValue (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetJSEngineTelemetryValue: *const ::libc::c_void,

    /* [implicit_jscontext] jsval cloneInto (in jsval value, in jsval scope, [optional] in jsval options); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub CloneInto: *const ::libc::c_void,

    /* nsIPrincipal getWebIDLCallerPrincipal (); */
    pub GetWebIDLCallerPrincipal: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, _retval: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* [implicit_jscontext] nsIPrincipal getObjectPrincipal (in jsval obj); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetObjectPrincipal: *const ::libc::c_void,

    /* [implicit_jscontext] ACString getRealmLocation (in jsval obj); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetRealmLocation: *const ::libc::c_void,

    /* double now (); */
    pub Now: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* AUTF8String readUTF8File (in nsIFile file); */
    pub ReadUTF8File: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, file: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String readUTF8URI (in nsIURI url); */
    pub ReadUTF8URI: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, url: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsIEditorSpellCheck createSpellChecker (); */
    pub CreateSpellChecker: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, _retval: *mut*const nsIEditorSpellCheck) -> ::nserror::nsresult,

    /* nsISupports createCommandLine ([optional] in nsIFile aWorkingDir); */
    pub CreateCommandLine: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, aWorkingDir: *const nsIFile, _retval: *mut *const nsISupports) -> ::nserror::nsresult,

    /* nsICommandParams createCommandParams (); */
    pub CreateCommandParams: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, _retval: *mut*const nsICommandParams) -> ::nserror::nsresult,

    /* nsILoadContext createLoadContext (); */
    pub CreateLoadContext: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, _retval: *mut*const nsILoadContext) -> ::nserror::nsresult,

    /* nsILoadContext createPrivateLoadContext (); */
    pub CreatePrivateLoadContext: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, _retval: *mut*const nsILoadContext) -> ::nserror::nsresult,

    /* nsIPersistentProperties createPersistentProperties (); */
    pub CreatePersistentProperties: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, _retval: *mut*const nsIPersistentProperties) -> ::nserror::nsresult,

    /* nsIDocumentEncoder createDocumentEncoder (in string contentType); */
    pub CreateDocumentEncoder: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, contentType: *const libc::c_char, _retval: *mut*const nsIDocumentEncoder) -> ::nserror::nsresult,

    /* nsIDocumentEncoder createHTMLCopyEncoder (); */
    pub CreateHTMLCopyEncoder: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, _retval: *mut*const nsIDocumentEncoder) -> ::nserror::nsresult,

    /* readonly attribute Array<ACString> loadedModules; */
    pub GetLoadedModules: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, aLoadedModules: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute Array<ACString> loadedComponents; */
    pub GetLoadedComponents: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, aLoadedComponents: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* ACString getModuleImportStack (in AUTF8String aLocation); */
    pub GetModuleImportStack: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, aLocation: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getComponentLoadStack (in AUTF8String aLocation); */
    pub GetComponentLoadStack: unsafe extern "system" fn (this: *const nsIXPCComponents_Utils, aLocation: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPCComponents_Utils {


    /// `[implicit_jscontext] void reportError (in jsval error, [optional] in jsval stack);`
    const _ReportError: () = ();


    /// `readonly attribute nsIXPCComponents_utils_Sandbox Sandbox;`
    #[inline]
    pub unsafe fn GetSandbox(&self, aSandbox: *mut *const nsIXPCComponents_utils_Sandbox) -> ::nserror::nsresult {
        ((*self.vtable).GetSandbox)(self, aSandbox)
    }



    /// `[implicit_jscontext] jsval createServicesCache ();`
    const _CreateServicesCache: () = ();


    /// `[implicit_jscontext,optional_argc] jsval evalInSandbox (in AString source, in jsval sandbox, [optional] in jsval version, [optional] in AUTF8String filename, [optional] in long lineNo, [optional] in bool enforceFilenameRestrictions);`
    const _EvalInSandbox: () = ();


    /// `[implicit_jscontext] jsval getUAWidgetScope (in nsIPrincipal principal);`
    const _GetUAWidgetScope: () = ();


    /// `[implicit_jscontext] jsval getSandboxMetadata (in jsval sandbox);`
    const _GetSandboxMetadata: () = ();


    /// `[implicit_jscontext] void setSandboxMetadata (in jsval sandbox, in jsval metadata);`
    const _SetSandboxMetadata: () = ();


    /// `[implicit_jscontext,optional_argc] jsval import (in AUTF8String aResourceURI, [optional] in jsval targetObj);`
    const _Import: () = ();

    /// ```text
    /// /**
    ///      * Returns true if the js file located at 'registryLocation' location has
    ///      * been loaded previously via the import method above. Returns false
    ///      * otherwise.
    ///      *
    ///      * @param resourceURI A resource:// URI string representing the location of
    ///      *        the js file to be checked if it is already loaded or not.
    ///      * @returns boolean, true if the js file has been loaded via import. false
    ///      *          otherwise
    ///      */
    /// ```
    ///

    /// `boolean isModuleLoaded (in AUTF8String aResourceURI);`
    #[inline]
    pub unsafe fn IsModuleLoaded(&self, aResourceURI: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsModuleLoaded)(self, aResourceURI, _retval)
    }



    /// `void unload (in AUTF8String registryLocation);`
    #[inline]
    pub unsafe fn Unload(&self, registryLocation: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Unload)(self, registryLocation)
    }



    /// `[implicit_jscontext] void importGlobalProperties (in jsval aPropertyList);`
    const _ImportGlobalProperties: () = ();


    /// `[implicit_jscontext] xpcIJSWeakReference getWeakReference (in jsval obj);`
    const _GetWeakReference: () = ();


    /// `[implicit_jscontext] void forceGC ();`
    const _ForceGC: () = ();


    /// `void forceCC ([optional] in nsICycleCollectorListener aListener);`
    #[inline]
    pub unsafe fn ForceCC(&self, aListener: *const nsICycleCollectorListener) -> ::nserror::nsresult {
        ((*self.vtable).ForceCC)(self, aListener)
    }



    /// `nsICycleCollectorListener createCCLogger ();`
    #[inline]
    pub unsafe fn CreateCCLogger(&self, _retval: *mut*const nsICycleCollectorListener) -> ::nserror::nsresult {
        ((*self.vtable).CreateCCLogger)(self, _retval)
    }



    /// `void finishCC ();`
    #[inline]
    pub unsafe fn FinishCC(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).FinishCC)(self, )
    }



    /// `void ccSlice (in long long budget);`
    #[inline]
    pub unsafe fn CcSlice(&self, budget: i64) -> ::nserror::nsresult {
        ((*self.vtable).CcSlice)(self, budget)
    }



    /// `long getMaxCCSliceTimeSinceClear ();`
    #[inline]
    pub unsafe fn GetMaxCCSliceTimeSinceClear(&self, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxCCSliceTimeSinceClear)(self, _retval)
    }



    /// `void clearMaxCCTime ();`
    #[inline]
    pub unsafe fn ClearMaxCCTime(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearMaxCCTime)(self, )
    }



    /// `[implicit_jscontext] void forceShrinkingGC ();`
    const _ForceShrinkingGC: () = ();


    /// `void schedulePreciseGC (in nsIScheduledGCCallback callback);`
    #[inline]
    pub unsafe fn SchedulePreciseGC(&self, callback: *const nsIScheduledGCCallback) -> ::nserror::nsresult {
        ((*self.vtable).SchedulePreciseGC)(self, callback)
    }



    /// `void schedulePreciseShrinkingGC (in nsIScheduledGCCallback callback);`
    #[inline]
    pub unsafe fn SchedulePreciseShrinkingGC(&self, callback: *const nsIScheduledGCCallback) -> ::nserror::nsresult {
        ((*self.vtable).SchedulePreciseShrinkingGC)(self, callback)
    }



    /// `void unlinkGhostWindows ();`
    #[inline]
    pub unsafe fn UnlinkGhostWindows(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UnlinkGhostWindows)(self, )
    }



    /// `void intentionallyLeak ();`
    #[inline]
    pub unsafe fn IntentionallyLeak(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).IntentionallyLeak)(self, )
    }



    /// `[implicit_jscontext] jsval getJSTestingFunctions ();`
    const _GetJSTestingFunctions: () = ();

    /// ```text
    /// /**
    ///      * Returns an object containing `filename` and `lineNumber` properties
    ///      * describing the source location of the given function.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getFunctionSourceLocation (in jsval func);`
    const _GetFunctionSourceLocation: () = ();


    /// `[implicit_jscontext] jsval callFunctionWithAsyncStack (in jsval function, in nsIStackFrame stack, in AString asyncCause);`
    const _CallFunctionWithAsyncStack: () = ();


    /// `[implicit_jscontext] jsval getGlobalForObject (in jsval obj);`
    const _GetGlobalForObject: () = ();


    /// `[implicit_jscontext] boolean isProxy (in jsval vobject);`
    const _IsProxy: () = ();


    /// `[implicit_jscontext] jsval exportFunction (in jsval vfunction, in jsval vscope, [optional] in jsval voptions);`
    const _ExportFunction: () = ();


    /// `[implicit_jscontext] jsval createObjectIn (in jsval vobj, [optional] in jsval voptions);`
    const _CreateObjectIn: () = ();


    /// `[implicit_jscontext] void makeObjectPropsNormal (in jsval vobj);`
    const _MakeObjectPropsNormal: () = ();

    /// ```text
    /// /**
    ///      * Determines whether this object is backed by a DeadObjectProxy.
    ///      *
    ///      * Dead-wrapper objects hold no other objects alive (they have no outgoing
        ///      * reference edges) and will throw if you touch them (e.g. by
        ///      * reading/writing a property).
    ///      */
    /// ```
    ///

    /// `bool isDeadWrapper (in jsval obj);`
    const _IsDeadWrapper: () = ();

    /// ```text
    /// /**
    ///      * Determines whether this value is a remote object proxy, such as
    ///      * RemoteWindowProxy or RemoteLocationProxy, for an out-of-process frame.
    ///      *
    ///      * Remote object proxies do not grant chrome callers the same exemptions
    ///      * to the same-origin-policy that in-process wrappers typically do, so
    ///      * this can be used to determine whether access to cross-origin proxies is
    ///      * safe:
    ///      *
    ///      *   if (!Cu.isRemoteProxy(frame.contentWindow)) {
        ///      *     frame.contentWindow.doCrossOriginThing();
        ///      *   }
    ///      */
    /// ```
    ///

    /// `bool isRemoteProxy (in jsval val);`
    const _IsRemoteProxy: () = ();


    /// `[implicit_jscontext] void recomputeWrappers ([optional] in jsval vobj);`
    const _RecomputeWrappers: () = ();


    /// `[implicit_jscontext] void setWantXrays (in jsval vscope);`
    const _SetWantXrays: () = ();


    /// `[implicit_jscontext] void dispatch (in jsval runnable, [optional] in jsval scope);`
    const _Dispatch: () = ();


    /// `[implicit_jscontext] attribute boolean strict_mode;`
    const _GetStrict_mode: () = ();


    /// `[implicit_jscontext] attribute boolean strict_mode;`
    const _SetStrict_mode: () = ();


    /// `readonly attribute boolean isInAutomation;`
    #[inline]
    pub unsafe fn GetIsInAutomation(&self, aIsInAutomation: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsInAutomation)(self, aIsInAutomation)
    }



    /// `void exitIfInAutomation ();`
    #[inline]
    pub unsafe fn ExitIfInAutomation(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ExitIfInAutomation)(self, )
    }



    /// `void crashIfNotInAutomation ();`
    #[inline]
    pub unsafe fn CrashIfNotInAutomation(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CrashIfNotInAutomation)(self, )
    }



    /// `[implicit_jscontext] void setGCZeal (in long zeal);`
    const _SetGCZeal: () = ();


    /// `[implicit_jscontext] void nukeSandbox (in jsval obj);`
    const _NukeSandbox: () = ();


    /// `[implicit_jscontext] void blockScriptForGlobal (in jsval global);`
    const _BlockScriptForGlobal: () = ();


    /// `[implicit_jscontext] void unblockScriptForGlobal (in jsval global);`
    const _UnblockScriptForGlobal: () = ();

    /// ```text
    /// /**
    ///      * Check whether the given object is an opaque wrapper (PermissiveXrayOpaque).
    ///      */
    /// ```
    ///

    /// `bool isOpaqueWrapper (in jsval obj);`
    const _IsOpaqueWrapper: () = ();

    /// ```text
    /// /**
    ///      * Check whether the given object is an XrayWrapper.
    ///      */
    /// ```
    ///

    /// `bool isXrayWrapper (in jsval obj);`
    const _IsXrayWrapper: () = ();

    /// ```text
    /// /**
    ///      * Waive Xray on a given value. Identity op for primitives.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] jsval waiveXrays (in jsval aVal);`
    const _WaiveXrays: () = ();

    /// ```text
    /// /**
    ///      * Strip off Xray waivers on a given value. Identity op for primitives.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] jsval unwaiveXrays (in jsval aVal);`
    const _UnwaiveXrays: () = ();

    /// ```text
    /// /**
    ///      * Gets the name of the JSClass of the object.
    ///      *
    ///      * if |aUnwrap| is true, all wrappers are unwrapped first. Unless you're
    ///      * specifically trying to detect whether the object is a proxy, this is
    ///      * probably what you want.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] string getClassName (in jsval aObj, in bool aUnwrap);`
    const _GetClassName: () = ();

    /// ```text
    /// /**
    ///      * Get a DOM classinfo for the given classname.  Only some class
    ///      * names are supported.
    ///      */
    /// ```
    ///

    /// `nsIClassInfo getDOMClassInfo (in AString aClassName);`
    #[inline]
    pub unsafe fn GetDOMClassInfo(&self, aClassName: *const ::nsstring::nsAString, _retval: *mut*const nsIClassInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetDOMClassInfo)(self, aClassName, _retval)
    }


    /// ```text
    /// /**
    ///      * Gets the incument global for the execution of this function. For internal
    ///      * and testing use only.
    ///      *
    ///      * If |callback| is passed, it is invoked with the incumbent global as its
    ///      * sole argument. This allows the incumbent global to be measured in callback
    ///      * environments with no scripted frames on the stack.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getIncumbentGlobal ([optional] in jsval callback);`
    const _GetIncumbentGlobal: () = ();

    /// ```text
    /// /**
    ///      * Forces the generation of an XPCWrappedJS for a given object. For internal
    ///      * and testing use only. This is only useful to set up wrapper map conditions
    ///      * for a testcase. The return value is not an XPCWrappedJS itself, but an
    ///      * opaque nsISupports holder that keeps the underlying XPCWrappedJS alive.
    ///      *
    ///      * if |scope| is passed, the XPCWrappedJS is generated in the scope of that object.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] nsISupports generateXPCWrappedJS (in jsval obj, [optional] in jsval scope);`
    const _GenerateXPCWrappedJS: () = ();

    /// ```text
    /// /**
    ///       * Retrieve the last time, in microseconds since epoch, that a given
    ///       * watchdog-related event occured.
    ///       *
    ///       * Valid categories:
    ///       *   "ContextStateChange"      - Context switching between active and inactive states
    ///       *   "WatchdogWakeup"          - Watchdog waking up from sleeping
    ///       *   "WatchdogHibernateStart"  - Watchdog begins hibernating
    ///       *   "WatchdogHibernateStop"   - Watchdog stops hibernating
    ///       */
    /// ```
    ///

    /// `PRTime getWatchdogTimestamp (in AString aCategory);`
    #[inline]
    pub unsafe fn GetWatchdogTimestamp(&self, aCategory: *const ::nsstring::nsAString, _retval: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetWatchdogTimestamp)(self, aCategory, _retval)
    }



    /// `[implicit_jscontext] jsval getJSEngineTelemetryValue ();`
    const _GetJSEngineTelemetryValue: () = ();


    /// `[implicit_jscontext] jsval cloneInto (in jsval value, in jsval scope, [optional] in jsval options);`
    const _CloneInto: () = ();


    /// `nsIPrincipal getWebIDLCallerPrincipal ();`
    #[inline]
    pub unsafe fn GetWebIDLCallerPrincipal(&self, _retval: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetWebIDLCallerPrincipal)(self, _retval)
    }



    /// `[implicit_jscontext] nsIPrincipal getObjectPrincipal (in jsval obj);`
    const _GetObjectPrincipal: () = ();


    /// `[implicit_jscontext] ACString getRealmLocation (in jsval obj);`
    const _GetRealmLocation: () = ();


    /// `double now ();`
    #[inline]
    pub unsafe fn Now(&self, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).Now)(self, _retval)
    }



    /// `AUTF8String readUTF8File (in nsIFile file);`
    #[inline]
    pub unsafe fn ReadUTF8File(&self, file: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ReadUTF8File)(self, file, _retval)
    }



    /// `AUTF8String readUTF8URI (in nsIURI url);`
    #[inline]
    pub unsafe fn ReadUTF8URI(&self, url: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ReadUTF8URI)(self, url, _retval)
    }



    /// `nsIEditorSpellCheck createSpellChecker ();`
    #[inline]
    pub unsafe fn CreateSpellChecker(&self, _retval: *mut*const nsIEditorSpellCheck) -> ::nserror::nsresult {
        ((*self.vtable).CreateSpellChecker)(self, _retval)
    }



    /// `nsISupports createCommandLine ([optional] in nsIFile aWorkingDir);`
    #[inline]
    pub unsafe fn CreateCommandLine(&self, aWorkingDir: *const nsIFile, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).CreateCommandLine)(self, aWorkingDir, _retval)
    }



    /// `nsICommandParams createCommandParams ();`
    #[inline]
    pub unsafe fn CreateCommandParams(&self, _retval: *mut*const nsICommandParams) -> ::nserror::nsresult {
        ((*self.vtable).CreateCommandParams)(self, _retval)
    }



    /// `nsILoadContext createLoadContext ();`
    #[inline]
    pub unsafe fn CreateLoadContext(&self, _retval: *mut*const nsILoadContext) -> ::nserror::nsresult {
        ((*self.vtable).CreateLoadContext)(self, _retval)
    }



    /// `nsILoadContext createPrivateLoadContext ();`
    #[inline]
    pub unsafe fn CreatePrivateLoadContext(&self, _retval: *mut*const nsILoadContext) -> ::nserror::nsresult {
        ((*self.vtable).CreatePrivateLoadContext)(self, _retval)
    }



    /// `nsIPersistentProperties createPersistentProperties ();`
    #[inline]
    pub unsafe fn CreatePersistentProperties(&self, _retval: *mut*const nsIPersistentProperties) -> ::nserror::nsresult {
        ((*self.vtable).CreatePersistentProperties)(self, _retval)
    }



    /// `nsIDocumentEncoder createDocumentEncoder (in string contentType);`
    #[inline]
    pub unsafe fn CreateDocumentEncoder(&self, contentType: *const libc::c_char, _retval: *mut*const nsIDocumentEncoder) -> ::nserror::nsresult {
        ((*self.vtable).CreateDocumentEncoder)(self, contentType, _retval)
    }



    /// `nsIDocumentEncoder createHTMLCopyEncoder ();`
    #[inline]
    pub unsafe fn CreateHTMLCopyEncoder(&self, _retval: *mut*const nsIDocumentEncoder) -> ::nserror::nsresult {
        ((*self.vtable).CreateHTMLCopyEncoder)(self, _retval)
    }



    /// `readonly attribute Array<ACString> loadedModules;`
    #[inline]
    pub unsafe fn GetLoadedModules(&self, aLoadedModules: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadedModules)(self, aLoadedModules)
    }



    /// `readonly attribute Array<ACString> loadedComponents;`
    #[inline]
    pub unsafe fn GetLoadedComponents(&self, aLoadedComponents: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadedComponents)(self, aLoadedComponents)
    }



    /// `ACString getModuleImportStack (in AUTF8String aLocation);`
    #[inline]
    pub unsafe fn GetModuleImportStack(&self, aLocation: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetModuleImportStack)(self, aLocation, _retval)
    }



    /// `ACString getComponentLoadStack (in AUTF8String aLocation);`
    #[inline]
    pub unsafe fn GetComponentLoadStack(&self, aLocation: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetComponentLoadStack)(self, aLocation, _retval)
    }


}


/// `interface nsIXPCComponents : nsISupports`
///

/// ```text
/// /**
/// * Interface for the 'Components' object.
/// */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPCComponents {
    vtable: *const nsIXPCComponentsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPCComponents.
unsafe impl XpCom for nsIXPCComponents {
    const IID: nsIID = nsID(0xaa28aaf6, 0x70ce, 0x4b03,
        [0x95, 0x14, 0xaf, 0xe4, 0x3c, 0x7d, 0xfd, 0xa8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPCComponents {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPCComponents.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPCComponentsCoerce {
    /// Cheaply cast a value of this type from a `nsIXPCComponents`.
    fn coerce_from(v: &nsIXPCComponents) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPCComponentsCoerce for nsIXPCComponents {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents) -> &Self {
        v
    }
}

impl nsIXPCComponents {
    /// Cast this `nsIXPCComponents` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPCComponentsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPCComponents {
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
impl<T: nsISupportsCoerce> nsIXPCComponentsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPCComponents
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPCComponentsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIXPCComponents_Interfaces interfaces; */
    pub GetInterfaces: unsafe extern "system" fn (this: *const nsIXPCComponents, aInterfaces: *mut *const nsIXPCComponents_Interfaces) -> ::nserror::nsresult,

    /* readonly attribute nsIXPCComponents_Results results; */
    pub GetResults: unsafe extern "system" fn (this: *const nsIXPCComponents, aResults: *mut *const nsIXPCComponents_Results) -> ::nserror::nsresult,

    /* boolean isSuccessCode (in nsresult result); */
    pub IsSuccessCode: unsafe extern "system" fn (this: *const nsIXPCComponents, result: ::nserror::nsresult, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsIXPCComponents_Classes classes; */
    pub GetClasses: unsafe extern "system" fn (this: *const nsIXPCComponents, aClasses: *mut *const nsIXPCComponents_Classes) -> ::nserror::nsresult,

    /* readonly attribute nsIStackFrame stack; */
    pub GetStack: unsafe extern "system" fn (this: *const nsIXPCComponents, aStack: *mut*const nsIStackFrame) -> ::nserror::nsresult,

    /* readonly attribute nsIComponentManager manager; */
    pub GetManager: unsafe extern "system" fn (this: *const nsIXPCComponents, aManager: *mut*const nsIComponentManager) -> ::nserror::nsresult,

    /* readonly attribute nsIXPCComponents_Utils utils; */
    pub GetUtils: unsafe extern "system" fn (this: *const nsIXPCComponents, aUtils: *mut *const nsIXPCComponents_Utils) -> ::nserror::nsresult,

    /* readonly attribute nsIXPCComponents_ID ID; */
    pub GetID: unsafe extern "system" fn (this: *const nsIXPCComponents, aID: *mut *const nsIXPCComponents_ID) -> ::nserror::nsresult,

    /* readonly attribute nsIXPCComponents_Exception Exception; */
    pub GetException: unsafe extern "system" fn (this: *const nsIXPCComponents, aException: *mut *const nsIXPCComponents_Exception) -> ::nserror::nsresult,

    /* readonly attribute nsIXPCComponents_Constructor Constructor; */
    pub GetConstructor: unsafe extern "system" fn (this: *const nsIXPCComponents, aConstructor: *mut *const nsIXPCComponents_Constructor) -> ::nserror::nsresult,

    /* [implicit_jscontext] attribute jsval returnCode; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetReturnCode: *const ::libc::c_void,

    /* [implicit_jscontext] attribute jsval returnCode; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub SetReturnCode: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPCComponents {


    /// `readonly attribute nsIXPCComponents_Interfaces interfaces;`
    #[inline]
    pub unsafe fn GetInterfaces(&self, aInterfaces: *mut *const nsIXPCComponents_Interfaces) -> ::nserror::nsresult {
        ((*self.vtable).GetInterfaces)(self, aInterfaces)
    }



    /// `readonly attribute nsIXPCComponents_Results results;`
    #[inline]
    pub unsafe fn GetResults(&self, aResults: *mut *const nsIXPCComponents_Results) -> ::nserror::nsresult {
        ((*self.vtable).GetResults)(self, aResults)
    }



    /// `boolean isSuccessCode (in nsresult result);`
    #[inline]
    pub unsafe fn IsSuccessCode(&self, result: ::nserror::nsresult, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsSuccessCode)(self, result, _retval)
    }



    /// `readonly attribute nsIXPCComponents_Classes classes;`
    #[inline]
    pub unsafe fn GetClasses(&self, aClasses: *mut *const nsIXPCComponents_Classes) -> ::nserror::nsresult {
        ((*self.vtable).GetClasses)(self, aClasses)
    }



    /// `readonly attribute nsIStackFrame stack;`
    #[inline]
    pub unsafe fn GetStack(&self, aStack: *mut*const nsIStackFrame) -> ::nserror::nsresult {
        ((*self.vtable).GetStack)(self, aStack)
    }



    /// `readonly attribute nsIComponentManager manager;`
    #[inline]
    pub unsafe fn GetManager(&self, aManager: *mut*const nsIComponentManager) -> ::nserror::nsresult {
        ((*self.vtable).GetManager)(self, aManager)
    }



    /// `readonly attribute nsIXPCComponents_Utils utils;`
    #[inline]
    pub unsafe fn GetUtils(&self, aUtils: *mut *const nsIXPCComponents_Utils) -> ::nserror::nsresult {
        ((*self.vtable).GetUtils)(self, aUtils)
    }



    /// `readonly attribute nsIXPCComponents_ID ID;`
    #[inline]
    pub unsafe fn GetID(&self, aID: *mut *const nsIXPCComponents_ID) -> ::nserror::nsresult {
        ((*self.vtable).GetID)(self, aID)
    }



    /// `readonly attribute nsIXPCComponents_Exception Exception;`
    #[inline]
    pub unsafe fn GetException(&self, aException: *mut *const nsIXPCComponents_Exception) -> ::nserror::nsresult {
        ((*self.vtable).GetException)(self, aException)
    }



    /// `readonly attribute nsIXPCComponents_Constructor Constructor;`
    #[inline]
    pub unsafe fn GetConstructor(&self, aConstructor: *mut *const nsIXPCComponents_Constructor) -> ::nserror::nsresult {
        ((*self.vtable).GetConstructor)(self, aConstructor)
    }



    /// `[implicit_jscontext] attribute jsval returnCode;`
    const _GetReturnCode: () = ();


    /// `[implicit_jscontext] attribute jsval returnCode;`
    const _SetReturnCode: () = ();

}


