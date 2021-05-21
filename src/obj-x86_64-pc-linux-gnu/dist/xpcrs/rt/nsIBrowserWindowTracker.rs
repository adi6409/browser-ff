//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/modules/nsIBrowserWindowTracker.idl
//


/// `interface nsIVisibleTab : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIVisibleTab {
    vtable: *const nsIVisibleTabVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIVisibleTab.
unsafe impl XpCom for nsIVisibleTab {
    const IID: nsIID = nsID(0xf6190951, 0x69d0, 0x4ce5,
        [0xb5, 0x03, 0xd2, 0x53, 0x5d, 0x9d, 0xe9, 0x8c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIVisibleTab {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIVisibleTab.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIVisibleTabCoerce {
    /// Cheaply cast a value of this type from a `nsIVisibleTab`.
    fn coerce_from(v: &nsIVisibleTab) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIVisibleTabCoerce for nsIVisibleTab {
    #[inline]
    fn coerce_from(v: &nsIVisibleTab) -> &Self {
        v
    }
}

impl nsIVisibleTab {
    /// Cast this `nsIVisibleTab` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIVisibleTabCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIVisibleTab {
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
impl<T: nsISupportsCoerce> nsIVisibleTabCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIVisibleTab) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIVisibleTab
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIVisibleTabVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute AString contentTitle; */
    pub GetContentTitle: unsafe extern "system" fn (this: *const nsIVisibleTab, aContentTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString contentTitle; */
    pub SetContentTitle: unsafe extern "system" fn (this: *const nsIVisibleTab, aContentTitle: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute int64_t browserId; */
    pub GetBrowserId: unsafe extern "system" fn (this: *const nsIVisibleTab, aBrowserId: *mut int64_t) -> ::nserror::nsresult,

    /* attribute int64_t browserId; */
    pub SetBrowserId: unsafe extern "system" fn (this: *const nsIVisibleTab, aBrowserId: int64_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIVisibleTab {


    /// `attribute AString contentTitle;`
    #[inline]
    pub unsafe fn GetContentTitle(&self, aContentTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetContentTitle)(self, aContentTitle)
    }



    /// `attribute AString contentTitle;`
    #[inline]
    pub unsafe fn SetContentTitle(&self, aContentTitle: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetContentTitle)(self, aContentTitle)
    }



    /// `attribute int64_t browserId;`
    #[inline]
    pub unsafe fn GetBrowserId(&self, aBrowserId: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetBrowserId)(self, aBrowserId)
    }



    /// `attribute int64_t browserId;`
    #[inline]
    pub unsafe fn SetBrowserId(&self, aBrowserId: int64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetBrowserId)(self, aBrowserId)
    }


}


/// `interface nsIBrowserWindowTracker : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBrowserWindowTracker {
    vtable: *const nsIBrowserWindowTrackerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBrowserWindowTracker.
unsafe impl XpCom for nsIBrowserWindowTracker {
    const IID: nsIID = nsID(0x846ff245, 0xccbf, 0x4c7a,
        [0x80, 0x7e, 0x06, 0x0f, 0x02, 0x92, 0x76, 0x51]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBrowserWindowTracker {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBrowserWindowTracker.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBrowserWindowTrackerCoerce {
    /// Cheaply cast a value of this type from a `nsIBrowserWindowTracker`.
    fn coerce_from(v: &nsIBrowserWindowTracker) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBrowserWindowTrackerCoerce for nsIBrowserWindowTracker {
    #[inline]
    fn coerce_from(v: &nsIBrowserWindowTracker) -> &Self {
        v
    }
}

impl nsIBrowserWindowTracker {
    /// Cast this `nsIBrowserWindowTracker` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBrowserWindowTrackerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBrowserWindowTracker {
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
impl<T: nsISupportsCoerce> nsIBrowserWindowTrackerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserWindowTracker) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBrowserWindowTracker
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBrowserWindowTrackerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Array<nsIVisibleTab> getAllVisibleTabs (); */
    pub GetAllVisibleTabs: unsafe extern "system" fn (this: *const nsIBrowserWindowTracker, _retval: *mut thin_vec::ThinVec<RefPtr<nsIVisibleTab>>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBrowserWindowTracker {

    /// ```text
    /// /**
    ///    * Return array of browser tabs that are currently visible.
    ///    */
    /// ```
    ///

    /// `Array<nsIVisibleTab> getAllVisibleTabs ();`
    #[inline]
    pub unsafe fn GetAllVisibleTabs(&self, _retval: *mut thin_vec::ThinVec<RefPtr<nsIVisibleTab>>) -> ::nserror::nsresult {
        ((*self.vtable).GetAllVisibleTabs)(self, _retval)
    }


}


