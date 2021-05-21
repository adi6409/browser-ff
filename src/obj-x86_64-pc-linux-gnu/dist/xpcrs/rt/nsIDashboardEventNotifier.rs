//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDashboardEventNotifier.idl
//


/// `interface nsIDashboardEventNotifier : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDashboardEventNotifier {
    vtable: *const nsIDashboardEventNotifierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDashboardEventNotifier.
unsafe impl XpCom for nsIDashboardEventNotifier {
    const IID: nsIID = nsID(0x24fdfcbe, 0x54cb, 0x4997,
        [0x83, 0x92, 0x3c, 0x47, 0x61, 0x26, 0xea, 0x3b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDashboardEventNotifier {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDashboardEventNotifier.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDashboardEventNotifierCoerce {
    /// Cheaply cast a value of this type from a `nsIDashboardEventNotifier`.
    fn coerce_from(v: &nsIDashboardEventNotifier) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDashboardEventNotifierCoerce for nsIDashboardEventNotifier {
    #[inline]
    fn coerce_from(v: &nsIDashboardEventNotifier) -> &Self {
        v
    }
}

impl nsIDashboardEventNotifier {
    /// Cast this `nsIDashboardEventNotifier` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDashboardEventNotifierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDashboardEventNotifier {
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
impl<T: nsISupportsCoerce> nsIDashboardEventNotifierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDashboardEventNotifier) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDashboardEventNotifier
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDashboardEventNotifierVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void addHost (in ACString aHost, in unsigned long aSerial, in boolean aEncrypted); */
    pub AddHost: unsafe extern "system" fn (this: *const nsIDashboardEventNotifier, aHost: *const ::nsstring::nsACString, aSerial: u32, aEncrypted: bool) -> ::nserror::nsresult,

    /* void removeHost (in ACString aHost, in unsigned long aSerial); */
    pub RemoveHost: unsafe extern "system" fn (this: *const nsIDashboardEventNotifier, aHost: *const ::nsstring::nsACString, aSerial: u32) -> ::nserror::nsresult,

    /* void newMsgSent (in ACString aHost, in unsigned long aSerial, in unsigned long aLength); */
    pub NewMsgSent: unsafe extern "system" fn (this: *const nsIDashboardEventNotifier, aHost: *const ::nsstring::nsACString, aSerial: u32, aLength: u32) -> ::nserror::nsresult,

    /* void newMsgReceived (in ACString aHost, in unsigned long aSerial, in unsigned long aLength); */
    pub NewMsgReceived: unsafe extern "system" fn (this: *const nsIDashboardEventNotifier, aHost: *const ::nsstring::nsACString, aSerial: u32, aLength: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDashboardEventNotifier {


    /// `void addHost (in ACString aHost, in unsigned long aSerial, in boolean aEncrypted);`
    #[inline]
    pub unsafe fn AddHost(&self, aHost: *const ::nsstring::nsACString, aSerial: u32, aEncrypted: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddHost)(self, aHost, aSerial, aEncrypted)
    }



    /// `void removeHost (in ACString aHost, in unsigned long aSerial);`
    #[inline]
    pub unsafe fn RemoveHost(&self, aHost: *const ::nsstring::nsACString, aSerial: u32) -> ::nserror::nsresult {
        ((*self.vtable).RemoveHost)(self, aHost, aSerial)
    }



    /// `void newMsgSent (in ACString aHost, in unsigned long aSerial, in unsigned long aLength);`
    #[inline]
    pub unsafe fn NewMsgSent(&self, aHost: *const ::nsstring::nsACString, aSerial: u32, aLength: u32) -> ::nserror::nsresult {
        ((*self.vtable).NewMsgSent)(self, aHost, aSerial, aLength)
    }



    /// `void newMsgReceived (in ACString aHost, in unsigned long aSerial, in unsigned long aLength);`
    #[inline]
    pub unsafe fn NewMsgReceived(&self, aHost: *const ::nsstring::nsACString, aSerial: u32, aLength: u32) -> ::nserror::nsresult {
        ((*self.vtable).NewMsgReceived)(self, aHost, aSerial, aLength)
    }


}


