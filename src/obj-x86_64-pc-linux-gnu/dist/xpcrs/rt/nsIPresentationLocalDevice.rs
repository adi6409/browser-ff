//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationLocalDevice.idl
//


/// `interface nsIPresentationLocalDevice : nsIPresentationDevice`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationLocalDevice {
    vtable: *const nsIPresentationLocalDeviceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationLocalDevice.
unsafe impl XpCom for nsIPresentationLocalDevice {
    const IID: nsIID = nsID(0xdd239720, 0xcab6, 0x4fb5,
        [0x90, 0x25, 0xcb, 0xa2, 0x3f, 0x1b, 0xbc, 0x2d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationLocalDevice {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationLocalDevice.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationLocalDeviceCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationLocalDevice`.
    fn coerce_from(v: &nsIPresentationLocalDevice) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationLocalDeviceCoerce for nsIPresentationLocalDevice {
    #[inline]
    fn coerce_from(v: &nsIPresentationLocalDevice) -> &Self {
        v
    }
}

impl nsIPresentationLocalDevice {
    /// Cast this `nsIPresentationLocalDevice` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationLocalDeviceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationLocalDevice {
    type Target = nsIPresentationDevice;
    #[inline]
    fn deref(&self) -> &nsIPresentationDevice {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPresentationDeviceCoerce> nsIPresentationLocalDeviceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationLocalDevice) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationLocalDevice
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationLocalDeviceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPresentationDeviceVTable,

    /* readonly attribute AUTF8String windowId; */
    pub GetWindowId: unsafe extern "system" fn (this: *const nsIPresentationLocalDevice, aWindowId: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationLocalDevice {


    /// `readonly attribute AUTF8String windowId;`
    #[inline]
    pub unsafe fn GetWindowId(&self, aWindowId: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetWindowId)(self, aWindowId)
    }


}


