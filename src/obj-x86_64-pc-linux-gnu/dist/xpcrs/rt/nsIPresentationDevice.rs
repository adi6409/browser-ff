//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationDevice.idl
//


/// `interface nsIPresentationDevice : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationDevice {
    vtable: *const nsIPresentationDeviceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationDevice.
unsafe impl XpCom for nsIPresentationDevice {
    const IID: nsIID = nsID(0xb1e0a7af, 0x5936, 0x4066,
        [0x8f, 0x2e, 0xf7, 0x89, 0xfb, 0x9a, 0x7e, 0x8f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationDevice {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationDevice.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationDeviceCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationDevice`.
    fn coerce_from(v: &nsIPresentationDevice) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationDeviceCoerce for nsIPresentationDevice {
    #[inline]
    fn coerce_from(v: &nsIPresentationDevice) -> &Self {
        v
    }
}

impl nsIPresentationDevice {
    /// Cast this `nsIPresentationDevice` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationDeviceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationDevice {
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
impl<T: nsISupportsCoerce> nsIPresentationDeviceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDevice) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationDevice
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationDeviceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String id; */
    pub GetId: unsafe extern "system" fn (this: *const nsIPresentationDevice, aId: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIPresentationDevice, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIPresentationDevice, aType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsIPresentationControlChannel establishControlChannel (); */
    pub EstablishControlChannel: unsafe extern "system" fn (this: *const nsIPresentationDevice, _retval: *mut*const nsIPresentationControlChannel) -> ::nserror::nsresult,

    /* void disconnect (); */
    pub Disconnect: unsafe extern "system" fn (this: *const nsIPresentationDevice) -> ::nserror::nsresult,

    /* boolean isRequestedUrlSupported (in AString requestedUrl); */
    pub IsRequestedUrlSupported: unsafe extern "system" fn (this: *const nsIPresentationDevice, requestedUrl: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationDevice {


    /// `readonly attribute AUTF8String id;`
    #[inline]
    pub unsafe fn GetId(&self, aId: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetId)(self, aId)
    }



    /// `readonly attribute AUTF8String name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }



    /// `readonly attribute AUTF8String type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }



    /// `nsIPresentationControlChannel establishControlChannel ();`
    #[inline]
    pub unsafe fn EstablishControlChannel(&self, _retval: *mut*const nsIPresentationControlChannel) -> ::nserror::nsresult {
        ((*self.vtable).EstablishControlChannel)(self, _retval)
    }



    /// `void disconnect ();`
    #[inline]
    pub unsafe fn Disconnect(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Disconnect)(self, )
    }



    /// `boolean isRequestedUrlSupported (in AString requestedUrl);`
    #[inline]
    pub unsafe fn IsRequestedUrlSupported(&self, requestedUrl: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsRequestedUrlSupported)(self, requestedUrl, _retval)
    }


}


