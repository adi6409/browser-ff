//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleStates.idl
//


/// `interface nsIAccessibleStates : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleStates {
    vtable: *const nsIAccessibleStatesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleStates.
unsafe impl XpCom for nsIAccessibleStates {
    const IID: nsIID = nsID(0xf1e0fbb7, 0xfde4, 0x4519,
        [0x93, 0x83, 0x2b, 0xcb, 0xee, 0x42, 0x85, 0x13]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleStates {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleStates.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleStatesCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleStates`.
    fn coerce_from(v: &nsIAccessibleStates) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleStatesCoerce for nsIAccessibleStates {
    #[inline]
    fn coerce_from(v: &nsIAccessibleStates) -> &Self {
        v
    }
}

impl nsIAccessibleStates {
    /// Cast this `nsIAccessibleStates` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleStatesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleStates {
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
impl<T: nsISupportsCoerce> nsIAccessibleStatesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleStates) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleStates
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleStatesVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleStates {
    /// ```text
    /// /**
    ///    * MSAA State flags - used for bitfield. More than 1 allowed.
    ///    */
    /// ```
    ///

    pub const STATE_UNAVAILABLE: i64 = 1;


    pub const STATE_SELECTED: i64 = 2;


    pub const STATE_FOCUSED: i64 = 4;


    pub const STATE_PRESSED: i64 = 8;


    pub const STATE_CHECKED: i64 = 16;


    pub const STATE_MIXED: i64 = 32;


    pub const STATE_READONLY: i64 = 64;


    pub const STATE_HOTTRACKED: i64 = 128;


    pub const STATE_DEFAULT: i64 = 256;


    pub const STATE_EXPANDED: i64 = 512;


    pub const STATE_COLLAPSED: i64 = 1024;


    pub const STATE_BUSY: i64 = 2048;


    pub const STATE_FLOATING: i64 = 4096;


    pub const STATE_MARQUEED: i64 = 8192;


    pub const STATE_ANIMATED: i64 = 16384;


    pub const STATE_INVISIBLE: i64 = 32768;


    pub const STATE_OFFSCREEN: i64 = 65536;


    pub const STATE_SIZEABLE: i64 = 131072;


    pub const STATE_MOVEABLE: i64 = 262144;


    pub const STATE_SELFVOICING: i64 = 524288;


    pub const STATE_FOCUSABLE: i64 = 1048576;


    pub const STATE_SELECTABLE: i64 = 2097152;


    pub const STATE_LINKED: i64 = 4194304;


    pub const STATE_TRAVERSED: i64 = 8388608;


    pub const STATE_MULTISELECTABLE: i64 = 16777216;


    pub const STATE_EXTSELECTABLE: i64 = 33554432;


    pub const STATE_ALERT_LOW: i64 = 67108864;


    pub const STATE_ALERT_MEDIUM: i64 = 134217728;


    pub const STATE_ALERT_HIGH: i64 = 268435456;


    pub const STATE_PROTECTED: i64 = 536870912;


    pub const STATE_HASPOPUP: i64 = 1073741824;


    pub const STATE_REQUIRED: i64 = 67108864;


    pub const STATE_IMPORTANT: i64 = 134217728;


    pub const STATE_INVALID: i64 = 268435456;


    pub const STATE_CHECKABLE: i64 = 8192;

    /// ```text
    /// /**
    ///  * Extended state flags (for now non-MSAA, for Java and Gnome/ATK support)
    ///  * "Extended state flags" has separate value space from "MSAA State flags".
    ///  */
    /// ```
    ///

    pub const EXT_STATE_SUPPORTS_AUTOCOMPLETION: i64 = 1;


    pub const EXT_STATE_DEFUNCT: i64 = 2;


    pub const EXT_STATE_SELECTABLE_TEXT: i64 = 4;


    pub const EXT_STATE_EDITABLE: i64 = 8;


    pub const EXT_STATE_ACTIVE: i64 = 16;


    pub const EXT_STATE_MODAL: i64 = 32;


    pub const EXT_STATE_MULTI_LINE: i64 = 64;


    pub const EXT_STATE_HORIZONTAL: i64 = 128;


    pub const EXT_STATE_OPAQUE: i64 = 256;


    pub const EXT_STATE_SINGLE_LINE: i64 = 512;


    pub const EXT_STATE_TRANSIENT: i64 = 1024;


    pub const EXT_STATE_VERTICAL: i64 = 2048;


    pub const EXT_STATE_STALE: i64 = 4096;


    pub const EXT_STATE_ENABLED: i64 = 8192;


    pub const EXT_STATE_SENSITIVE: i64 = 16384;


    pub const EXT_STATE_EXPANDABLE: i64 = 32768;


    pub const EXT_STATE_PINNED: i64 = 65536;


    pub const EXT_STATE_CURRENT: i64 = 131072;


}


