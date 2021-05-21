//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTypes.idl
//


/// `interface nsIAccessibleScrollType : nsISupports`
///

/// ```text
/// /**
///  * These constants control the scrolling of an object or substring into a
///  * window. Note, keep them synchronized with IA2ScrollType.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleScrollType {
    vtable: *const nsIAccessibleScrollTypeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleScrollType.
unsafe impl XpCom for nsIAccessibleScrollType {
    const IID: nsIID = nsID(0x05cd38b1, 0x94b3, 0x4cdf,
        [0x83, 0x71, 0x39, 0x35, 0xa9, 0x61, 0x14, 0x05]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleScrollType {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleScrollType.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleScrollTypeCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleScrollType`.
    fn coerce_from(v: &nsIAccessibleScrollType) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleScrollTypeCoerce for nsIAccessibleScrollType {
    #[inline]
    fn coerce_from(v: &nsIAccessibleScrollType) -> &Self {
        v
    }
}

impl nsIAccessibleScrollType {
    /// Cast this `nsIAccessibleScrollType` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleScrollTypeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleScrollType {
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
impl<T: nsISupportsCoerce> nsIAccessibleScrollTypeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleScrollType) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleScrollType
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleScrollTypeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleScrollType {
    /// ```text
    /// /**
    ///    * Scroll the top left of the object or substring to the top left of the
    ///    * window (or as close as possible).
    ///    */
    /// ```
    ///

    pub const SCROLL_TYPE_TOP_LEFT: i64 = 0;

    /// ```text
    /// /**
    ///    * Scroll the bottom right of the object or substring to the bottom right of
    ///    * the window (or as close as possible).
    ///    */
    /// ```
    ///

    pub const SCROLL_TYPE_BOTTOM_RIGHT: i64 = 1;

    /// ```text
    /// /**
    ///    * Scroll the top edge of the object or substring to the top edge of the
    ///    * window (or as close as possible).
    ///    */
    /// ```
    ///

    pub const SCROLL_TYPE_TOP_EDGE: i64 = 2;

    /// ```text
    /// /**
    ///    * Scroll the bottom edge of the object or substring to the bottom edge of
    ///    * the window (or as close as possible).
    ///    */
    /// ```
    ///

    pub const SCROLL_TYPE_BOTTOM_EDGE: i64 = 3;

    /// ```text
    /// /**
    ///    * Scroll the left edge of the object or substring to the left edge of the
    ///    * window (or as close as possible).
    ///    */
    /// ```
    ///

    pub const SCROLL_TYPE_LEFT_EDGE: i64 = 4;

    /// ```text
    /// /**
    ///    * Scroll the right edge of the object or substring to the right edge of the
    ///    * window (or as close as possible).
    ///    */
    /// ```
    ///

    pub const SCROLL_TYPE_RIGHT_EDGE: i64 = 5;

    /// ```text
    /// /**
    ///    * Scroll an object the minimum amount necessary in order for the entire
    ///    * frame to be visible (if possible).
    ///    */
    /// ```
    ///

    pub const SCROLL_TYPE_ANYWHERE: i64 = 6;


}


/// `interface nsIAccessibleCoordinateType : nsISupports`
///

/// ```text
/// /**
///  * These constants define which coordinate system a point is located in.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleCoordinateType {
    vtable: *const nsIAccessibleCoordinateTypeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleCoordinateType.
unsafe impl XpCom for nsIAccessibleCoordinateType {
    const IID: nsIID = nsID(0xc9fbdf10, 0x619e, 0x436f,
        [0xbf, 0x4b, 0x85, 0x66, 0x68, 0x6f, 0x15, 0x77]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleCoordinateType {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleCoordinateType.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleCoordinateTypeCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleCoordinateType`.
    fn coerce_from(v: &nsIAccessibleCoordinateType) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleCoordinateTypeCoerce for nsIAccessibleCoordinateType {
    #[inline]
    fn coerce_from(v: &nsIAccessibleCoordinateType) -> &Self {
        v
    }
}

impl nsIAccessibleCoordinateType {
    /// Cast this `nsIAccessibleCoordinateType` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleCoordinateTypeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleCoordinateType {
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
impl<T: nsISupportsCoerce> nsIAccessibleCoordinateTypeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleCoordinateType) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleCoordinateType
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleCoordinateTypeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleCoordinateType {
    /// ```text
    /// /**
    ///    * The coordinates are relative to the screen.
    ///    */
    /// ```
    ///

    pub const COORDTYPE_SCREEN_RELATIVE: i64 = 0;

    /// ```text
    /// /**
    ///    * The coordinates are relative to the window.
    ///    */
    /// ```
    ///

    pub const COORDTYPE_WINDOW_RELATIVE: i64 = 1;

    /// ```text
    /// /**
    ///    * The coordinates are relative to the upper left corner of the bounding box
    ///    * of the immediate parent.
    ///    */
    /// ```
    ///

    pub const COORDTYPE_PARENT_RELATIVE: i64 = 2;


}


