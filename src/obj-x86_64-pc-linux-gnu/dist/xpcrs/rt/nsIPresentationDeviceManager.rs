//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationDeviceManager.idl
//


/// `interface nsIPresentationDeviceManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationDeviceManager {
    vtable: *const nsIPresentationDeviceManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationDeviceManager.
unsafe impl XpCom for nsIPresentationDeviceManager {
    const IID: nsIID = nsID(0xbeb61db5, 0x3d5f, 0x454f,
        [0xa1, 0x5a, 0xdb, 0xfa, 0x03, 0x37, 0xc5, 0x69]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationDeviceManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationDeviceManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationDeviceManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationDeviceManager`.
    fn coerce_from(v: &nsIPresentationDeviceManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationDeviceManagerCoerce for nsIPresentationDeviceManager {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceManager) -> &Self {
        v
    }
}

impl nsIPresentationDeviceManager {
    /// Cast this `nsIPresentationDeviceManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationDeviceManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationDeviceManager {
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
impl<T: nsISupportsCoerce> nsIPresentationDeviceManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationDeviceManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationDeviceManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean deviceAvailable; */
    pub GetDeviceAvailable: unsafe extern "system" fn (this: *const nsIPresentationDeviceManager, aDeviceAvailable: *mut bool) -> ::nserror::nsresult,

    /* void addDeviceProvider (in nsIPresentationDeviceProvider provider); */
    pub AddDeviceProvider: unsafe extern "system" fn (this: *const nsIPresentationDeviceManager, provider: *const nsIPresentationDeviceProvider) -> ::nserror::nsresult,

    /* void removeDeviceProvider (in nsIPresentationDeviceProvider provider); */
    pub RemoveDeviceProvider: unsafe extern "system" fn (this: *const nsIPresentationDeviceManager, provider: *const nsIPresentationDeviceProvider) -> ::nserror::nsresult,

    /* void forceDiscovery (); */
    pub ForceDiscovery: unsafe extern "system" fn (this: *const nsIPresentationDeviceManager) -> ::nserror::nsresult,

    /* nsIArray getAvailableDevices ([optional] in nsIArray presentationUrls); */
    pub GetAvailableDevices: unsafe extern "system" fn (this: *const nsIPresentationDeviceManager, presentationUrls: *const nsIArray, _retval: *mut*const nsIArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationDeviceManager {


    /// `readonly attribute boolean deviceAvailable;`
    #[inline]
    pub unsafe fn GetDeviceAvailable(&self, aDeviceAvailable: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDeviceAvailable)(self, aDeviceAvailable)
    }



    /// `void addDeviceProvider (in nsIPresentationDeviceProvider provider);`
    #[inline]
    pub unsafe fn AddDeviceProvider(&self, provider: *const nsIPresentationDeviceProvider) -> ::nserror::nsresult {
        ((*self.vtable).AddDeviceProvider)(self, provider)
    }



    /// `void removeDeviceProvider (in nsIPresentationDeviceProvider provider);`
    #[inline]
    pub unsafe fn RemoveDeviceProvider(&self, provider: *const nsIPresentationDeviceProvider) -> ::nserror::nsresult {
        ((*self.vtable).RemoveDeviceProvider)(self, provider)
    }



    /// `void forceDiscovery ();`
    #[inline]
    pub unsafe fn ForceDiscovery(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ForceDiscovery)(self, )
    }



    /// `nsIArray getAvailableDevices ([optional] in nsIArray presentationUrls);`
    #[inline]
    pub unsafe fn GetAvailableDevices(&self, presentationUrls: *const nsIArray, _retval: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetAvailableDevices)(self, presentationUrls, _retval)
    }


}


