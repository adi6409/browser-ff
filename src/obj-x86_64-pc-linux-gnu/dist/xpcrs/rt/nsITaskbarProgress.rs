//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsITaskbarProgress.idl
//


/// `typedef int32_t  nsTaskbarProgressState;`
///


pub type nsTaskbarProgressState = i32;


/// `interface nsITaskbarProgress : nsISupports`
///

/// ```text
/// /**
///  * Starting in Windows 7, applications can display a progress notification in
///  * the taskbar. This class wraps around the native functionality to do this.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITaskbarProgress {
    vtable: *const nsITaskbarProgressVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITaskbarProgress.
unsafe impl XpCom for nsITaskbarProgress {
    const IID: nsIID = nsID(0x23ac257d, 0xef3c, 0x4033,
        [0xb4, 0x24, 0xbe, 0x7f, 0xef, 0x91, 0xa8, 0x6c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITaskbarProgress {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITaskbarProgress.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITaskbarProgressCoerce {
    /// Cheaply cast a value of this type from a `nsITaskbarProgress`.
    fn coerce_from(v: &nsITaskbarProgress) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITaskbarProgressCoerce for nsITaskbarProgress {
    #[inline]
    fn coerce_from(v: &nsITaskbarProgress) -> &Self {
        v
    }
}

impl nsITaskbarProgress {
    /// Cast this `nsITaskbarProgress` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITaskbarProgressCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITaskbarProgress {
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
impl<T: nsISupportsCoerce> nsITaskbarProgressCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITaskbarProgress) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITaskbarProgress
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITaskbarProgressVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setProgressState (in nsTaskbarProgressState state, [optional] in unsigned long long currentValue, [optional] in unsigned long long maxValue); */
    pub SetProgressState: unsafe extern "system" fn (this: *const nsITaskbarProgress, state: nsTaskbarProgressState, currentValue: u64, maxValue: u64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITaskbarProgress {
    /// ```text
    /// /**
    ///    * Stop displaying progress on the taskbar button. This should be used when
    ///    * the operation is complete or cancelled.
    ///    */
    /// ```
    ///

    pub const STATE_NO_PROGRESS: i64 = 0;

    /// ```text
    /// /**
    ///    * Display a cycling, indeterminate progress bar.
    ///    */
    /// ```
    ///

    pub const STATE_INDETERMINATE: i64 = 1;

    /// ```text
    /// /**
    ///    * Display a determinate, normal progress bar.
    ///    */
    /// ```
    ///

    pub const STATE_NORMAL: i64 = 2;

    /// ```text
    /// /**
    ///    * Display a determinate, error progress bar.
    ///    */
    /// ```
    ///

    pub const STATE_ERROR: i64 = 3;

    /// ```text
    /// /**
    ///    * Display a determinate progress bar indicating that the operation has
    ///    * paused.
    ///    */
    /// ```
    ///

    pub const STATE_PAUSED: i64 = 4;

    /// ```text
    /// /**
    ///    * Sets the taskbar progress state and value for this window. The currentValue
    ///    * and maxValue parameters are optional and should be supplied when |state|
    ///    * is one of STATE_NORMAL, STATE_ERROR or STATE_PAUSED.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG if state is STATE_NO_PROGRESS or
    ///    *         STATE_INDETERMINATE, and either currentValue or maxValue is not 0.
    ///    * @throws NS_ERROR_ILLEGAL_VALUE if currentValue is greater than maxValue.
    ///    */
    /// ```
    ///

    /// `void setProgressState (in nsTaskbarProgressState state, [optional] in unsigned long long currentValue, [optional] in unsigned long long maxValue);`
    #[inline]
    pub unsafe fn SetProgressState(&self, state: nsTaskbarProgressState, currentValue: u64, maxValue: u64) -> ::nserror::nsresult {
        ((*self.vtable).SetProgressState)(self, state, currentValue, maxValue)
    }


}


