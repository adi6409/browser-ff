//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsISelectionListener.idl
//


/// `interface nsISelectionListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISelectionListener {
    vtable: *const nsISelectionListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISelectionListener.
unsafe impl XpCom for nsISelectionListener {
    const IID: nsIID = nsID(0x45686299, 0xae2b, 0x46bc,
        [0x95, 0x02, 0xc5, 0x6c, 0x35, 0x69, 0x1a, 0xb9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISelectionListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISelectionListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISelectionListenerCoerce {
    /// Cheaply cast a value of this type from a `nsISelectionListener`.
    fn coerce_from(v: &nsISelectionListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISelectionListenerCoerce for nsISelectionListener {
    #[inline]
    fn coerce_from(v: &nsISelectionListener) -> &Self {
        v
    }
}

impl nsISelectionListener {
    /// Cast this `nsISelectionListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISelectionListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISelectionListener {
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
impl<T: nsISupportsCoerce> nsISelectionListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISelectionListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISelectionListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISelectionListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] void notifySelectionChanged (in Document doc, in Selection sel, in short reason); */
    pub NotifySelectionChanged: unsafe extern "system" fn (this: *const nsISelectionListener, doc: *const libc::c_void, sel: *const libc::c_void, reason: i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISelectionListener {

    pub const NO_REASON: i64 = 0;


    pub const DRAG_REASON: i64 = 1;


    pub const MOUSEDOWN_REASON: i64 = 2;


    pub const MOUSEUP_REASON: i64 = 4;


    pub const KEYPRESS_REASON: i64 = 8;


    pub const SELECTALL_REASON: i64 = 16;


    pub const COLLAPSETOSTART_REASON: i64 = 32;


    pub const COLLAPSETOEND_REASON: i64 = 64;


    pub const IME_REASON: i64 = 128;


    pub const JS_REASON: i64 = 256;


    /// `[can_run_script] void notifySelectionChanged (in Document doc, in Selection sel, in short reason);`
    #[inline]
    pub unsafe fn NotifySelectionChanged(&self, doc: *const libc::c_void, sel: *const libc::c_void, reason: i16) -> ::nserror::nsresult {
        ((*self.vtable).NotifySelectionChanged)(self, doc, sel, reason)
    }


}


